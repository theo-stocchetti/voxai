# VoxAI 🎤

**Real-time Audio Transcription powered by Whisper AI**

VoxAI is a cross-platform desktop application that provides real-time speech-to-text transcription using OpenAI's Whisper model. Press a global keyboard shortcut to speak, and watch your words appear as text instantly in any application.

## 🌟 Features

- **Real-time Transcription**: Latency < 2 seconds from speech to text
- **Cross-Platform**: Works on Windows 11, macOS (Intel & ARM), and Linux
- **Hardware Acceleration**: GPU support via CUDA, Metal, and OpenCL
- **Privacy-Focused**: 100% local processing, no cloud dependencies
- **Global Hotkeys**: Activate from any application
- **System Tray Integration**: Minimal UI, runs in the background
- **Voice Activity Detection**: Automatically detects speech vs. silence
- **Noise Reduction**: Filters background noise for better accuracy

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- CMake (for building Whisper.cpp)
- Platform-specific dependencies (see below)

### Installation

```bash
# Clone the repository
git clone https://github.com/theo-stocchetti/voxai.git
cd voxai

# Build the project
cargo build --release

# Run
cargo run --release
```

## 📋 Platform-Specific Requirements

### Windows
- Visual Studio Build Tools or Windows SDK
- CMake

### macOS
- Xcode Command Line Tools
- CMake

### Linux
- build-essential
- cmake
- pkg-config
- libasound2-dev (for audio capture)

## ⚙️ Configuration

On first run, VoxAI will download the Whisper model (default: base, ~142 MB).

Default settings:
- **Hotkey**: `Ctrl+Shift+Space`
- **Model**: base (good balance of speed and accuracy)
- **Models directory**: `~/.voxai/models/`

## 🎯 Usage

1. **Start VoxAI**: The application runs in the system tray
2. **Press hotkey** (default: Ctrl+Shift+Space) to start recording
3. **Speak** into your microphone
4. **Press hotkey again** to stop and transcribe
5. **Text appears** in your currently active application

## 🏗️ Project Status

**Current Status**: Initial Development Phase

VoxAI is actively under development. Core features being implemented:

- ✅ Project structure and build system
- 🔄 Audio capture (CPAL)
- 🔄 Whisper.cpp integration
- 🔄 Transcription pipeline
- 🔄 System tray UI
- 🔄 Global hotkeys
- ⏳ Text injection
- ⏳ Configuration UI

## 🛠️ Development

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=info cargo run
```

### Project Structure

```
voxai/
├── src/
│   ├── audio/          # Audio capture and processing
│   ├── transcription/  # Whisper integration
│   ├── ui/             # System tray UI
│   ├── hotkeys/        # Global keyboard shortcuts
│   ├── output/         # Text injection
│   ├── config/         # Configuration management
│   └── gpu/            # GPU acceleration (optional)
├── tests/              # Integration tests
├── benches/            # Performance benchmarks
└── backlogs/           # Issue tracking
```

## 📚 Documentation

- [Architecture Guide](./backlogs/README.md)
- [Development Guide](./CLAUDE.md)
- [Issue Tracker](./backlogs/todo/)

## 🤝 Contributing

Contributions are welcome! Please read the [CLAUDE.md](./CLAUDE.md) file for development guidelines.

## 📄 License

This project is licensed under the MIT License.

## 🙏 Acknowledgments

- [Whisper.cpp](https://github.com/ggerganov/whisper.cpp) for the amazing transcription engine
- [CPAL](https://github.com/RustAudio/cpal) for cross-platform audio
- OpenAI for the Whisper model

## 📞 Support

For issues and feature requests, please use the [GitHub issue tracker](https://github.com/theo-stocchetti/voxai/issues).

---

**Made with ❤️ and Rust**
