CREATE TABLE IF NOT EXISTS accounts (
  id uuid PRIMARY KEY, -- uuid
  name text UNIQUE NOT NULL,
  balance numeric NOT NULL DEFAULT 0
);
