# Mugen — SurrealDB Migration Milestone

## Scope

Completion of the SurrealDB migration on the `feat/surrealdb_migration` branch, including dependency upgrades, infrastructure changes, and architectural decisions for future growth.

Phases 4 (feature completion) and 5 (architecture enhancements including S3 storage) are handled in separate feature branches.

---

## Decisions Made

| Decision | Choice | Rationale |
|---|---|---|
| SurrealDB version | **3.x** (3.2.0) | Go all-in, absorb breaking changes now |
| Axum version | **0.8.x** | Latest stable, moderate risk |
| Leptos version | **0.8.20** | Stable release available on crates.io |
| argon2 version | **0.5.x** (keep current) | 0.6 is RC — too risky for password hashing |
| jsonwebtoken | **10.x** (10.4.0) | Latest stable |
| Cron scheduler | **tokio-cron-scheduler** 0.15.1 | `tokio_schedule` is unmaintained |
| Cloud storage | S3-compatible (AWS + Garage) | Deferred to separate feature PR |
| Container runtime | **Podman** | Team preference |
| Rust edition | **2024** | Available with rustc 1.96, low risk |

---

## A. Dependency Upgrades

### A.1 SurrealDB 2.x → 3.x (High Risk)

**Current:** `surrealdb = "2"` → resolves to 2.6.5
**Target:** `surrealdb = "3"` → resolves to 3.2.0

**Breaking changes to investigate and migrate:**
- Query API (`query!()` macro, `Surreal::new`, `Connection` trait)
- Protocol/transport layer (`connect()` signature, WebSocket handling)
- SQL schema format (`.surql` syntax may have evolved in 3.x)
- `kv-rocksdb` feature flag behavior in 3.x
- `surrealdb::sql::Thing` / ID model changes
- `Any` type usage (`Surreal<Any>`) — may have alternatives in 3.x

**Migration steps:**
1. Read SurrealDB 3.x release notes and migration guide
2. Upgrade version pin, run `cargo build` — identify compile errors
3. Fix API breakages systematically
4. Test all three connection engines: `ws://`, `rocksdb://`, `mem://`
5. Verify `schema.surql` compatibility with 3.x parser

### A.2 Axum 0.7 → 0.8 (Moderate Risk)

**Current:** `axum = "0.7"`, `axum-extra = "0.9"`
**Target:** `axum = "0.8"`, compatible `axum-extra`

**Known changes in 0.8:**
- Extractor trait redesign (async trait cleanup)
- `TypedHeader` module reorganization
- Router API adjustments
- Middleware pipeline changes

**Impact on Mugen:**
- `AuthUser` extractor (`auth/middleware.rs`) uses custom `async_trait` — verify compatibility with axum-core 0.4 expansion in 0.8
- `api_routes()` middleware chain (CORS, timeout, error handling, trace) — review for breaking changes
- `AppState<S, A>` generic state pattern — verify still works with new extractor system

### A.3 Leptos 0.6 → 0.8.20 (Moderate Risk)

**Current:** `leptos = "0.6"`, `leptos_meta = "0.6"`, `leptos_router = "0.6"`
**Target:** `leptos = "0.8.20"`, matching `leptos_meta = "0.8"`, `leptos_router = "0.8"`

**Known changes between 0.6 and 0.8:**
- Component model updates (review migration guide)
- CSR mode patterns may differ slightly
- `gloo-file`, `gloo-net`, `gloo-utils` versions may need alignment
- `leptos-use = "0.9"` compatibility check needed

### A.4 Other Dependencies

| Dependency | Current → Target | Action |
|---|---|---|
| `jsonwebtoken` | 9.3.1 → 10.4.0 | Upgrade, verify `encode`/`decode` API |
| `argon2` | 0.5.x → 0.5.x | **No change** — keep current |
| `tokio_schedule` | 0.3.2 (unmaintained) → `tokio-cron-scheduler` 0.15.1 | Swap — cron handler is a placeholder anyway |
| `dotenv` | 0.15 → latest | Run `cargo update` |
| `thiserror` | 1.x → latest | Run `cargo update` |
| `chrono` | 0.4.x → latest | Run `cargo update` |
| `uuid` | 1.x → latest | Run `cargo update` |
| `tower` / `tower-http` | 0.4/0.5 → latest | Run `cargo update` |
| `tracing*` | 0.1.x → latest | Run `cargo update` |

### A.5 Rust Edition 2021 → 2024

All three crates use `edition = "2021"`:
- Workspace root (`Cargo.toml`) sets `edition.workspace = true`
- `backend/Cargo.toml` inherits from workspace
- `common/Cargo.toml` inherits from workspace
- `frontend/Cargo.toml` has explicit `edition = "2021"`

rustc 1.96 supports edition 2024. Migration is primarily automated via `cargo fix --edition`. Low risk.

---

## B. Infrastructure: Podman Migration

### B.1 Container Configuration

**Current:** `backend/docker/docker-compose.yml` — Docker Compose v3 format
**Target:** Podman-compatible configuration

**Changes needed:**
- Rename file to `backend/docker/podman-compose.yml` (or keep name — Podman reads both formats)
- Convert volume syntax for Podman compatibility
- Update `.env.sample` if container references change

### B.2 Cleanup

Remove commented-out Meilisearch service block (future feature, not part of this migration).

---

## C. Schema & Data Layer Architecture

