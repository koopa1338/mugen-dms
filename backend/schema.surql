-- mugen-dms SurrealDB schema
-- Applied idempotently at startup via DEFINE ... (re-running DEFINE is safe in SurrealDB)

-- Schema version tracking table
DEFINE TABLE IF NOT EXISTS _schema_version SCHEMALESS;
DEFINE FIELD IF NOT EXISTS version       ON _schema_version TYPE int    ASSERT $value >= 0;
DEFINE FIELD IF NOT EXISTS applied_at    ON _schema_version TYPE datetime DEFAULT time::now();

-- USER table
DEFINE TABLE IF NOT EXISTS user SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS username      ON user TYPE string  ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS email         ON user TYPE string  ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS password_hash ON user TYPE string  ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS created_at    ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS bucket_path   ON user TYPE string;
DEFINE INDEX IF NOT EXISTS user_email_unique    ON user FIELDS email    UNIQUE;
DEFINE INDEX IF NOT EXISTS user_username_unique ON user FIELDS username UNIQUE;

-- COLLECTION table
DEFINE TABLE IF NOT EXISTS collection SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name       ON collection TYPE string            ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS created_at ON collection TYPE datetime          DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON collection TYPE datetime          DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS owner      ON collection TYPE record<user>      ASSERT $value != NONE;

-- DOCUMENT table
DEFINE TABLE IF NOT EXISTS document SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS filename   ON document TYPE string              ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS filepath   ON document TYPE string              ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS filetype   ON document TYPE option<string>;
DEFINE FIELD IF NOT EXISTS size       ON document TYPE int                 DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON document TYPE datetime            DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON document TYPE datetime            DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS collection ON document TYPE option<record<collection>>;
DEFINE FIELD IF NOT EXISTS owner      ON document TYPE record<user>        ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS ocr_text   ON document TYPE option<string>;
DEFINE FIELD IF NOT EXISTS tags       ON document TYPE array<string>       DEFAULT [];
DEFINE INDEX IF NOT EXISTS collection_owner_idx ON collection FIELDS owner;
DEFINE INDEX IF NOT EXISTS document_owner_idx   ON document FIELDS owner;
DEFINE INDEX IF NOT EXISTS document_collection_idx  ON document FIELDS collection;
DEFINE INDEX IF NOT EXISTS document_updated_idx     ON document FIELDS updated_at;
