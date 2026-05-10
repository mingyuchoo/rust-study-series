#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

cd "${PROJECT_ROOT}"

APP_PACKAGE="${APP_PACKAGE:-webcam-detector}"
ACTION="${1:-all}"

shift || true
if [[ "${1:-}" == "--" ]]; then
  shift
fi

usage() {
  cat <<EOF
Usage: scripts/run.sh [build|test|run|release|package|ci|all] [-- <cargo run args>]

Actions:
  build  Build the current codebase.
  test   Run all tests.
  run    Run the application.
  release Build an optimized desktop binary.
  package Copy the release binary into dist/.
  ci     Format, check, clippy, and test.
  all    Build, test, then run. This is the default.

Environment:
  APP_PACKAGE  Workspace package to run. Default: ${APP_PACKAGE}
EOF
}

cargo_build() {
  cargo build --workspace --profile dev --all-features
}

cargo_test() {
  cargo test --workspace --all-targets --all-features
}

cargo_run() {
  cargo run -p "${APP_PACKAGE}" -- "$@"
}

cargo_release() {
  cargo build --release -p "${APP_PACKAGE}"
}

cargo_package() {
  cargo_release

  mkdir -p dist
  local exe_name="${APP_PACKAGE}"
  if [[ "${OS:-}" == "Windows_NT" ]]; then
    exe_name="${APP_PACKAGE}.exe"
  fi

  cp "target/release/${exe_name}" "dist/${exe_name}"
  printf 'Packaged desktop binary: dist/%s\n' "${exe_name}"
}

cargo_ci() {
  cargo fmt --all
  cargo check --workspace --all-targets --all-features
  cargo clippy --workspace --all-targets --all-features
  cargo_test
}

case "${ACTION}" in
  build)
    cargo_build
    ;;
  test)
    cargo_test
    ;;
  run)
    cargo_run "$@"
    ;;
  release)
    cargo_release
    ;;
  package)
    cargo_package
    ;;
  ci)
    cargo_ci
    ;;
  all)
    cargo_build
    cargo_test
    cargo_run "$@"
    ;;
  -h|--help|help)
    usage
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac
