use std::{str::FromStr, time::Duration};

use anyhow::{Context, Result};
use sqlx::SqlitePool;
use tracing::Instrument;
use uuid::Uuid;

use crate::{
    configuration::Settings,
    models::p2p::PeerAddress,
    peers::{get_peers, update_db_peer_info},
};

pub async fn run_peer_finder_forever(
    read_pool: SqlitePool,
    write_pool: SqlitePool,
    settings: Settings,
) -> Result<()> {
    loop {
        // Open the job-level span here so we also include the job_id in the error message if this result comes back Error.
        let span = tracing::span!(
            tracing::Level::INFO,
            "Peer Finder Task",
            job_id = Uuid::new_v4().to_string()
        );
        let result = peer_finder(read_pool.clone(), write_pool.clone(), settings.clone())
            .instrument(span)
            .await;
        if result.is_err() {
            tracing::error!("Error in peer finder: {:?}", result);
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

/// This worker finds new peers by querying the existing peers in the database.
/// If no peers exist in the database, it will read from the configuration bootstrap
/// peers list.
#[tracing::instrument(name = "Peer Finder", skip_all)]
pub async fn peer_finder(
    read_pool: SqlitePool,
    write_pool: SqlitePool,
    settings: Settings,
) -> Result<()> {
    tracing::info!("Seeking new peers");
    // Try to get random peer from database
    let row = sqlx::query!(
        r#"
            SELECT peer_announced_address
            FROM peers
            WHERE blacklist_until IS NULL or blacklist_until < DATETIME('now')
            ORDER BY RANDOM()
            LIMIT 1;
        "#
    )
    .fetch_optional(&read_pool)
    .await?;

    // Check if we were able to get a row
    let x = if let Some(r) = row {
        PeerAddress::from_str(r.peer_announced_address.as_str())
    } else {
        let err = anyhow::anyhow!("No valid peers available in the database.");
        tracing::debug!("Couldn't get peer from database: {}", err);
        Err(err)
    };

    // Check if we got a row AND were able to parse it
    let peer = if let Ok(peer_address) = x {
        // Use address from database
        peer_address
    } else {
        // Try address from bootstrap
        let peer = settings
            .p2p
            .bootstrap_peers
            .first()
            .ok_or_else(|| anyhow::anyhow!("Unable to get peer"))?;
        tracing::debug!("Trying the bootstrap list.");
        peer.to_owned()
    };

    tracing::debug!("Randomly chosen peer is {}", peer);
    // Next, send a request to that peer asking for its peers list.
    let peers = get_peers(peer)
        .await
        .context("unable to get peers from database")?;

    let mut transaction = write_pool
        .begin()
        .await
        .context("unable to get transaction from pool")?;

    // Insert the peers into the database, silently ignoring if they fail
    // due to the unique requirement for primary key
    let mut new_peers_count = 0;
    for peer in peers {
        tracing::trace!("Trying to save peer {}", peer);
        let result = sqlx::query!(
            r#" INSERT OR IGNORE
            INTO peers (peer_announced_address)
            VALUES ($1)
        "#,
            peer
        )
        .execute(&mut *transaction)
        .await;

        match result {
            Ok(r) => {
                let number = r.rows_affected();
                new_peers_count += number;
                if number > 0 {
                    tracing::debug!("Saving new peer {}", peer);
                    tracing::debug!("Attempting to update peer info database for '{}'", &peer);
                    tokio::spawn(update_db_peer_info(write_pool.clone(), peer).in_current_span());
                } else {
                    tracing::debug!("Already have peer {}", peer)
                }
            }
            Err(e) => {
                tracing::error!("Unable to save peer: {:?}", e);
                continue;
            }
        }
    }
    transaction
        .commit()
        .await
        .context("unable to commit transaction -- peers not saved")?;
    tracing::info!("Added {} new peers.", new_peers_count);
    Ok(())
}
