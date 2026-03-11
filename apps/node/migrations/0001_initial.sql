-- OSP Node — initial schema

CREATE TABLE IF NOT EXISTS shop (
    id          INTEGER PRIMARY KEY CHECK (id = 1), -- singleton
    name        TEXT    NOT NULL DEFAULT 'My Shop',
    description TEXT,
    owner       TEXT,
    email       TEXT,
    lang        TEXT    DEFAULT 'en',
    currency    TEXT    DEFAULT 'USD',
    logo_url    TEXT,
    categories  TEXT    DEFAULT '[]',  -- JSON array
    tags        TEXT    DEFAULT '[]',  -- JSON array
    links       TEXT    DEFAULT '{}'   -- JSON object
);

-- Seed the singleton shop row
INSERT OR IGNORE INTO shop (id, name) VALUES (1, 'My Shop');

CREATE TABLE IF NOT EXISTS products (
    id              TEXT    PRIMARY KEY,
    name            TEXT    NOT NULL,
    description     TEXT,
    long_description TEXT,
    url             TEXT    NOT NULL,
    product_type    TEXT    NOT NULL DEFAULT 'download',
    price_model     TEXT    NOT NULL DEFAULT 'free',
    price_amount    INTEGER,          -- minor units (cents)
    price_currency  TEXT,
    license_spdx    TEXT,
    license_name    TEXT,
    license_url     TEXT,
    cover_url       TEXT,
    assets          TEXT    DEFAULT '[]',  -- JSON array
    categories      TEXT    DEFAULT '[]',  -- JSON array
    tags            TEXT    DEFAULT '[]',  -- JSON array
    version         TEXT,
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
