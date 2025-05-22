CREATE TABLE IF NOT EXISTS referendum
(
    id                      SERIAL PRIMARY KEY,
    network_id              INTEGER                     NOT NULL,
    index                   INTEGER                     NOT NULL,
    track                   INTEGER                     NOT NULL,
    submission_block_number BIGINT                      NOT NULL,
    referendum_status_id    INTEGER                     NOT NULL,
    created_at              TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at              TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT referendum_u_cohort_network_index UNIQUE (network_id, index),
    CONSTRAINT referendum_fk_referendum_status
        FOREIGN KEY (referendum_status_id)
            REFERENCES referendum_status (id)
            ON DELETE RESTRICT
            ON UPDATE CASCADE
);