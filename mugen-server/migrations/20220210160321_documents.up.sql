-- Add up migration script here
CREATE TABLE documents (
    id serial primary key,
    created timestamptz,
    last_updated timestamptz,
    filetype varchar(255),
    version serial,
    size bigserial,
    data bytea
)
