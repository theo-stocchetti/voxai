#!/usr/bin/env bash

# VoxAI Development Environment Setup Script
# This script installs all required dependencies and tools for VoxAI development

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            DISTRO=$ID
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        OS="windows"
    else
        error "Unsupported OS: $OSTYPE"
        exit 1
    fi

    info "Detected OS: $OS${DISTRO:+ ($DISTRO)}"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Rust
install_rust() {
    if command_exists rustc; then
        success "Rust is already installed ($(rustc --version))"
    else
        info "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        success "Rust installed successfully"
    fi
}

# Install system dependencies for Linux
install_linux_deps() {
    info "Installing Linux system dependencies..."

    case "$DISTRO" in
        ubuntu|debian)
            sudo apt-get update
            sudo apt-get install -y \
                build-essential \
                pkg-config \
                libasound2-dev \
                libxdo-dev \
                libgtk-3-dev \
                libgdk-pixbuf2.0-dev \
                libpango1.0-dev \
                libatk1.0-dev \
                libcairo2-dev \
                libssl-dev \
                curl \
                git
            ;;
        fedora|rhel|centos)
            sudo dnf install -y \
                gcc \
                pkg-config \
                alsa-lib-devel \
                xdotool-devel \
                gtk3-devel \
                gdk-pixbuf2-devel \
                pango-devel \
                atk-devel \
                cairo-devel \
                openssl-devel \
                curl \
                git
            ;;
        arch|manjaro)
            sudo pacman -S --needed \
                base-devel \
                pkg-config \
                alsa-lib \
                xdotool \
                gtk3 \
                gdk-pixbuf2 \
                pango \
                atk \
                cairo \
                openssl \
                curl \
                git
            ;;
        *)
            warn "Unsupported Linux distribution: $DISTRO"
            warn "Please install the following packages manually:"
            echo "  - ALSA development libraries"
            echo "  - GTK3 development libraries"
            echo "  - xdotool development libraries"
            echo "  - OpenSSL development libraries"
            return 1
            ;;
    esac

    success "Linux dependencies installed"
}

# Install system dependencies for macOS
install_macos_deps() {
    info "Installing macOS system dependencies..."

    if ! command_exists brew; then
        info "Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi

    brew install pkg-config

    success "macOS dependencies installed"
}

# Install GitHub CLI
install_github_cli() {
    if command_exists gh; then
        success "GitHub CLI is already installed ($(gh --version | head -n1))"
    else
        info "Installing GitHub CLI..."

        case "$OS" in
            linux)
                case "$DISTRO" in
                    ubuntu|debian)
                        curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
                        sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
                        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
                        sudo apt update
                        sudo apt install gh -y
                        ;;
                    fedora|rhel|centos)
                        sudo dnf install -y gh
                        ;;
                    arch|manjaro)
                        sudo pacman -S github-cli
                        ;;
                    *)
                        warn "Cannot auto-install GitHub CLI for $DISTRO"
                        warn "Please install manually: https://cli.github.com/manual/installation"
                        return 1
                        ;;
                esac
                ;;
            macos)
                brew install gh
                ;;
            windows)
                warn "Please install GitHub CLI manually from: https://cli.github.com/manual/installation"
                return 1
                ;;
        esac

        success "GitHub CLI installed"
    fi

    # Check if authenticated
    if gh auth status >/dev/null 2>&1; then
        success "GitHub CLI is authenticated"
    else
        warn "GitHub CLI is not authenticated"
        info "Run 'gh auth login' to authenticate"
    fi
}

# Verify compilation
verify_compilation() {
    info "Verifying project compilation..."

    if cargo build --verbose; then
        success "Project compiles successfully!"
    else
        error "Project failed to compile"
        error "Please check the error messages above"
        return 1
    fi
}

# Run cargo checks
run_checks() {
    info "Running code quality checks..."

    # Format check
    info "Checking code formatting..."
    if cargo fmt --all -- --check; then
        success "Code formatting is correct"
    else
        warn "Code formatting issues found. Run 'cargo fmt --all' to fix"
    fi

    # Clippy
    info "Running clippy linter..."
    if cargo clippy --all-targets --all-features -- -D warnings; then
        success "No clippy warnings found"
    else
        warn "Clippy warnings found. Please fix them before committing"
    fi
}

# Main setup flow
main() {
    echo "========================================="
    echo "  VoxAI Development Environment Setup"
    echo "========================================="
    echo ""

    detect_os
    echo ""

    info "Step 1/6: Installing Rust..."
    install_rust
    echo ""

    info "Step 2/6: Installing system dependencies..."
    case "$OS" in
        linux)
            install_linux_deps
            ;;
        macos)
            install_macos_deps
            ;;
        windows)
            warn "Windows detected. Please install dependencies manually:"
            echo "  - Visual Studio Build Tools"
            echo "  - Git for Windows"
            ;;
    esac
    echo ""

    info "Step 3/6: Installing GitHub CLI..."
    install_github_cli || true  # Don't fail if gh install fails
    echo ""

    info "Step 4/6: Verifying Rust toolchain..."
    if command_exists cargo; then
        success "Cargo is available ($(cargo --version))"
    else
        error "Cargo not found. Please install Rust manually from https://rustup.rs"
        exit 1
    fi
    echo ""

    info "Step 5/6: Verifying project compilation..."
    verify_compilation || true  # Don't fail on compilation errors
    echo ""

    info "Step 6/6: Running code quality checks..."
    run_checks || true  # Don't fail on check errors
    echo ""

    echo "========================================="
    success "Development environment setup complete!"
    echo "========================================="
    echo ""

    info "Next steps:"
    echo "  1. If GitHub CLI is not authenticated, run: gh auth login"
    echo "  2. Run tests: cargo test"
    echo "  3. Run the application: cargo run"
    echo "  4. Before committing, run: ./scripts/check-ci.sh"
    echo ""

    info "Useful commands:"
    echo "  - Format code: cargo fmt --all"
    echo "  - Run linter: cargo clippy --all-targets --all-features"
    echo "  - Check CI locally: ./scripts/check-ci.sh"
    echo "  - View CI runs: gh run list"
    echo "  - View CI logs: gh run view --log"
}

# Run main function
main "$@"
