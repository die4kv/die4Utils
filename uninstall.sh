#!/bin/bash

set -e

PROJECT_NAME="die4Utils"
BINARY_NAME="die4Utils"
USER_BIN="$HOME/.cargo/bin"
GLOBAL_BIN="/usr/local/bin"

MODE=""

show_help() {
    echo "Usage: ./uninstall.sh [OPTIONS]"
    echo
    echo "Uninstalls $PROJECT_NAME binary."
    echo
    echo "Options:"
    echo "  --local           Remove binary from $USER_BIN"
    echo "  --global          Remove binary from $GLOBAL_BIN (requires sudo)"
    echo "  -h, --help        Show this help message"
    echo
    echo "If no option is given, you will be prompted interactively."
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --local)
            MODE="local"
            shift
            ;;
        --global)
            MODE="global"
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

echo "== Uninstaller untuk $PROJECT_NAME =="

if [[ -z "$MODE" ]]; then
    echo "Pilih lokasi penghapusan binary:"
    echo "1) Lokal pengguna ($USER_BIN)"
    echo "2) Global ($GLOBAL_BIN, membutuhkan sudo)"
    read -p "Pilihan [1/2]: " choice
    case "$choice" in
        1) MODE="local" ;;
        2) MODE="global" ;;
        *) echo "Pilihan tidak valid." ; exit 1 ;;
    esac
fi

case "$MODE" in
    local)
        TARGET="$USER_BIN/$BINARY_NAME"
        if [[ -f "$TARGET" ]]; then
            rm "$TARGET"
            echo "Dihapus: $TARGET"
        else
            echo "Binary tidak ditemukan di $TARGET"
        fi
        ;;
    global)
        TARGET="$GLOBAL_BIN/$BINARY_NAME"
        if [[ "$EUID" -ne 0 ]]; then
            echo "ERROR: Penghapusan global membutuhkan sudo."
            echo "Silakan jalankan kembali dengan:"
            echo "  sudo ./uninstall.sh --global"
            exit 1
        fi
        if [[ -f "$TARGET" ]]; then
            rm "$TARGET"
            echo "Dihapus: $TARGET"
        else
            echo "Binary tidak ditemukan di $TARGET"
        fi
        ;;
    *)
        echo "Mode tidak valid: $MODE"
        exit 1
        ;;
esac

echo "Uninstall selesai."
