CREATE TABLE IF NOT EXISTS accounts (
  id uuid PRIMARY KEY, -- uuid
  name text UNIQUE NOT NULL,
  balance_cents bigint NOT NULL DEFAULT 0
);