### C.1 Schema Evolution Strategy

**Current:** Idempotent schema applied every startup via `include_str!("../../schema.surql")`. Safe for fresh installs but provides no migration history or rollback capability.

**Approach:** Lightweight version table alongside existing idempotent schema.

```sql
DEFINE TABLE IF NOT EXISTS _schema_version SCHEMALESS;
DEFINE FIELD IF NOT EXISTS version       ON _schema_version TYPE int    ASSERT $value >= 0;
DEFINE FIELD IF NOT EXISTS applied_at    ON _schema_version TYPE datetime DEFAULT time::now();
```

Future destructive migrations (renaming fields, changing types on existing data) scripted as standalone `.surql` files applied manually or via CLI subcommand. No external migration tooling added at this stage.

### C.2 Foreign Key Semantics

**Current:** SurrealDB record types (`record<user>`, `record<collection>`) provide type-level references but no enforcement at database level. Deleting a user leaves orphaned records.

**Approach:** Application-level cascade in delete handlers. Each delete checks and removes child records before the parent. Soft-delete fields (`deleted_at`) deferred to Phase 4 when audit requirements are clearer.

### C.3 Index Additions

**Missing index identified:** Collections lookup by owner has no index, but is called on every authenticated request (list collections route).

Add immediately:
```sql
DEFINE INDEX IF NOT EXISTS collection_owner_idx ON collection FIELDS owner;
```

Consider later (monitor performance):
```sql
DEFINE INDEX IF NOT EXISTS document_owner_created_idx ON document FIELDS owner, created_at;
```

### C.4 Known Bug Fix

Fix typo in existing schema (`schema.surql:34`):
```sql
-- Before (creates a field named after an index):
DEFINE FIELD IF NOT EXISTS document_updated_idx ON document FIELDS updated_at;
-- After (creates an actual index):
DEFINE INDEX IF NOT EXISTS document_updated_idx ON document FIELDS updated_at;
```

---

## D. Authentication Architecture

### D.1 Assessment

The `AuthProvider` trait is well-designed and future-proof:

```rust
trait AuthProvider {
    type Credentials;
    type RegistrationData;
    async fn authenticate(&self, credentials: Self::Credentials, jwt_secret: &str) -> Result<String, ApiError>;
    async fn register(&self, data: Self::RegistrationData) -> Result<User, ApiError>;
}
```

Generic over `A` in `AppState<S, A>` — OAuth2/OIDC will be a drop-in replacement when implemented in Phase 4.

### D.2 JWT Claim Coupling Observation

JWT tokens currently contain `bucket_path` (claims in `auth/mod.rs`). This couples auth tokens to the storage abstraction — if storage backend changes (filesystem → S3), existing JWTs reference stale bucket paths that don't exist anymore.

**Recommendation:** Remove `bucket_path` from JWT claims. Resolve it at runtime by looking up the user record from SurrealDB when needed. This decouples auth from storage configuration and makes provider swaps transparent to active sessions.

---

## E. Storage Abstraction Assessment

### E.1 Current Design (No Changes in This Milestone)

The existing `StorageBackend` trait is well-designed and ready for future S3 implementation:

```rust
trait StorageBackend {
    async fn store(&self, bucket: &str, key: &str, data: &[u8]) -> ApiResult<()>;
    async fn retrieve(&self, bucket: &str, key: &str) -> ApiResult<Vec<u8>>;
    async fn delete(&self, bucket: &str, key: &str) -> ApiResult<()>;
    async fn exists(&self, bucket: &str, key: &str) -> ApiResult<bool>;
}
```

S3-compatible backend (AWS S3 + Garage self-hosted) will be implemented in a separate feature PR as planned in Phase 5.

---

## F. Stale Config Fixes (Blocking)

These must be fixed regardless of other decisions:

1. **`.env` files contain PostgreSQL URLs** — Replace with correct SurrealDB connection strings from `.env.sample`
2. **Frontend About page says "Axum with Sea ORM"** — No ORM is used; raw SurrealQL throughout
3. **Empty route stubs** — `routes/collection.rs` and `routes/settings.rs` are empty (defer to Phase 4)

---

## G. Execution Order

Recommended sequence to minimize risk and merge conflicts:

1. Fix stale configs (blocking bugs — .env, schema typo, frontend text)
2. Upgrade SurrealDB to 3.x + fix API breakages
3. Upgrade Axum to 0.8 + fix API breakages
4. Upgrade Leptos to 0.8.20 + fix frontend breakages
5. Upgrade remaining dependencies (jsonwebtoken, tokio_schedule→tokio-cron-scheduler, etc.)
6. Migrate Rust edition to 2024
7. Podman compose file update
8. Add missing indexes (`collection_owner_idx`, fix `document_updated_idx`)
9. Decouple `bucket_path` from JWT claims
10. Add lightweight schema version table

---

## H. Out of Scope (Separate Feature PRs)

- S3-compatible storage backend (AWS S3 / Garage) — Phase 5
- OCR processing pipeline — Phase 4
- OAuth2/OIDC auth provider — Phase 4
- Meilisearch full-text search — Phase 5
- Scanner cron implementation — Phase 4
- Empty route stubs (`collection.rs`, `settings.rs`) — Phase 4
- Cursor-based pagination — Phase 4
- Document soft-delete / audit trail — Phase 4
- Rate limiting — Phase 5
- Health check endpoint — Phase 5
