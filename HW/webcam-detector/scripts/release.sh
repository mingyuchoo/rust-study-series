#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

cd "${PROJECT_ROOT}"

APP_PACKAGE="${APP_PACKAGE:-webcam-detector}"
APP_DISPLAY_NAME="${APP_DISPLAY_NAME:-Webcam Detector}"
APP_DESCRIPTION="${APP_DESCRIPTION:-Cross-platform desktop webcam viewer}"
MAINTAINER="${MAINTAINER:-webcam-detector maintainers <maintainers@example.com>}"
LICENSE="${LICENSE:-MIT}"
ACTION="${1:-auto}"

DIST_DIR="${PROJECT_ROOT}/dist"
WORK_DIR="${DIST_DIR}/release"

usage() {
  cat <<EOF
Usage: scripts/release.sh [auto|linux|deb|rpm|dmg|msi|all|help]

Targets:
  auto   Build installer(s) for the current platform.
  linux  Build Linux .deb and .rpm installers.
  deb    Build a Linux .deb installer.
  rpm    Build a Linux .rpm installer. Requires rpmbuild.
  dmg    Build a macOS .dmg installer. Requires macOS hdiutil.
  msi    Build a Windows .msi installer by delegating to scripts/release.ps1.
  all    Build every installer supported by the current platform.

Environment:
  APP_PACKAGE       Cargo package to bundle. Default: ${APP_PACKAGE}
  APP_DISPLAY_NAME  Human-readable app name. Default: ${APP_DISPLAY_NAME}
  MAINTAINER        Linux package maintainer. Default: ${MAINTAINER}
  LICENSE           Package license label. Default: ${LICENSE}
EOF
}

require_command() {
  local name="$1"
  if ! command -v "${name}" >/dev/null 2>&1; then
    printf 'error: required command not found: %s\n' "${name}" >&2
    exit 1
  fi
}

version() {
  cargo pkgid -p "${APP_PACKAGE}" | sed 's/.*#//'
}

host_os() {
  uname -s | tr '[:upper:]' '[:lower:]'
}

host_arch() {
  uname -m
}

deb_arch() {
  case "$(host_arch)" in
    x86_64|amd64) printf 'amd64' ;;
    aarch64|arm64) printf 'arm64' ;;
    armv7l) printf 'armhf' ;;
    *) host_arch ;;
  esac
}

rpm_arch() {
  case "$(host_arch)" in
    aarch64|arm64) printf 'aarch64' ;;
    x86_64|amd64) printf 'x86_64' ;;
    *) host_arch ;;
  esac
}

build_release() {
  cargo build --release -p "${APP_PACKAGE}"
}

binary_path() {
  local exe_name="${APP_PACKAGE}"
  if [[ "${OS:-}" == "Windows_NT" ]]; then
    exe_name="${APP_PACKAGE}.exe"
  fi

  printf '%s/target/release/%s' "${PROJECT_ROOT}" "${exe_name}"
}

package_slug() {
  printf '%s' "${APP_PACKAGE}" | tr '[:upper:]' '[:lower:]'
}

desktop_file_contents() {
  cat <<EOF
[Desktop Entry]
Type=Application
Name=${APP_DISPLAY_NAME}
Comment=${APP_DESCRIPTION}
Exec=${APP_PACKAGE}
Terminal=false
Categories=AudioVideo;Video;
EOF
}

prepare_linux_root() {
  local root="$1"

  rm -rf "${root}"
  mkdir -p "${root}/usr/bin" "${root}/usr/share/applications"
  install -m 0755 "$(binary_path)" "${root}/usr/bin/${APP_PACKAGE}"
  desktop_file_contents >"${root}/usr/share/applications/$(package_slug).desktop"
}

build_deb() {
  case "$(host_os)" in
    linux) ;;
    *)
      printf 'error: .deb packaging must run on Linux.\n' >&2
      exit 1
      ;;
  esac

  require_command ar
  require_command tar
  require_command gzip
  build_release

  local version package arch pkg_root debian_dir output control_size
  version="$(version)"
  package="$(package_slug)"
  arch="$(deb_arch)"
  pkg_root="${WORK_DIR}/deb/${package}_${version}_${arch}"
  debian_dir="${pkg_root}/DEBIAN"
  output="${DIST_DIR}/${package}_${version}_${arch}.deb"

  prepare_linux_root "${pkg_root}"
  mkdir -p "${debian_dir}"
  control_size="$(du -sk "${pkg_root}" | awk '{print $1}')"

  cat >"${debian_dir}/control" <<EOF
Package: ${package}
Version: ${version}
Section: video
Priority: optional
Architecture: ${arch}
Maintainer: ${MAINTAINER}
Installed-Size: ${control_size}
Depends: libc6, libx11-6, libxrandr2, libxi6, libxcursor1, libgl1, libv4l-0
Description: ${APP_DESCRIPTION}
 ${APP_DISPLAY_NAME} is a desktop webcam viewer.
EOF

  rm -f "${output}"
  (
    cd "${pkg_root}"
    printf '2.0\n' >debian-binary
    tar --owner=0 --group=0 -czf control.tar.gz -C DEBIAN .
    tar --owner=0 --group=0 -czf data.tar.gz ./usr
    ar rcs "${output}" debian-binary control.tar.gz data.tar.gz
  )

  printf 'Created %s\n' "${output}"
}

