#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
WASM_DIR="$ROOT_DIR/wasm-lib"
REACT_DIR="$ROOT_DIR/react-app"

# 색상 출력
info()  { printf '\033[1;34m[INFO]\033[0m  %s\n' "$1"; }
ok()    { printf '\033[1;32m[OK]\033[0m    %s\n' "$1"; }
err()   { printf '\033[1;31m[ERR]\033[0m   %s\n' "$1" >&2; }

check_prerequisites() {
  local missing=()
  command -v rustup  >/dev/null || missing+=("rustup")
  command -v wasm-pack >/dev/null || missing+=("wasm-pack (cargo install wasm-pack)")
  command -v bun     >/dev/null || missing+=("bun")

  if (( ${#missing[@]} > 0 )); then
    err "필수 도구가 설치되지 않았습니다:"
    for m in "${missing[@]}"; do
      err "  - $m"
    done
    exit 1
  fi

  if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    info "wasm32-unknown-unknown 타겟 추가 중..."
    rustup target add wasm32-unknown-unknown
  fi
}

wasm_build() {
  local profile="${1:-dev}"
  info "WASM 빌드 시작 (profile: $profile)..."

  cd "$WASM_DIR"

  if [[ "$profile" == "release" ]]; then
    wasm-pack build --target web --out-dir pkg --release
  else
    wasm-pack build --target web --out-dir pkg --dev
  fi

  ok "WASM 빌드 완료 → wasm-lib/pkg/"
}

react_install() {
  info "React 앱 의존성 설치 중..."
  cd "$REACT_DIR"
  bun install
  ok "의존성 설치 완료"
}

cmd_dev() {
  check_prerequisites
  wasm_build dev
  react_install

  info "개발 서버 시작..."
  cd "$REACT_DIR"
  exec bun run dev
}

cmd_build() {
  check_prerequisites
  wasm_build release
  react_install

  info "React 프로덕션 빌드 시작..."
  cd "$REACT_DIR"
  bun run build
  ok "프로덕션 빌드 완료 → react-app/dist/"
}

cmd_clean() {
  info "빌드 산출물 정리 중..."

  rm -rf "$WASM_DIR/pkg"
  rm -rf "$WASM_DIR/target"
  rm -rf "$REACT_DIR/node_modules"
  rm -rf "$REACT_DIR/dist"
  rm -rf "$REACT_DIR/node_modules/.vite"

  ok "정리 완료"
}

usage() {
  cat <<EOF
사용법: $0 <command>

Commands:
  dev     WASM 빌드(dev) + 의존성 설치 + Vite 개발 서버 실행
  build   WASM 빌드(release) + React 프로덕션 빌드
  clean   모든 빌드 산출물 제거 (pkg, target, node_modules, dist)
EOF
}

case "${1:-}" in
  dev)   cmd_dev   ;;
  build) cmd_build ;;
  clean) cmd_clean ;;
  *)     usage; exit 1 ;;
esac
