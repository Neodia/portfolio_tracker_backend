CREATE TABLE IF NOT EXISTS users
(
    id            UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    email         VARCHAR     NOT NULL UNIQUE,
    password_hash VARCHAR     NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS assets
(
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol           VARCHAR NOT NULL,
    name             VARCHAR NOT NULL,
    network          VARCHAR NOT NULL,
    contract_address VARCHAR NOT NULL,
    UNIQUE (network, contract_address)
);

CREATE TABLE IF NOT EXISTS expected_portfolio_allocations
(
    user_id    UUID        NOT NULL REFERENCES users (id),
    asset_id   UUID        NOT NULL REFERENCES assets (id),
    percentage NUMERIC     NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (user_id, asset_id)
);

CREATE TABLE IF NOT EXISTS current_holdings
(
    id          UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    user_id     UUID        NOT NULL REFERENCES users (id),
    asset_id    UUID        NOT NULL REFERENCES assets (id),
    amount      NUMERIC     NOT NULL,
    description VARCHAR,
    updated_at  TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS rates
(
    asset_id UUID        NOT NULL REFERENCES assets (id),
    rate_usd NUMERIC     NOT NULL,
    rate_at  TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (asset_id, rate_at)
);

CREATE TABLE IF NOT EXISTS portfolio_snapshots
(
    user_id    UUID        NOT NULL REFERENCES users (id),
    value_usd  NUMERIC     NOT NULL,
    at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (user_id, at)
);

CREATE TABLE IF NOT EXISTS outbox
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    event_type VARCHAR     NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    handled_at TIMESTAMPTZ
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_current_holdings_user ON current_holdings (user_id);
CREATE INDEX IF NOT EXISTS idx_current_holdings_asset ON current_holdings (asset_id);
CREATE INDEX IF NOT EXISTS idx_snapshots_user ON portfolio_snapshots (user_id);
CREATE INDEX IF NOT EXISTS idx_snapshots_created ON portfolio_snapshots (at);
CREATE INDEX IF NOT EXISTS idx_outbox_handled ON outbox (handled_at) WHERE handled_at IS NULL;