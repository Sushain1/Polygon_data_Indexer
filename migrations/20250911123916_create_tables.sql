-- Add migration script here.
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY,
    tx_hash TEXT NOT NULL UNIQUE,
    block_number INTEGER NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    value REAL NOT NULL,
    timestamp TEXT NOT NULL
);

CREATE TABLE net_flows (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    cumulative_net_flow REAL NOT NULL
);-- Add migration script here
