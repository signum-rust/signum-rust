-- Add up migration script here
CREATE TABLE IF NOT EXISTS peers (
    peer_address TEXT PRIMARY KEY,
    application TEXT,
    version TEXT,
    platform TEXT,
    share_address BOOLEAN,
    network TEXT,
    blacklist_until DATETIME,
    blacklist_count INTEGER
);