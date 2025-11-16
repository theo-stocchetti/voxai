#!/usr/bin/env bash

# VoxAI CI Validation Script
# This script runs the same checks as the CI pipeline locally
# Run this before committing to catch issues early

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Track failures
FAILED_CHECKS=0

# Helper functions
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

fail() {
    echo -e "${RED}[✗]${NC} $1"
    FAILED_CHECKS=$((FAILED_CHECKS + 1))
}

section() {
    echo ""
    echo "========================================="
    echo "  $1"
    echo "========================================="
    echo ""
}

# Check code formatting
check_formatting() {
    section "Checking Code Formatting"
    info "Running: cargo fmt --all -- --check"

    if cargo fmt --all -- --check; then
        success "Code formatting is correct"
        return 0
    else
        fail "Code formatting issues found"
        echo ""
        echo "  Fix with: cargo fmt --all"
        return 1
    fi
}

# Run clippy linter
check_clippy() {
    section "Running Clippy Linter"
    info "Running: cargo clippy --all-targets --all-features -- -D warnings"

    if cargo clippy --all-targets --all-features -- -D warnings; then
        success "No clippy warnings"
        return 0
    else
        fail "Clippy warnings found"
        echo ""
        echo "  Fix the warnings above before committing"
        return 1
    fi
}

# Build project (debug mode)
check_build() {
    section "Building Project (Debug)"
    info "Running: cargo build --verbose"

    if cargo build --verbose; then
        success "Build succeeded"
        return 0
    else
        fail "Build failed"
        return 1
    fi
}

# Run tests
check_tests() {
    section "Running Tests"
    info "Running: cargo test --verbose"

    if cargo test --verbose; then
        success "All tests passed"
        return 0
    else
        fail "Tests failed"
        return 1
    fi
}

# Optional: Check with specific features
check_features() {
    local feature=$1

    if [ -z "$feature" ]; then
        return 0
    fi

    section "Building with Feature: $feature"
    info "Running: cargo build --features $feature"

    if cargo build --features "$feature"; then
        success "Build with feature '$feature' succeeded"
        return 0
    else
        fail "Build with feature '$feature' failed"
        return 1
    fi
}

# Display summary
show_summary() {
    echo ""
    echo "========================================="
    if [ $FAILED_CHECKS -eq 0 ]; then
        success "ALL CHECKS PASSED! ✨"
        echo "========================================="
        echo ""
        info "Your code is ready to commit and push!"
        return 0
    else
        fail "CHECKS FAILED: $FAILED_CHECKS"
        echo "========================================="
        echo ""
        echo "  Please fix the issues above before committing"
        return 1
    fi
}

# Main execution
main() {
    echo ""
    echo "╔═══════════════════════════════════════╗"
    echo "║   VoxAI CI Validation Script          ║"
    echo "║   Running local CI checks...          ║"
    echo "╚═══════════════════════════════════════╝"
    echo ""

    # Parse arguments
    RUN_ALL=true
    FEATURES=""

    while [[ $# -gt 0 ]]; do
        case $1 in
            --feature)
                FEATURES="$2"
                shift 2
                ;;
            --format-only)
                RUN_ALL=false
                check_formatting || true
                shift
                ;;
            --clippy-only)
                RUN_ALL=false
                check_clippy || true
                shift
                ;;
            --build-only)
                RUN_ALL=false
                check_build || true
                shift
                ;;
            --test-only)
                RUN_ALL=false
                check_tests || true
                shift
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --format-only    Only check formatting"
                echo "  --clippy-only    Only run clippy"
                echo "  --build-only     Only build the project"
                echo "  --test-only      Only run tests"
                echo "  --feature NAME   Build with specific feature"
                echo "  --help           Show this help message"
                echo ""
                echo "Examples:"
                echo "  $0                      # Run all checks"
                echo "  $0 --format-only        # Only check formatting"
                echo "  $0 --feature cuda       # Build with CUDA feature"
                exit 0
                ;;
            *)
                echo "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done

    # Run checks
    if [ "$RUN_ALL" = true ]; then
        check_formatting || true
        check_clippy || true
        check_build || true
        check_tests || true

        if [ -n "$FEATURES" ]; then
            check_features "$FEATURES" || true
        fi
    fi

    # Show summary
    show_summary
}

# Run main function
main "$@"
