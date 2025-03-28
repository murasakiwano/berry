CREATE TABLE IF NOT EXISTS postings (
  id uuid PRIMARY KEY, -- uuid
  title text NOT NULL,
  amount numeric NOT NULL,
  source_account_id uuid NOT NULL REFERENCES accounts(id),
  destination_account_id uuid NOT NULL REFERENCES accounts(id),
  category text,
  posting_date TIMESTAMPTZ NOT NULL -- timestamp of the transaction
);
