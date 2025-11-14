# VoxAI User Guide

**Version**: 0.1.0
**Last Updated**: 2025-11-14

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Usage](#basic-usage)
3. [Configuration](#configuration)
4. [Keyboard Shortcuts](#keyboard-shortcuts)
5. [Troubleshooting](#troubleshooting)
6. [FAQ](#faq)

---

## Getting Started

### Installation

**Windows:**
1. Download `VoxAI-Windows-x64.zip`
2. Extract to a folder (e.g., `C:\Program Files\VoxAI\`)
3. Run `voxai.exe`
4. VoxAI icon will appear in system tray

**macOS:**
1. Download `VoxAI-macOS.dmg`
2. Open DMG and drag VoxAI to Applications
3. Run VoxAI from Applications
4. Grant permissions when prompted:
   - Microphone access
   - Accessibility (for text injection)
   - Input Monitoring (for global hotkeys)

**Linux:**
1. Download `VoxAI-Linux-x64.AppImage` or `.deb`/`.rpm`
2. Make executable: `chmod +x VoxAI-Linux-x64.AppImage`
3. Run: `./VoxAI-Linux-x64.AppImage`
4. Icon appears in system tray (GNOME/KDE/XFCE)

### First Launch

On first launch, VoxAI will:
1. Download the Whisper AI model (default: `base`, 142MB)
2. Create config file at:
   - Windows: `%APPDATA%\voxai\config.json`
   - macOS: `~/Library/Application Support/voxai/config.json`
   - Linux: `~/.config/voxai/config.json`
3. Show welcome notification

---

## Basic Usage

### Recording Audio

1. **Press the hotkey** (default: `Ctrl+Shift+Space` on Windows/Linux, `Cmd+Shift+R` on macOS)
2. **Speak** into your microphone
3. **Press the hotkey again** to stop recording
4. Transcribed text will be **typed automatically** into the active application

### System Tray Menu

Right-click the VoxAI icon in the system tray:

- **Start Recording** - Begin voice transcription
- **Settings** - Open settings window
- **Quit** - Exit VoxAI

### Icon States

- **Gray**: Idle (ready to record)
- **Red**: Recording audio
- **Blue**: Processing transcription

---

## Configuration

### Opening Settings

- **System Tray**: Right-click icon → **Settings**
- **Keyboard**: `Ctrl+,` (or `Cmd+,` on macOS)

### Settings Options

#### General

- **Hotkey**: Global keyboard shortcut (e.g., `Ctrl+Shift+Space`)
- **Model Size**: Whisper model quality/speed tradeoff
  - **Tiny** (75MB): Fastest, least accurate
  - **Base** (142MB): Good balance (recommended)
  - **Small** (466MB): Better accuracy, slower
  - **Medium** (1.5GB): Best accuracy, requires GPU

#### Audio

- **Noise Reduction**: Remove background noise
- **Voice Activity Detection**: Automatically detect when speaking
- **Sample Rate**: Audio quality (16000 Hz recommended)

#### Output

- **Copy to Clipboard**: Also copy transcription to clipboard
- **Auto Capitalize**: Capitalize first letter of sentences
- **Output Method**: Type text or clipboard only

#### Advanced

- **GPU Acceleration**: Use GPU for faster transcription (if available)
- **Language**: Transcription language (auto-detect by default)

---

## Keyboard Shortcuts

### Default Hotkeys

| Platform | Default Hotkey | Action |
|----------|----------------|--------|
| Windows | `Ctrl+Shift+Space` | Toggle recording |
| macOS | `Cmd+Shift+R` | Toggle recording |
| Linux | `Ctrl+Shift+R` | Toggle recording |

### Changing the Hotkey

1. Open **Settings**
2. Click **Hotkey** field
3. Press desired key combination
4. Click **Save**

**Recommended combinations:**
- `Ctrl+Alt+V` (V for Voice)
- `Ctrl+Shift+T` (T for Transcribe)
- `Win+Shift+V` (Windows)

**Avoid system shortcuts:**
- `Ctrl+C`, `Ctrl+V` (clipboard)
- `Alt+Tab` (window switcher)
- `Win+L` (lock screen)

---

## Troubleshooting

### Hotkey Not Working

**Windows:**
- Check if another app is using the same hotkey
- Run VoxAI as administrator if needed

**macOS:**
- Go to **System Settings** → **Privacy & Security** → **Accessibility**
- Enable VoxAI

**Linux (Wayland):**
- Wayland restricts global hotkeys
- Use your desktop environment's keyboard settings to manually configure
- Or switch to X11 session

### Text Not Injecting

**macOS:**
- Grant **Accessibility** permissions in **System Settings**

**Linux:**
- Works on X11 (not Wayland)
- Alternatively, enable "Copy to Clipboard" in settings

### Microphone Not Working

- Check system microphone permissions
- Select correct input device in system settings
- Test microphone with another app first

### Poor Transcription Quality

- Use a better microphone
- Enable **Noise Reduction**
- Upgrade to **Small** or **Medium** model
- Speak clearly and at moderate pace

### High CPU Usage

- Use smaller model (**Tiny** or **Base**)
- Enable **GPU Acceleration** if available
- Close other applications

---

## FAQ

### Is my audio sent to the cloud?

**No.** VoxAI processes everything **locally** on your device. No audio is sent to any server.

### What languages are supported?

Whisper AI supports 99+ languages including:
- English, Spanish, French, German, Italian
- Chinese, Japanese, Korean
- Arabic, Hindi, Portuguese, Russian
- And many more

Change language in **Settings** → **Advanced** → **Language**

### Can I use VoxAI for dictation?

Yes! VoxAI is perfect for:
- Dictating documents
- Writing emails
- Taking notes
- Coding (with voice)

### Does VoxAI work offline?

**Yes!** After downloading the Whisper model, VoxAI works completely offline.

### How accurate is the transcription?

Accuracy depends on:
- **Model size**: Larger = more accurate
- **Microphone quality**: Better mic = better results
- **Speech clarity**: Clear speech = better accuracy
- **Background noise**: Quiet environment = better results

Typical accuracy: **85-95%** with Base model in good conditions.

### Can I run VoxAI on startup?

**Windows:**
- Press `Win+R`, type `shell:startup`
- Create shortcut to `voxai.exe` in that folder

**macOS:**
- **System Settings** → **General** → **Login Items**
- Add VoxAI to the list

**Linux:**
- Add to autostart applications in your desktop environment

---

## System Requirements

**Minimum:**
- **CPU**: Intel Core i3 or equivalent
- **RAM**: 2GB available
- **Storage**: 500MB (for app + models)
- **OS**: Windows 10+, macOS 11+, Linux (X11/Wayland)

**Recommended:**
- **CPU**: Intel Core i5/AMD Ryzen 5 or better
- **RAM**: 4GB available
- **GPU**: NVIDIA/AMD/Intel for acceleration
- **Microphone**: Quality USB microphone or headset

---

## Privacy Policy

VoxAI does not:
- Send data to external servers
- Collect telemetry or analytics
- Require account creation
- Access the internet (except for model download)

All transcription happens **100% locally** on your device.

---

## Support

- **Documentation**: https://github.com/theo-stocchetti/voxai/docs
- **Issues**: https://github.com/theo-stocchetti/voxai/issues
- **Discussions**: https://github.com/theo-stocchetti/voxai/discussions

---

**Enjoy using VoxAI!** 🎤✨
