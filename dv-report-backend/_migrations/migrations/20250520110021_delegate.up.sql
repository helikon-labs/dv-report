CREATE TABLE IF NOT EXISTS delegate
(
    id         VARCHAR(64) PRIMARY KEY     NOT NULL,
    name       TEXT                        NOT NULL,
    short_name TEXT                        NOT NULL,
    url        TEXT,
    twitter    TEXT,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

INSERT INTO delegate(id, name, short_name, url, twitter)
VALUES ('pdao', 'Permanence DAO', 'Permanence', 'https://permanence.io', 'PermanenceDAO');

INSERT INTO delegate(id, name, short_name, url, twitter)
VALUES ('thekus', 'The Kus DAO', 'The Kus', NULL, 'KusDAO');

INSERT INTO delegate(id, name, short_name, url, twitter)
VALUES ('polkaworld', 'PolkaWorld', 'PolkaWorld', NULL, 'polkaworld_org');

INSERT INTO delegate(id, name, short_name, url, twitter)
VALUES ('tcore', 'Trustless Core', 'Trustless', NULL, 'trustlesscore');

INSERT INTO delegate(id, name, short_name, url, twitter)
VALUES ('jid', 'JAM Implementers DAO', 'JID', NULL, NULL);

INSERT INTO delegate(id, name, short_name, url, twitter)
VALUES ('hungary', 'Polkadot Hungary DAO', 'Hungary', 'https://polkadothungary.net', 'PolkadotHungary');