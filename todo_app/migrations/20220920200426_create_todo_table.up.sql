CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE todos
(
    id    uuid DEFAULT uuid_generate_v4(),
    title VARCHAR NOT NULL,
    done  BOOLEAN NOT NULL,
    PRIMARY KEY (id)
);