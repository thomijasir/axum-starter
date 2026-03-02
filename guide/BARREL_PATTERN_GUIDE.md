# Barrel Pattern Guide

This project uses the **barrel pattern** throughout its Rust module system. Understanding this pattern is essential for navigating and extending the codebase correctly.

---

## What Is the Barrel Pattern?

A **barrel** is a `mod.rs` file that:

1. Declares submodules with `pub mod`
2. Re-exports selected items from those submodules with `pub use`

The goal is to create a single, clean public surface for a module group. Consumers import from the barrel — not from the internal files directly. This keeps `use` statements short and decouples callers from your internal file layout.

The name comes from JavaScript/TypeScript where an `index.ts` that re-exports is called a "barrel file". The same concept applies here.

---

## How It Works in This Project

Every shared directory has a `mod.rs` that acts as its barrel.

### `src/services/mod.rs`

```rust
pub mod http_error;
pub mod http_response;
pub mod sqlite;

pub use http_error::HttpError;
pub use http_response::HttpResponse;
pub use sqlite::DBSqlite;
```

Instead of importing from the internal file:

```rust
// DON'T — reaches into internals
use crate::services::http_error::HttpError;
use crate::services::http_response::HttpResponse;
use crate::services::sqlite::DBSqlite;
```

Consumers import from the barrel:

```rust
// DO — imports from the barrel surface
use crate::services::{HttpError, HttpResponse, DBSqlite};
```

---

### `src/extractors/mod.rs`

```rust
pub mod auth;
pub mod body;
pub mod formdata;
pub mod path;

pub use auth::AuthUser;
pub use body::BodyJson;
pub use formdata::{FileValidationConfig, MultipartFile, MultipartForm, MultipartFormWithConfig};
pub use path::PathParam;
```

Usage in a controller:

```rust
// DO
use crate::extractors::{AuthUser, BodyJson};

// DON'T
use crate::extractors::auth::AuthUser;
use crate::extractors::body::BodyJson;
```

---

### `src/utils/mod.rs`

The utils barrel also **renames** items on re-export to give them clearer call-site names:

```rust
pub mod encrypt;
pub mod generator;
pub mod token;
// ...

pub use encrypt::{hash as hash_password, verify as verify_password};
pub use generator::id as generate_id;
pub use token::{create_token, decode_token};
```

Usage:

```rust
// DO — uses the barrel's clean alias
use crate::utils::{hash_password, generate_id, create_token};

// DON'T — exposes internal module names and raw function names
use crate::utils::encrypt::hash;
use crate::utils::generator::id;
```

---

### `src/models/mod.rs`

Some barrels use glob re-exports (`pub use module::*`) when every public item in the submodule should be surfaced:

```rust
pub mod environment;
pub mod pagination;

pub use environment::*;
pub use pagination::*;
```

Usage:

```rust
// DO
use crate::models::AppState;

// DON'T
use crate::models::environment::AppState;
```

Use glob re-exports only when the submodule is **small and well-defined**. Prefer explicit `pub use` for larger modules to keep the public surface clear.

---

## Feature Module Barrels

Inside each feature module (`user/`, `auth/`, etc.), the `mod.rs` also acts as a barrel — but its primary job is **wiring routes**, not re-exporting types. Internal submodules (`controller`, `service`, `repository`, `model`) are declared public but are only consumed within the module itself.

```rust
// src/modules/user/mod.rs
pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

pub struct UserRoutes;

impl UserRoutes {
    pub fn build() -> Router<Arc<AppState>> {
        Router::new()
            .route("/users", get(controller::list))
            .route("/users/me", get(controller::get_me))
    }
}
```

Within the module, submodules reference siblings via `super::`:

```rust
// src/modules/user/controller.rs
use super::{model::UserResponse, service};
```

They do **not** import siblings through the crate root:

```rust
// DON'T — wrong path for sibling access
use crate::modules::user::service;
use crate::modules::user::model::UserResponse;
```

---

## Rules at a Glance

| Rule | Do | Don't |
|------|----|-------|
| Import shared infrastructure | `use crate::services::{HttpError, HttpResponse}` | `use crate::services::http_error::HttpError` |
| Import extractors | `use crate::extractors::{AuthUser, BodyJson}` | `use crate::extractors::auth::AuthUser` |
| Import utilities | `use crate::utils::{hash_password, create_token}` | `use crate::utils::encrypt::hash` |
| Import sibling submodules (inside a feature) | `use super::{model::Foo, service}` | `use crate::modules::user::model::Foo` |
| Re-export with alias | `pub use encrypt::{hash as hash_password}` | Expose raw internal names like `hash` |
| Glob re-export | Only for small, cohesive modules | On large modules with many types |

---

## Adding a New Shared Utility

When you add a new file to a shared directory, always update the barrel:

1. Create `src/utils/my_util.rs` with your functions.
2. Open `src/utils/mod.rs` and add:

```rust
pub mod my_util;

pub use my_util::my_function;         // direct re-export
// or with alias:
pub use my_util::internal_fn as my_function;
```

3. Consumers now import: `use crate::utils::my_function;`

Never ask consumers to import from `crate::utils::my_util::my_function` directly.

---

## Adding a New Shared Service

Same pattern as utilities:

1. Create `src/services/my_service.rs`.
2. Update `src/services/mod.rs`:

```rust
pub mod my_service;

pub use my_service::MyService;
```

3. Consumers import: `use crate::services::MyService;`

---

## Why This Matters

- **Refactoring is safe**: if you rename or split an internal file, only the barrel needs updating — not every consumer across the codebase.
- **Imports stay short**: one `use crate::services::{HttpError, HttpResponse, DBSqlite}` instead of three separate deep paths.
- **Discoverability**: the barrel is the authoritative list of what a module exposes publicly. Reading `mod.rs` tells you everything available.
- **Encapsulation**: internal implementation details (`fn hash`, `fn id`) stay hidden; the barrel surface (`hash_password`, `generate_id`) is what the rest of the app knows about.
