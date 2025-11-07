#!/bin/bash

# Plugin deployment script for Rust plugin system
# This script copies built plugin libraries to the runtime plugins directory

set -e

# Determine the build profile (default to debug)
PROFILE="${1:-debug}"
TARGET_DIR="target/${PROFILE}"
PLUGINS_DIR="${TARGET_DIR}/plugins"

# Detect the operating system and set the library extension
# Check for Windows first by looking for .exe files or OS environment variable
if [[ "$OS" == "Windows_NT" ]] || [[ -f "${TARGET_DIR}/cli.exe" ]]; then
    LIB_PREFIX=""
    LIB_EXT="dll"
    PLATFORM="Windows"
else
    case "$(uname -s)" in
        Linux*)
            LIB_PREFIX="lib"
            LIB_EXT="so"
            PLATFORM="Linux"
            ;;
        Darwin*)
            LIB_PREFIX="lib"
            LIB_EXT="dylib"
            PLATFORM="macOS"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            LIB_PREFIX=""
            LIB_EXT="dll"
            PLATFORM="Windows"
            ;;
        *)
            echo "Unsupported operating system"
            exit 1
            ;;
    esac
fi

echo "Deploying plugins for ${PROFILE} build..."
echo "Platform: ${PLATFORM}"
echo "Library extension: .${LIB_EXT}"

# Create the plugins directory if it doesn't exist
mkdir -p "${PLUGINS_DIR}"
echo "Created/verified plugins directory: ${PLUGINS_DIR}"

# Counter for deployed plugins
DEPLOYED=0

# Find and copy all plugin libraries
for plugin_lib in "${TARGET_DIR}/${LIB_PREFIX}"*plugin*.${LIB_EXT}; do
    if [ -f "${plugin_lib}" ]; then
        plugin_name=$(basename "${plugin_lib}")
        echo "Deploying: ${plugin_name}"
        cp "${plugin_lib}" "${PLUGINS_DIR}/"
        DEPLOYED=$((DEPLOYED + 1))
    fi
done

if [ ${DEPLOYED} -eq 0 ]; then
    echo "Warning: No plugin libraries found in ${TARGET_DIR}"
    echo "Make sure you have built the project with: cargo build"
    exit 1
fi

echo ""
echo "Successfully deployed ${DEPLOYED} plugin(s) to ${PLUGINS_DIR}"
echo ""
echo "Plugin libraries:"
ls -lh "${PLUGINS_DIR}"

echo ""
echo "You can now run the core application with: cargo run --bin cli"
