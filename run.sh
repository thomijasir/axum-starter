#!/usr/bin/env bash
# run.sh — minimal task runner (simple case-switch + tiny env loader)
# Usage:
#   ./run.sh dev                      # loads .env and runs cargo run
#   ./run.sh dev:staging              # loads .env.staging and runs cargo run
#   ./run.sh dev:production           # loads .env.production and runs cargo run --release
#   ./run.sh start                    # production start (alias of dev:production)
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

usage() {
  cat <<'EOF'
Usage: ./run.sh <command> [-- <args...>]

App:
  start                   Load .env.production, run cargo run --release
  dev                     Load .env, run cargo run
  dev:staging             Load .env.staging, run cargo run
  dev:production          Load .env.production, run cargo run --release
  build                   cargo build --release
  build:staging           Load .env.staging, build --release
  build:production        Load .env.production, build --release

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
    cargo run --release -- "$@"
    ;;
  dev)
    load_env ".env.local"
    cargo run -- -- "$@"
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

  # ------------- Database (Diesel) ---------
  db:migration:create)
    ensure_command diesel
    diesel migration create "$@"
    ;;
  db:migration:run)
    ensure_command diesel
    diesel migration run "$@"
    ;;
  db:migration:revert)
    ensure_command diesel
    diesel migration revert "$@"
    ;;
  db:migration:reset)
    ensure_command diesel
    diesel migration redo "$@"
    ;;
  db:migration:status)
    ensure_command diesel
    diesel migration list "$@"
    ;;
  db:migration:schema)
    ensure_command diesel
    mkdir -p src/schema
    diesel print-schema >src/schema/table.rs
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
