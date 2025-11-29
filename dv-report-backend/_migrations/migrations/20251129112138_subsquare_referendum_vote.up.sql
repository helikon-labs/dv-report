CREATE TABLE IF NOT EXISTS subsquare_referendum_vote
(
    id                  SERIAL PRIMARY KEY NOT NULL,
    network_id          INTEGER            NOT NULL,
    referendum_index    INTEGER            NOT NULL,
    account_id          VARCHAR(64)        NOT NULL,
    delegate_account_id VARCHAR(64),
    is_standard         BOOLEAN            NOT NULL,
    is_split            BOOLEAN            NOT NULL,
    is_split_abstain    BOOLEAN            NOT NULL,
    balance             VARCHAR,
    aye                 BOOLEAN,
    conviction          INTEGER            NOT NULL,
    abstain_balance     VARCHAR,
    abstain_votes       VARCHAR,
    aye_balance         VARCHAR,
    aye_votes           VARCHAR,
    nay_balance         VARCHAR,
    nay_votes           VARCHAR,
    votes               VARCHAR,
    delegated_votes     VARCHAR,
    delegated_capital   VARCHAR,
    query_at            BIGINT             NOT NULL,
    CONSTRAINT ss_referendum_vote_u_vote UNIQUE (network_id, referendum_index, account_id),
    CONSTRAINT ss_referendum_vote_fk_referendum
        FOREIGN KEY (network_id, referendum_index)
            REFERENCES referendum (network_id, index)
            ON DELETE RESTRICT
            ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS ss_referendum_vote_idx_network_referendum
    ON subsquare_referendum_vote (network_id, referendum_index);
CREATE INDEX IF NOT EXISTS ss_referendum_vote_idx_network_referendum_account
    ON subsquare_referendum_vote (network_id, referendum_index, account_id);
CREATE INDEX IF NOT EXISTS ss_referendum_vote_idx_network_referendum_delegate
    ON subsquare_referendum_vote (network_id, referendum_index, delegate_account_id);
CREATE INDEX IF NOT EXISTS ss_referendum_vote_idx_account
    ON subsquare_referendum_vote (account_id);
CREATE INDEX IF NOT EXISTS ss_referendum_vote_idx_delegate
    ON subsquare_referendum_vote (delegate_account_id);