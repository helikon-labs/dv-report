CREATE TABLE IF NOT EXISTS vote
(
    id                SERIAL PRIMARY KEY,
    network_id        INTEGER                     NOT NULL,
    referendum_index  INTEGER                     NOT NULL,
    track             INTEGER                     NOT NULL,
    block_hash        VARCHAR(64)                 NOT NULL,
    extrinsic_index   INTEGER                     NOT NULL,
    extrinsic_hash    VARCHAR(64)                 NOT NULL,
    is_batch          BOOLEAN                     NOT NULL DEFAULT FALSE,
    is_multisig       BOOLEAN                     NOT NULL DEFAULT FALSE,
    is_proxy          BOOLEAN                     NOT NULL DEFAULT FALSE,
    is_successful     BOOLEAN                     NOT NULL DEFAULT FALSE,
    signer_account_id VARCHAR(64)                 NOT NULL,
    voter_account_id  VARCHAR(64)                 NOT NULL,
    created_at        TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at        TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT vote_fk_referendum
        FOREIGN KEY (network_id, referendum_index)
            REFERENCES referendum (network_id, index)
            ON DELETE RESTRICT
            ON UPDATE CASCADE,
    CONSTRAINT remove_vote_fk_block
        FOREIGN KEY (network_id, block_hash)
            REFERENCES block (network_id, hash)
            ON DELETE RESTRICT
            ON UPDATE CASCADE
);