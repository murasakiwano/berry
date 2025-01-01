CREATE TABLE IF NOT EXISTS postings (
  id TEXT NOT NULL PRIMARY KEY, -- uuid
  title TEXT NOT NULL,
  amount_cents BIGINT NOT NULL,
  source_account_id TEXT NOT NULL,
  destination_account_id TEXT NOT NULL,
  category TEXT,
  posting_date BIGINT NOT NULL, -- timestamp of the transaction
  FOREIGN KEY (source_account_id) REFERENCES accounts(id) ON DELETE RESTRICT,
  FOREIGN KEY (destination_account_id) REFERENCES accounts(id) ON DELETE RESTRICT
);
