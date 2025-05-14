#!/bin/bash

set -e

PROJECT_NAME="die4Utils"
BINARY_NAME="die4Utils"
USER_BIN="$HOME/.cargo/bin"
GLOBAL_BIN="/usr/local/bin"

MODE=""
SHELL_TYPE=""

show_help() {
    echo
    echo "Usage: ./install.sh [OPTIONS]"
    echo
    echo "Installs $PROJECT_NAME binary."
    echo
    echo "Options:"
    echo "  --local           Install to $USER_BIN"
    echo "  --global          Install to $GLOBAL_BIN (requires sudo for copying)"
    echo "  --shell [bash|zsh|fish]  Set PATH in shell config (only for --local)"
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
        --shell)
            SHELL_TYPE="$2"
            shift 2
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

echo "== $PROJECT_NAME Installer =="

if [[ -z "$MODE" ]]; then
    echo "Pilih lokasi instalasi:"
    echo "1) Lokal pengguna ($USER_BIN)"
    echo "2) Global ($GLOBAL_BIN, membutuhkan sudo)"
    read -p "Pilihan [1/2]: " choice
    case "$choice" in
        1) MODE="local" ;;
        2) MODE="global" ;;
        *) echo "Pilihan tidak valid." ; exit 1 ;;
    esac
fi

echo "Membangun binary..."
cargo build --release

case "$MODE" in
    local)
        mkdir -p "$USER_BIN"
        cp "target/release/$BINARY_NAME" "$USER_BIN/"
        echo "Binary dipasang di: $USER_BIN"

        # Tambahkan PATH jika belum ada
        if ! echo "$PATH" | grep -q "$USER_BIN"; then
            if [[ -z "$SHELL_TYPE" ]]; then
                echo
                echo "$USER_BIN belum ada di PATH."
                echo "Pilih shell yang digunakan:"
                echo "1) Bash"
                echo "2) Zsh"
                echo "3) Fish"
                read -p "Shell [1/2/3]: " shell_choice
                case "$shell_choice" in
                    1) SHELL_TYPE="bash" ;;
                    2) SHELL_TYPE="zsh" ;;
                    3) SHELL_TYPE="fish" ;;
                    *) echo "Shell tidak valid, lewati penambahan PATH." ;;
                esac
            fi

            case "$SHELL_TYPE" in
                bash)
                    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.bashrc"
                    echo "PATH ditambahkan ke .bashrc"
                    ;;
                zsh)
                    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.zshrc"
                    echo "PATH ditambahkan ke .zshrc"
                    ;;
                fish)
                    mkdir -p "$HOME/.config/fish"
                    echo 'set -gx PATH $HOME/.cargo/bin $PATH' >> "$HOME/.config/fish/config.fish"
                    echo "PATH ditambahkan ke config.fish"
                    ;;
                *)
                    echo "Shell tidak valid, PATH tidak ditambahkan."
                    ;;
            esac
        fi
        ;;
    global)
        echo "Memindahkan binary ke $GLOBAL_BIN (memerlukan sudo)..."
        sudo cp "target/release/$BINARY_NAME" "$GLOBAL_BIN/"
        echo "Binary dipasang di: $GLOBAL_BIN"
        ;;
    *)
        echo "Mode tidak valid: $MODE"
        exit 1
        ;;
esac

echo
echo "Instalasi selesai."
