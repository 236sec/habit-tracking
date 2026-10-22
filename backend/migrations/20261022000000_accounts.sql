CREATE TABLE accounts (
    id uuid PRIMARY KEY,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE account_identities (
    issuer text NOT NULL,
    subject text NOT NULL,
    account_id uuid NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    PRIMARY KEY (issuer, subject)
);

CREATE INDEX account_identities_account_id_idx ON account_identities(account_id);