build_rpm() {
  case "$(host_os)" in
    linux) ;;
    *)
      printf 'error: .rpm packaging must run on Linux.\n' >&2
      exit 1
      ;;
  esac

  require_command rpmbuild
  require_command tar
  require_command gzip
  build_release

  local version package arch payload_root source_tar topdir spec output
  version="$(version)"
  package="$(package_slug)"
  arch="$(rpm_arch)"
  payload_root="${WORK_DIR}/rpm/payload"
  source_tar="${WORK_DIR}/rpm/${package}-${version}-linux.tar.gz"
  topdir="${WORK_DIR}/rpm/rpmbuild"
  spec="${WORK_DIR}/rpm/${package}.spec"

  prepare_linux_root "${payload_root}"
  mkdir -p "$(dirname "${source_tar}")" "${topdir}/"{BUILD,BUILDROOT,RPMS,SOURCES,SPECS,SRPMS,rpmdb}
  tar -czf "${source_tar}" -C "${payload_root}" usr
  cp "${source_tar}" "${topdir}/SOURCES/"

  cat >"${spec}" <<EOF
Name: ${package}
Version: ${version}
Release: 1%{?dist}
Summary: ${APP_DESCRIPTION}
License: ${LICENSE}
BuildArch: ${arch}
Source0: ${package}-${version}-linux.tar.gz

%description
${APP_DISPLAY_NAME} is a desktop webcam viewer.

%prep

%build

%install
mkdir -p %{buildroot}
tar -xzf %{SOURCE0} -C %{buildroot}

%files
/usr/bin/${APP_PACKAGE}
/usr/share/applications/${package}.desktop
EOF

  rpmbuild -bb --define "_topdir ${topdir}" --define "_dbpath ${topdir}/rpmdb" "${spec}"
  mkdir -p "${DIST_DIR}"
  output="$(find "${topdir}/RPMS" -type f -name '*.rpm' | head -n 1)"
  cp "${output}" "${DIST_DIR}/"

  printf 'Created %s\n' "${DIST_DIR}/$(basename "${output}")"
}

build_dmg() {
  case "$(host_os)" in
    darwin) ;;
    *)
      printf 'error: .dmg packaging must run on macOS.\n' >&2
      exit 1
      ;;
  esac

  require_command hdiutil
  build_release

  local version app_dir contents_dir macos_dir output plist_id
  version="$(version)"
  app_dir="${WORK_DIR}/macos/${APP_DISPLAY_NAME}.app"
  contents_dir="${app_dir}/Contents"
  macos_dir="${contents_dir}/MacOS"
  output="${DIST_DIR}/$(package_slug)-${version}-macos.dmg"
  plist_id="com.example.$(package_slug)"

  rm -rf "${app_dir}" "${output}"
  mkdir -p "${macos_dir}" "${contents_dir}/Resources"
  install -m 0755 "$(binary_path)" "${macos_dir}/${APP_PACKAGE}"

  cat >"${contents_dir}/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>${APP_PACKAGE}</string>
  <key>CFBundleIdentifier</key>
  <string>${plist_id}</string>
  <key>CFBundleName</key>
  <string>${APP_DISPLAY_NAME}</string>
  <key>CFBundleDisplayName</key>
  <string>${APP_DISPLAY_NAME}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${version}</string>
  <key>CFBundleVersion</key>
  <string>${version}</string>
  <key>NSCameraUsageDescription</key>
  <string>${APP_DISPLAY_NAME} needs camera access to show the webcam stream.</string>
</dict>
</plist>
EOF

  hdiutil create -volname "${APP_DISPLAY_NAME}" -srcfolder "${app_dir}" -ov -format UDZO "${output}"
  printf 'Created %s\n' "${output}"
}

build_msi() {
  if command -v pwsh >/dev/null 2>&1; then
    pwsh -NoProfile -ExecutionPolicy Bypass -File "${SCRIPT_DIR}/release.ps1" msi
  elif command -v powershell.exe >/dev/null 2>&1; then
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File "${SCRIPT_DIR}/release.ps1" msi
  else
    printf 'error: .msi packaging requires PowerShell and WiX Toolset on Windows.\n' >&2
    exit 1
  fi
}

case "${ACTION}" in
  auto)
    case "$(host_os)" in
      linux) build_deb; build_rpm ;;
      darwin) build_dmg ;;
      mingw*|msys*|cygwin*) build_msi ;;
      *) printf 'error: unsupported platform: %s\n' "$(host_os)" >&2; exit 1 ;;
    esac
    ;;
  linux)
    build_deb
    build_rpm
    ;;
  deb)
    build_deb
    ;;
  rpm)
    build_rpm
    ;;
  dmg)
    build_dmg
    ;;
  msi)
    build_msi
    ;;
  all)
    case "$(host_os)" in
      linux) build_deb; build_rpm ;;
      darwin) build_dmg ;;
      mingw*|msys*|cygwin*) build_msi ;;
      *) printf 'error: unsupported platform: %s\n' "$(host_os)" >&2; exit 1 ;;
    esac
    ;;
  -h|--help|help)
    usage
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac
