#!/usr/bin/env bash
# run.sh — minimal task runner (simple case-switch + tiny env loader)
# Usage:
#   ./run.sh dev                      # loads .env and runs cargo run
#   ./run.sh dev:staging              # loads .env.staging and runs cargo run
#   ./run.sh dev:production           # loads .env.production and runs cargo run --release
#   ./run.sh start                    # production start (loads .env.production, execs release binary)
#   ./run.sh start:staging            # staging start (loads .env.staging, execs release binary)
#   ./run.sh build                    # cargo build --release
#   ./run.sh build:staging            # loads .env.staging then build --release
#   ./run.sh build:production         # loads .env.production then build --release
#   ./run.sh db:migration:create NAME # diesel migration create NAME
#   ./run.sh docker:up                # docker compose up -d --build
#   ./run.sh lint                     # cargo clippy -D warnings
#
# Pass extra args to cargo/diesel after --
#   ./run.sh dev -- --bin myapp --features foo

set -Eeuo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

BIN_NAME="axum-starter"
DEFAULT_BIN="$SCRIPT_DIR/target/release/$BIN_NAME"

build_release_binary() {
  ensure_command cargo
  echo "Building release binary ($BIN_NAME)..." >&2
  cargo build --release >&2
}

ensure_release_binary() {
  local resolved_bin="${RUN_BINARY_PATH:-$DEFAULT_BIN}"
  local needs_build=0

  if [[ "${FORCE_BUILD:-0}" = "1" ]]; then
    needs_build=1
  elif [[ ! -x "$resolved_bin" ]]; then
    needs_build=1
  fi

  if [[ $needs_build -eq 1 ]]; then
    if [[ "${SKIP_BINARY_BUILD:-0}" = "1" ]]; then
      echo "Release binary not available at $resolved_bin and SKIP_BINARY_BUILD=1; aborting." >&2
      exit 1
    fi
    build_release_binary
  fi

  if [[ ! -x "$resolved_bin" ]]; then
    echo "Release binary missing or not executable: $resolved_bin" >&2
    exit 1
  fi

  printf '%s' "$resolved_bin"
}

usage() {
  cat <<'EOF'
Usage: ./run.sh <command> [-- <args...>]

App:
  start                   Load .env.production, exec release binary
  start:staging           Load .env.staging, exec release binary
  dev                     Load .env, run cargo run
  dev:staging             Load .env.staging, run cargo run
  dev:production          Load .env.production, run cargo run --release
  build                   cargo build --release
  build:staging           Load .env.staging, build --release
  build:production        Load .env.production, build --release
  bin:ensure              Build (or verify) release binary without running it

Database (Diesel):
  db:migration:create NAME   diesel migration create NAME
  db:migration:run           diesel migration run
  db:migration:revert        diesel migration revert
  db:migration:reset         diesel migration redo
  db:migration:status        diesel migration list
  db:migration:schema        mkdir -p src/schema && diesel print-schema > src/schema/table.rs

Docker:
  docker:up              docker compose up -d --build
  docker:down            docker compose down

Code Quality:
  lint                   cargo clippy -- -D warnings
  lint:fix               cargo clippy --fix --allow-dirty --allow-staged
  format                 cargo fmt
  format:check           cargo fmt -- --check

Examples:
  ./run.sh dev -- --bin api --features tracing
  ./run.sh db:migration:create add_users
EOF
}

ensure_command() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "Required command not found: $1" >&2
    exit 1
  }
}

# Simple env loader: load_env ".env" (path can be relative to this script)
load_env() {
  local file="$1"
  local path="$file"
  # Resolve relative to script dir if not absolute
  if [[ "$file" != /* ]]; then
    path="$SCRIPT_DIR/$file"
  fi
  if [[ -f "$path" ]]; then
    echo "Loading env: $path"
    set -a
    # shellcheck disable=SC1090
    . "$path"
    set +a
  else
    echo "(env file not found, skipping): $path"
  fi
}

cmd="${1:-help}"
shift || true

case "$cmd" in
  help | -h | --help)
    usage
    ;;

  # ------------------ App ------------------
  start)
    load_env ".env.production"
    bin_path="$(ensure_release_binary)"
    exec "$bin_path" "$@"
    ;;
  start:staging)
    load_env ".env.staging"
    bin_path="$(ensure_release_binary)"
    exec "$bin_path" "$@"
    ;;
  release)
    load_env ".env.production"
    cargo run --release -- "$@"
    ;;
  release:staging)
    load_env ".env.staging"
    cargo run --release -- "$@"
    ;;
  dev)
    load_env ".env.local"
    cargo run
    ;;
  dev:watch)
    load_env ".env.local"
    cargo watch -q -c -w src/ -x run
    ;;
  dev:staging)
    load_env ".env.staging"
    cargo run -- -- "$@"
    ;;
  dev:production)
    load_env ".env.production"
    cargo run --release -- "$@"
    ;;
  build)
    cargo build --release "$@"
    ;;
  build:staging)
    load_env ".env.staging"
    cargo build --release "$@"
    ;;
  build:production)
    load_env ".env.production"
    cargo build --release "$@"
    ;;
  bin:ensure)
    bin_path="$(ensure_release_binary)"
    echo "Release binary ready at: $bin_path"
    ;;

  # ------------- Database (Diesel) ---------
  db:migration:create)
    ensure_command diesel
    diesel migration create "$@"
    ;;
  db:migration:run)
    load_env ".env.local"
    ensure_command diesel
    diesel migration run "$@"
    ;;
  db:migration:revert)
    load_env ".env.local"
    ensure_command diesel
    diesel migration revert "$@"
    ;;
  db:migration:reset)
    load_env ".env.local"
    ensure_command diesel
    diesel migration redo "$@"
    ;;
  db:migration:status)
    load_env ".env.local"
    ensure_command diesel
    diesel migration list "$@"
    ;;
  db:migration:schema)
    load_env ".env.local"
    ensure_command diesel
    mkdir -p src/schemas
    diesel print-schema >src/schemas/table.rs
    ;;

  # ------------------ Docker ----------------
  docker:up)
    ensure_command docker
    docker compose up -d --build
    ;;
  docker:down)
    ensure_command docker
    docker compose down
    ;;

  # --------------- Code quality -------------
  check)
    cargo check
    ;;
  lint)
    cargo clippy -- -D warnings
    ;;
  lint:fix)
    cargo clippy --fix --allow-dirty --allow-staged
    ;;
  format)
    cargo fmt
    ;;
  format:check)
    cargo fmt -- --check
    ;;

  *)
    echo "Unknown command: $cmd" >&2
    echo
    usage
    exit 1
    ;;

esac
