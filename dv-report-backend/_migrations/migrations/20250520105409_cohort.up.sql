CREATE TABLE IF NOT EXISTS cohort
(
    number             INTEGER                     NOT NULL,
    network_id         INTEGER                     NOT NULL,
    announcement_date  TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    announcement_url   TEXT,
    delegation_date    TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    start_block_number BIGINT                      NOT NULL,
    created_at         TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at         TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT cohort_pk PRIMARY KEY (number, network_id),
    CONSTRAINT cohort_fk_network
        FOREIGN KEY (network_id)
            REFERENCES network (id)
            ON DELETE RESTRICT
            ON UPDATE CASCADE
);

INSERT INTO cohort(number, network_id, announcement_date, announcement_url, delegation_date, start_block_number)
VALUES (4, 1, '2025-03-27',
        'https://medium.com/web3foundation/decentralized-voices-cohort-4-delegates-announced-a5a9c64927fd',
        '2025-04-14',
        25571091);

INSERT INTO cohort(number, network_id, announcement_date, announcement_url, delegation_date, start_block_number)
VALUES (4, 2, '2025-03-27',
        'https://medium.com/web3foundation/decentralized-voices-cohort-4-delegates-announced-a5a9c64927fd',
        '2025-04-14',
        27921529);