#!/usr/bin/env bash
set -e

main() {
    BIN_DIR=${BIN_DIR-"$HOME/.bin"}
    mkdir -p "$BIN_DIR"

    case $SHELL in
    */zsh)
        PROFILE=$HOME/.zshrc
        ;;
    */bash)
        PROFILE=$HOME/.bashrc
        ;;
    */fish)
        PROFILE=$HOME/.config/fish/config.fish
        ;;
    */ash)
        PROFILE=$HOME/.profile
        ;;
    *)
        echo "could not detect shell, manually add ${BIN_DIR} to your PATH."
        exit 1
    esac

    if [[ ":$PATH:" != *":${BIN_DIR}:"* ]]; then
        echo >> "$PROFILE" && echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$PROFILE"
    fi

    PLATFORM="$(uname -s)"
        case $PLATFORM in
        Linux)
            PLATFORM="linux"
            ;;
        Darwin)
            PLATFORM="darwin"
            ;;
        *)
            err "unsupported platform: $PLATFORM"
            ;;
        esac

        ARCHITECTURE="$(uname -m)"
        if [ "${ARCHITECTURE}" = "x86_64" ]; then
        # Redirect stderr to /dev/null to avoid printing errors if non Rosetta.
        if [ "$(sysctl -n sysctl.proc_translated 2>/dev/null)" = "1" ]; then
            ARCHITECTURE="aarch64" # Rosetta.
        else
            ARCHITECTURE="x86_64" # Intel.
        fi
        elif [ "${ARCHITECTURE}" = "arm64" ] ||[ "${ARCHITECTURE}" = "aarch64" ] ; then
        ARCHITECTURE="aarch64" # Arm.
        else
        ARCHITECTURE="x86_64" # Amd.
        fi

    BINARY_URL="https://github.com/m1guelpf/plz-cli/releases/latest/download/plz-${PLATFORM}-${ARCHITECTURE}"
    echo "$BINARY_URL"

    echo "downloading latest binary"
    ensure curl -L "$BINARY_URL" -o "$BIN_DIR/plz"
    chmod +x "$BIN_DIR/plz"

    echo "installed - $("$BIN_DIR/plz" --version)"
}

# Run a command that should never fail. If the command fails execution
# will immediately terminate with an error showing the failing
# command.
ensure() {
  if ! "$@"; then err "command failed: $*"; fi
}

main "$@" || exit 1
