# Conductor v4.0 Rebrand & Strategic Implementation Plan

**Date**: January 2025
**Version**: 1.0
**Status**: Strategic Planning Document

---

## Executive Summary

This document outlines the comprehensive plan to rebrand MIDIMon to Conductor, including:
- Complete codebase refactoring (150-200 files affected)
- Domain strategy across 6 domains
- Monetization infrastructure (Lemon Squeezy integration)
- Distribution partnerships (Raycast extension)
- Big bang v4.0 launch (no backward compatibility)

**Timeline**: 7-10 days of focused work
**Launch Target**: End of Week 2

---

## Strategic Decisions

### Rebrand Rationale

**Why "Conductor"?**
- MIDIMon is MIDI-centric (legacy naming)
- Conductor supports multiple protocols (MIDI + HID + OSC)
- "Conductor" implies orchestration of diverse inputs
- Better domain availability (getconductor.*)

### Domain Strategy

| Domain | Purpose | Launch Priority |
|--------|---------|----------------|
| **getconductor.app** | Main product site (download, features, marketing) | P0 (Critical) |
| **getconductor.studio** | Premium GUI tier ($29 Studio license) | P1 (High) |
| **getconductor.dev** | Developer documentation (mdBook/Docusaurus) | P0 (Critical) |
| **getconductor.pro** | Professional/enterprise tier licensing | P2 (Medium) |
| **getconductor.gg** | Gaming controller focus (B2C mini-site) | P1 (High) |
| **getconductor.bot** | AI features (natural language config, voice control) | P3 (Future) |

### Repository Strategy

**Decision**: Archive `amiable-dev/midimon`, create fresh `amiable-dev/conductor`

**Rationale**:
- Clean brand separation
- No redirect confusion
- Allows MIDIMon to remain as historical reference
- Fresh start for v4.0

**Migration Path**:
1. Tag `amiable-dev/midimon` as `v3.x-final`
2. Archive repo (read-only mode)
3. Pin issue: "MIDIMon has been rebranded to Conductor → [link to new repo]"
4. Create `amiable-dev/conductor` with refactored code
5. Tag as `v4.0.0`

### Backward Compatibility

**Decision**: Clean break, no migration (v4.0 = breaking change)

**Impact**:
- Config paths change: `~/.config/midimon/` → `~/.config/conductor/`
- Binary names change: `midimon` → `conductor`, `midimonctl` → `conductorctl`
- Crate names change: `midimon-core` → `conductor-core`
- Users must manually reconfigure

**Migration Guide**:
```bash
# Manual migration steps
# 1. Export old config
cp ~/.config/midimon/config.toml ~/Desktop/midimon-backup.toml

# 2. Install Conductor v4.0
brew install conductor  # or download from getconductor.app

# 3. Copy config to new location
mkdir -p ~/.config/conductor
cp ~/Desktop/midimon-backup.toml ~/.config/conductor/config.toml

# 4. Update service (macOS)
launchctl unload ~/Library/LaunchAgents/com.amiable.midimon.plist
launchctl load ~/Library/LaunchAgents/com.amiable.conductor.plist
```

### Timeline

**Big bang launch** - all changes at once in v4.0 release

**Phases**:
- Week 1: Codebase refactoring + documentation
- Week 2: Infrastructure + launch prep
- Launch day: Archive old repo, publish new repo, deploy sites

---

## Part 1: Codebase Refactoring

### 1.1 Package Rename (2-3 days)

**Workspace Structure**:

```
Before:                          After:
midimon/                    →    conductor/
├── midimon-core/          →    ├── conductor-core/
├── midimon-daemon/        →    ├── conductor-daemon/
├── midimon-gui/           →    ├── conductor-gui/
└── Cargo.toml                   └── Cargo.toml
```

**Files to Update**: 13 Cargo.toml files

**Critical Changes**:

```toml
# Root Cargo.toml
[package]
name = "conductor"  # was: midimon
repository = "https://github.com/amiable-dev/conductor"
authors = ["Conductor Contributors"]

[workspace]
members = [
    "conductor-core",      # was: midimon-core
    "conductor-daemon",    # was: midimon-daemon
    "conductor-gui",       # was: midimon-gui
]

# conductor-core/Cargo.toml
[package]
name = "conductor-core"  # was: midimon-core
description = "Multi-protocol input mapping engine"  # was: MIDI mapping

[lib]
name = "conductor_core"  # was: midimon_core

# conductor-daemon/Cargo.toml
[[bin]]
name = "conductor"  # was: midimon

[[bin]]
name = "conductorctl"  # was: midimonctl
```

**Plugin Packages** (8 files):
- `plugins/midimon-*` → `plugins/conductor-*`
- Update all plugin Cargo.toml files

### 1.2 Rust Source Code (1-2 days)

**Application-Level Renames** (MUST change):

```rust
// engine.rs
pub struct ConductorEngine {  // was: MidiMonEngine
    // ...
}

// Module headers
//! Conductor Core Engine           // was: MIDIMon Core
//! Multi-protocol input mapping     // was: Pure Rust MIDI mapping

// Import paths (across all files)
use conductor_core::*;              // was: use midimon_core::*;
extern crate conductor;             // was: extern crate midimon;
```

**Protocol-Specific Names** (KEEP unchanged):

These remain unchanged because they're protocol-specific:
```rust
// MIDI protocol types (keep as-is)
pub enum MidiEvent { /* ... */ }
pub struct MidiFeedback { /* ... */ }
pub struct MidiDeviceManager { /* ... */ }

// HID protocol types (keep as-is)
pub struct HidDeviceManager { /* ... */ }
pub enum GamepadEvent { /* ... */ }
```

**Rationale**: Conductor supports multiple protocols. MIDI-specific types should retain "Midi" prefix for clarity.

### 1.3 Binary & Tool Names (1 day)

**CLI Tools**:

```bash
# Main binaries
midimon → conductor
midimonctl → conductorctl
midimon-gui → conductor-gui

# Diagnostic tools (protocol-specific, can keep names)
midi_diagnostic  # Keep (MIDI-specific)
test_midi        # Keep (MIDI-specific)
led_diagnostic   # Keep (device-agnostic)
```

**Man Pages**:
```bash
# /usr/local/share/man/man1/
midimon.1 → conductor.1
midimonctl.1 → conductorctl.1
```

### 1.4 System Integration (1 day)

**macOS LaunchAgent**:

File: `com.amiable.conductor.plist` (was: `com.amiable.midimon.plist`)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.amiable.conductor</string>

    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/conductor</string>
        <string>daemon</string>
    </array>

    <key>StandardOutPath</key>
    <string>/Users/USERNAME/Library/Logs/conductor.log</string>

    <key>StandardErrorPath</key>
    <string>/Users/USERNAME/Library/Logs/conductor-error.log</string>

    <key>EnvironmentVariables</key>
    <dict>
        <key>RUST_LOG</key>
        <string>conductor=info</string>
    </dict>
</dict>
</plist>
```

**systemd Service** (Linux):

File: `conductor.service` (was: `midimon.service`)

```ini
[Unit]
Description=Conductor Multi-Protocol Input Mapping Daemon
Documentation=man:conductor(1) https://getconductor.dev
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/conductor daemon
Restart=on-failure
RestartSec=5s
SyslogIdentifier=conductor
Environment="RUST_LOG=conductor=info"

[Install]
WantedBy=default.target
```

### 1.5 Configuration Paths

**Directory Structure**:

```bash
# macOS/Linux
~/.config/midimon/        →  ~/.config/conductor/
~/.local/state/midimon/   →  ~/.local/state/conductor/
~/Library/Logs/midimon.log → ~/Library/Logs/conductor.log

# Windows
%APPDATA%\midimon\        →  %APPDATA%\conductor\
```

**No automatic migration** (per strategy) - users manually copy configs.

---

## Part 2: Documentation Updates

### 2.1 Root Documentation (1 day)

**Files**: 10+ markdown files

**README.md** - Complete rebrand:

```markdown
# Conductor

> Multi-protocol input automation for MIDI controllers, game controllers, and custom hardware

## What is Conductor?

Conductor transforms MIDI controllers, game controllers (Xbox, PlayStation, Switch),
and custom HID devices into advanced productivity automation tools with:

- **Velocity sensitivity**: Soft press = one action, hard press = another
- **Advanced triggers**: Long-press, double-tap, chords, encoders
- **Multi-protocol**: MIDI + HID gamepads + (coming: OSC)
- **Visual designer**: Drag-and-drop configuration GUI
- **Marketplace**: 1000+ community templates

**v4.0 Features**:
- 🎮 Full gamepad support (Xbox, PlayStation, Switch Pro)
- 🎹 MIDI controller support with RGB LED feedback
- 🤖 AI-powered natural language config generation
- 🔥 Hot-reload (0-10ms config changes)
- 🚀 <1ms latency, 5-10MB memory footprint

[Download](https://getconductor.app) | [Docs](https://getconductor.dev) | [Gaming](https://getconductor.gg)
```

**Other Files**:
- `CLAUDE.md` - Update project instructions (50+ references)
- `CONTRIBUTING.md` - Update contribution guidelines
- `DEPLOYMENT.md` - Update deployment instructions
- `DEVELOPMENT.md` - Update dev setup
- `ROADMAP.md` - Rebrand, update strategic vision
- `SECURITY.md` - Update security policy
- `CODE_OF_CONDUCT.md` - Update contact info

### 2.2 Documentation Site (2 days)

**Location**: `docs-site/src/` (60+ files)

**High-Priority Pages**:

1. **introduction.md** - Already updated with v3.0 content, rebrand hero copy
2. **getting-started/quick-start.md** - Update download URLs, binary names
3. **installation/*.md** (3 files) - Update Homebrew formula, package names
4. **configuration/examples.md** - Already has hybrid MIDI+gamepad examples
5. **reference/cli-commands.md** - Update `midimon` → `conductor`, `midimonctl` → `conductorctl`

**Example Page Updates**:

```markdown
<!-- Before -->
# Installing MIDIMon on macOS

brew install amiable-dev/tap/midimon
midimon --version

<!-- After -->
# Installing Conductor on macOS

brew install amiable-dev/tap/conductor
conductor --version

# Or download from getconductor.app
```

**Search & Replace**:
- "MIDIMon" → "Conductor" (case-sensitive)
- "midimon" → "conductor" (CLI commands)
- "MIDI controller mapping" → "Multi-protocol input automation"
- GitHub URLs updated

### 2.3 Strategic Documentation

**Location**: `docs/`

**Files**:
- `strategic-assessment-2025.md` - Update v1.0 with Conductor branding
- `strategic-assessment-2025-ai-enhanced.md` - Update v2.0 with Conductor branding
- Architecture diagrams (if any SVGs/PNGs need updating)

---

## Part 3: GUI & User-Facing Strings

### 3.1 Tauri Configuration (2 hours)

**File**: `conductor-gui/src-tauri/tauri.conf.json`

```json
{
  "productName": "Conductor",
  "version": "4.0.0",
  "identifier": "com.amiable.conductor",
  "bundle": {
    "identifier": "com.amiable.conductor",
    "icon": [
      "icons/conductor-32x32.png",
      "icons/conductor-128x128.png",
      "icons/conductor-256x256.png"
    ],
    "shortDescription": "Multi-Protocol Input Mapping System",
    "longDescription": "Transform MIDI controllers, game controllers, and custom hardware into advanced automation tools for productivity, music production, streaming, and more.",
    "macOS": {
      "entitlements": "entitlements.plist",
      "exceptionDomain": "getconductor.app",
      "frameworks": [],
      "minimumSystemVersion": "11.0"
    },
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  },
  "app": {
    "windows": [
      {
        "title": "Conductor Configuration",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ]
  }
}
```

### 3.2 UI Package (1 hour)

**File**: `conductor-gui/ui/package.json`

```json
{
  "name": "conductor-gui-ui",
  "version": "4.0.0",
  "description": "Conductor Configuration UI",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.0.0",
    "svelte": "^4.0.0"
  }
}
```

**File**: `conductor-gui/ui/index.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Conductor</title>
    <link rel="icon" href="/favicon.ico" />
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

### 3.3 Svelte Components (2-3 hours)

**Search in all `.svelte` files**:

```bash
# Find all user-facing strings
grep -r "MIDIMon" conductor-gui/ui/src/lib/components/
grep -r "MIDI Mon" conductor-gui/ui/src/lib/components/

# Likely locations:
# - StatusBar.svelte (app name in status)
# - SettingsPanel.svelte (settings labels)
# - AboutDialog.svelte (about box)
# - ErrorBoundary.svelte (error messages)
```

**Replace pattern**:
- Window titles: "MIDIMon Configuration" → "Conductor"
- Error messages: "MIDIMon failed to..." → "Conductor failed to..."
- Help text: "Learn more about MIDIMon" → "Learn more about Conductor"

---

## Part 4: Repository & Infrastructure

### 4.1 GitHub Repository Setup (2 hours)

**Step 1: Archive Old Repo**

```bash
# On GitHub web interface:
# 1. Navigate to amiable-dev/midimon
# 2. Settings → Archive this repository
# 3. Confirm archive

# Create final release
git tag -a v3.x-final -m "Final MIDIMon release before Conductor rebrand"
git push origin v3.x-final

# Create pinned issue
Title: "MIDIMon has been rebranded to Conductor"
Body:
"""
🎉 **MIDIMon is now Conductor!**

This repository has been archived. All future development happens at:
👉 **https://github.com/amiable-dev/conductor**

## What changed?
- Name: MIDIMon → Conductor
- Focus: MIDI-only → Multi-protocol (MIDI + HID + OSC)
- Version: v3.x → v4.0 (breaking change)

## Migration Guide
See: https://getconductor.dev/migration-from-midimon

## Why the rebrand?
Conductor now supports game controllers, MIDI devices, and more. The new name
reflects this multi-protocol future.

Thank you for your support! See you at the new repo 🚀
"""
```

**Step 2: Create New Repo**

```bash
# Create repo on GitHub: amiable-dev/conductor
# Description: "Multi-protocol input automation for MIDI controllers, game controllers, and custom hardware"
# Topics: rust, midi, gamepad, automation, productivity, tauri, controller-mapping

# Initialize locally
cd /path/to/refactored/codebase
git init
git remote add origin git@github.com:amiable-dev/conductor.git

# Initial commit
git add .
git commit -m "feat: Conductor v4.0.0 - rebrand from MIDIMon

BREAKING CHANGE: Complete rebrand from MIDIMon to Conductor
- Package names: midimon-* → conductor-*
- Binary names: midimon → conductor, midimonctl → conductorctl
- Config paths: ~/.config/midimon/ → ~/.config/conductor/
- Repository: amiable-dev/midimon → amiable-dev/conductor

This is a clean-break release with no backward compatibility.
Users must manually migrate configs."

git tag -a v4.0.0 -m "Conductor v4.0.0 - Multi-Protocol Input Automation"
git push -u origin main
git push origin v4.0.0
```

### 4.2 GitHub Metadata (1 hour)

**Issue Templates** (`.github/ISSUE_TEMPLATE/`):

Update all 5 templates:
- `bug_report.yml` - Replace "MIDIMon" with "Conductor"
- `feature_request.yml` - Update product name
- `device_support.yml` - Update branding
- `documentation.yml` - Update doc site URL (getconductor.dev)
- `config.yml` - Update branding

**CI/CD Workflows** (`.github/workflows/`):

```yaml
# deploy-docs.yml
name: Deploy Documentation
on:
  push:
    branches: [main]
    paths: ['docs-site/**']

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build docs
        run: |
          cd docs-site
          mdbook build
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs-site/book
          cname: getconductor.dev  # Custom domain
```

**Badges** (README.md):

```markdown
[![Build Status](https://github.com/amiable-dev/conductor/workflows/CI/badge.svg)](https://github.com/amiable-dev/conductor/actions)
[![Documentation](https://img.shields.io/badge/docs-getconductor.dev-blue)](https://getconductor.dev)
[![Crates.io](https://img.shields.io/crates/v/conductor-core)](https://crates.io/crates/conductor-core)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
```

### 4.3 Crates.io Publication (1 hour)

**Important**: Publish to prevent name squatting

```bash
# Publish in order (dependencies first)
cd conductor-core
cargo publish

cd ../conductor-daemon
cargo publish

cd ../conductor-gui/src-tauri
cargo publish

# Publish root crate last
cd ../../../
cargo publish
```

**Crate Descriptions**:

```toml
# conductor-core/Cargo.toml
[package]
description = "Multi-protocol input mapping engine for Conductor"
keywords = ["midi", "gamepad", "hid", "automation", "controller"]
categories = ["hardware-support", "multimedia", "command-line-utilities"]
```

---

## Part 5: Domain & Deployment

### 5.1 DNS Configuration (1 hour)

**Domain Provider**: (Assuming Cloudflare, Namecheap, or similar)

**DNS Records**:

```
# Primary site (marketing)
getconductor.app
  A     76.76.21.21          (Vercel IP)
  AAAA  2606:4700::...       (Vercel IPv6)
  CNAME www → getconductor.app

# Documentation
getconductor.dev
  CNAME → amiable-dev.github.io

# Premium tier
getconductor.studio
  CNAME → getconductor.app (for now, redirect)

# Professional tier
getconductor.pro
  CNAME → getconductor.app (for now, redirect)

# Gaming focus
getconductor.gg
  A     76.76.21.22          (Separate Vercel project)

# AI features (future)
getconductor.bot
  CNAME → getconductor.app (redirect for now)
```

**SSL Certificates**: Auto-provisioned by Vercel/GitHub Pages

### 5.2 Documentation Site (getconductor.dev) (2 hours)

**Deploy mdBook to GitHub Pages**:

```bash
# Configure custom domain
echo "getconductor.dev" > docs-site/book/CNAME

# GitHub Pages settings:
# - Source: gh-pages branch
# - Custom domain: getconductor.dev
# - Enforce HTTPS: enabled

# Build and deploy (via GitHub Actions - already configured)
git push origin main  # Triggers deploy-docs.yml workflow
```

**Site Structure**:

```
https://getconductor.dev/
├── introduction.html
├── getting-started/
│   ├── quick-start.html
│   └── installation.html
├── configuration/
│   ├── triggers.html
│   └── actions.html
├── devices/
│   └── compatibility.html
└── reference/
    └── cli.html
```

### 5.3 Main Site (getconductor.app) (3-4 hours)

**Landing Page** (Vercel deployment):

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Conductor - Multi-Protocol Input Automation</title>
    <meta name="description" content="Transform MIDI controllers and game controllers into productivity tools">

    <!-- Open Graph -->
    <meta property="og:title" content="Conductor - Multi-Protocol Input Automation">
    <meta property="og:description" content="Turn any controller into a productivity powerhouse">
    <meta property="og:image" content="https://getconductor.app/og-image.png">
    <meta property="og:url" content="https://getconductor.app">

    <!-- Twitter Card -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="Conductor">
    <meta name="twitter:description" content="Multi-protocol input automation for MIDI and game controllers">
    <meta name="twitter:image" content="https://getconductor.app/twitter-card.png">
</head>
<body>
    <header>
        <nav>
            <a href="https://getconductor.app">Conductor</a>
            <a href="https://getconductor.dev">Docs</a>
            <a href="https://getconductor.studio">Studio</a>
            <a href="https://getconductor.gg">Gaming</a>
            <a href="https://github.com/amiable-dev/conductor">GitHub</a>
        </nav>
    </header>

    <main>
        <section class="hero">
            <h1>Turn Any Controller Into a Productivity Powerhouse</h1>
            <p>Multi-protocol input automation for MIDI controllers, game controllers, and custom hardware</p>

            <div class="cta">
                <a href="#download" class="btn-primary">Download v4.0</a>
                <a href="https://getconductor.dev" class="btn-secondary">Read Docs</a>
            </div>
        </section>

        <section class="features">
            <h2>Why Conductor?</h2>

            <div class="feature">
                <h3>🎮 Multi-Protocol</h3>
                <p>MIDI controllers, Xbox/PlayStation gamepads, custom HID devices</p>
            </div>

            <div class="feature">
                <h3>⚡ Velocity Sensitive</h3>
                <p>Soft press = copy, hard press = paste. One button, multiple actions.</p>
            </div>

            <div class="feature">
                <h3>🔥 Hot-Reload</h3>
                <p>Config changes apply in 0-10ms without restart</p>
            </div>

            <div class="feature">
                <h3>🤖 AI-Powered</h3>
                <p>Natural language config: "Map A button to run cargo build"</p>
            </div>
        </section>

        <section id="download" class="download">
            <h2>Download Conductor v4.0</h2>

            <div class="platforms">
                <a href="https://github.com/amiable-dev/conductor/releases/download/v4.0.0/Conductor-macos-universal.dmg" class="platform">
                    <h3>macOS</h3>
                    <p>11+ (Big Sur)</p>
                    <p>Universal (Intel + Apple Silicon)</p>
                </a>

                <a href="https://github.com/amiable-dev/conductor/releases/download/v4.0.0/Conductor-windows-x64.exe" class="platform">
                    <h3>Windows</h3>
                    <p>10+ (64-bit)</p>
                    <p>Installer + Portable</p>
                </a>

                <a href="https://github.com/amiable-dev/conductor/releases/download/v4.0.0/conductor-linux-x86_64.tar.gz" class="platform">
                    <h3>Linux</h3>
                    <p>Ubuntu 20.04+</p>
                    <p>.deb / .tar.gz</p>
                </a>
            </div>

            <p class="homebrew">
                macOS Homebrew: <code>brew install amiable-dev/tap/conductor</code>
            </p>
        </section>

        <section class="pricing">
            <h2>Pricing</h2>

            <div class="tiers">
                <div class="tier">
                    <h3>Free</h3>
                    <p>$0</p>
                    <ul>
                        <li>Unlimited mappings</li>
                        <li>MIDI + HID support</li>
                        <li>10 AI generations/month</li>
                        <li>Local configs only</li>
                    </ul>
                    <a href="#download">Download</a>
                </div>

                <div class="tier featured">
                    <h3>Studio</h3>
                    <p>$29 one-time</p>
                    <ul>
                        <li>Visual designer</li>
                        <li>100 AI generations/month</li>
                        <li>Marketplace access</li>
                        <li>Premium templates</li>
                    </ul>
                    <a href="https://getconductor.studio">Buy Now</a>
                </div>

                <div class="tier">
                    <h3>Cloud</h3>
                    <p>$25/year</p>
                    <ul>
                        <li>Everything in Studio</li>
                        <li>Unlimited AI</li>
                        <li>Cloud sync</li>
                        <li>Voice control</li>
                    </ul>
                    <a href="https://getconductor.studio">Subscribe</a>
                </div>
            </div>
        </section>
    </main>

    <footer>
        <p>&copy; 2025 Amiable. Licensed under MIT.</p>
        <a href="https://github.com/amiable-dev/conductor">GitHub</a>
        <a href="https://getconductor.dev">Docs</a>
        <a href="mailto:hello@amiable.dev">Contact</a>
    </footer>
</body>
</html>
```

**Deploy to Vercel**:
```bash
vercel --prod
vercel alias getconductor.app
```

### 5.4 Gaming Site (getconductor.gg) (2 hours)

**Mini-Site for Gamers** (Separate Vercel project):

```html
<!-- B2C landing page targeting gamers -->
<header>
    <h1>🎮 Turn Your Xbox Controller Into a Macro Pad</h1>
    <p>Professional streaming controls for the price of a gamepad</p>
</header>

<section class="comparison">
    <h2>Why Buy a Stream Deck?</h2>

    <div class="vs">
        <div class="option">
            <h3>Stream Deck</h3>
            <p class="price">$150-$300</p>
            <ul>
                <li>15 buttons</li>
                <li>Wired only</li>
                <li>Desktop use only</li>
            </ul>
        </div>

        <div class="option highlight">
            <h3>Your Xbox Controller + Conductor</h3>
            <p class="price">$30 (controller) + $0 (Conductor is free!)</p>
            <ul>
                <li>15+ buttons</li>
                <li>Wireless</li>
                <li>Ergonomic</li>
                <li>Velocity-sensitive</li>
            </ul>
        </div>
    </div>

    <p class="savings">💰 Save $270!</p>
</section>

<section class="templates">
    <h2>Gaming Templates</h2>

    <div class="template">
        <h3>OBS Streaming</h3>
        <p>Start/stop recording, switch scenes, mute mic</p>
        <a href="#">Download Template</a>
    </div>

    <div class="template">
        <h3>Discord Control</h3>
        <p>Mute/unmute, deafen, push-to-talk</p>
        <a href="#">Download Template</a>
    </div>

    <div class="template">
        <h3>Racing Wheel Editing</h3>
        <p>Use pedals for timeline speed, wheel for scrubbing</p>
        <a href="#">Download Template</a>
    </div>
</section>

<section class="testimonials">
    <h2>Success Stories</h2>

    <blockquote>
        "I was about to buy a Stream Deck for $300 when I found Conductor.
        Now my old Xbox controller handles all my stream controls. Saved me $270!"
        <cite>- Sarah, Twitch Streamer</cite>
    </blockquote>
</section>

<section class="cta">
    <a href="https://getconductor.app#download" class="btn-large">Download Free</a>
    <p>Or install: <code>brew install conductor</code></p>
</section>
```

### 5.5 Binary Distribution (2 hours)

**Homebrew Tap** (macOS):

Create: `homebrew-conductor/conductor.rb`

```ruby
class Conductor < Formula
  desc "Multi-protocol input mapping system for MIDI and game controllers"
  homepage "https://getconductor.app"
  url "https://github.com/amiable-dev/conductor/releases/download/v4.0.0/conductor-v4.0.0-macos-universal.tar.gz"
  sha256 "SHA256_HASH_HERE"
  license "MIT"
  version "4.0.0"

  def install
    bin.install "conductor"
    bin.install "conductorctl"
    man1.install "man/conductor.1"
    man1.install "man/conductorctl.1"
  end

  service do
    run [opt_bin/"conductor", "daemon"]
    keep_alive true
    log_path var/"log/conductor.log"
    error_log_path var/"log/conductor-error.log"
  end

  test do
    system "#{bin}/conductor", "--version"
  end
end
```

**Installation**:
```bash
brew tap amiable-dev/conductor
brew install conductor
```

**Other Package Managers**:

```bash
# Debian/Ubuntu (.deb)
# Create package with:
cargo deb

# Arch AUR (PKGBUILD)
# Submit to AUR: conductor-bin

# Scoop (Windows)
# Create conductor.json manifest
# Submit PR to scoop-extras bucket

# Winget (Windows)
# Submit to winget-pkgs repo
```

---

## Part 6: Monetization Infrastructure

### 6.1 Lemon Squeezy Integration (1-2 days)

**Why Lemon Squeezy?** (not OpenSaaS.sh)

Based on research:
- OpenSaaS.sh requires Node.js (incompatible with Rust/Tauri)
- Lemon Squeezy designed for software licensing (not just SaaS)
- All-in pricing: 5% of revenue (includes payment processing, VAT, fraud)
- No hosting costs, no database costs
- Better for desktop software than Stripe

**Setup**:

1. Create Lemon Squeezy account at lemonsqueezy.com
2. Create store: "Conductor by Amiable"
3. Create products:
   - Conductor Studio ($29 one-time)
   - Conductor Cloud ($25/year subscription)

**Rust Integration**:

```rust
// conductor-core/src/licensing.rs
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub license_key: String,
    pub license_type: LicenseTier,
    pub valid: bool,
    pub activation_limit: i32,
    pub activations_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LicenseTier {
    Free,
    Studio,   // $29 one-time
    Cloud,    // $25/year
}

pub struct LicenseManager {
    api_key: String,
    client: Client,
}

impl LicenseManager {
    pub async fn activate_license(
        &self,
        license_key: &str,
    ) -> Result<LicenseInfo, LicenseError> {
        let response = self.client
            .post("https://api.lemonsqueezy.com/v1/licenses/activate")
            .json(&serde_json::json!({
                "license_key": license_key,
                "instance_name": self.get_machine_id()?
            }))
            .send()
            .await?;

        let info: LicenseInfo = response.json().await?;

        // Store license locally
        self.store_license(&info)?;

        Ok(info)
    }

    pub async fn validate_license(&self) -> Result<bool, LicenseError> {
        let stored = self.load_stored_license()?;

        let response = self.client
            .post("https://api.lemonsqueezy.com/v1/licenses/validate")
            .json(&serde_json::json!({
                "license_key": stored.license_key,
                "instance_id": self.get_machine_id()?
            }))
            .send()
            .await?;

        let result: ValidationResult = response.json().await?;
        Ok(result.valid)
    }

    fn get_machine_id(&self) -> Result<String, LicenseError> {
        // Generate unique machine ID (CPU ID + hostname hash)
        use sysinfo::{System, SystemExt};
        let sys = System::new_all();
        let cpu_id = sys.cpus().first().unwrap().brand();
        let hostname = sys.host_name().unwrap_or_default();

        let combined = format!("{}-{}", cpu_id, hostname);
        let hash = sha256::digest(combined);
        Ok(hash[..16].to_string())
    }

    fn store_license(&self, info: &LicenseInfo) -> Result<(), LicenseError> {
        let config_dir = dirs::config_dir()
            .ok_or(LicenseError::ConfigDirNotFound)?
            .join("conductor");

        std::fs::create_dir_all(&config_dir)?;

        let license_file = config_dir.join("license.json");
        let json = serde_json::to_string_pretty(info)?;
        std::fs::write(license_file, json)?;

        Ok(())
    }
}
```

**Tauri Commands** (for GUI):

```rust
// conductor-gui/src-tauri/src/lib.rs
#[tauri::command]
async fn activate_license(
    license_key: String,
    state: tauri::State<'_, AppState>
) -> Result<LicenseInfo, String> {
    state.license_manager
        .activate_license(&license_key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_license_status(
    state: tauri::State<'_, AppState>
) -> Result<LicenseInfo, String> {
    state.license_manager
        .get_current_license()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn open_checkout(tier: String) -> Result<(), String> {
    let url = match tier.as_str() {
        "studio" => "https://getconductor.lemonsqueezy.com/checkout/buy/studio",
        "cloud" => "https://getconductor.lemonsqueezy.com/checkout/buy/cloud",
        _ => return Err("Invalid tier".into()),
    };

    tauri::api::shell::open(url, None).map_err(|e| e.to_string())
}
```

**GUI License Activation**:

```svelte
<!-- LicenseActivation.svelte -->
<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';

let licenseKey = '';
let activating = false;
let error = '';

async function activate() {
    activating = true;
    error = '';

    try {
        const info = await invoke('activate_license', { licenseKey });
        alert(`License activated! You now have ${info.license_type} tier.`);
        window.location.reload();
    } catch (e) {
        error = e;
    } finally {
        activating = false;
    }
}

async function openCheckout(tier: string) {
    await invoke('open_checkout', { tier });
}
</script>

<div class="license-activation">
    <h2>Activate License</h2>

    <div class="tiers">
        <div class="tier">
            <h3>Free</h3>
            <p>$0</p>
            <ul>
                <li>10 AI generations/month</li>
                <li>Local configs only</li>
            </ul>
        </div>

        <div class="tier">
            <h3>Studio</h3>
            <p>$29 one-time</p>
            <ul>
                <li>100 AI generations/month</li>
                <li>Visual designer</li>
                <li>Marketplace access</li>
            </ul>
            <button on:click={() => openCheckout('studio')}>Buy Now</button>
        </div>

        <div class="tier">
            <h3>Cloud</h3>
            <p>$25/year</p>
            <ul>
                <li>Unlimited AI</li>
                <li>Cloud sync</li>
                <li>Voice control</li>
            </ul>
            <button on:click={() => openCheckout('cloud')}>Subscribe</button>
        </div>
    </div>

    <div class="activation-form">
        <h3>Already purchased? Enter your license key:</h3>
        <input
            type="text"
            bind:value={licenseKey}
            placeholder="XXXX-XXXX-XXXX-XXXX"
        />
        <button on:click={activate} disabled={activating}>
            {activating ? 'Activating...' : 'Activate'}
        </button>
        {#if error}
            <p class="error">{error}</p>
        {/if}
    </div>
</div>
```

**Feature Gating**:

```rust
// conductor-core/src/features.rs
pub struct FeatureGate {
    license: LicenseTier,
}

impl FeatureGate {
    pub fn can_use_ai_config(&self) -> Result<(), FeatureError> {
        match self.license {
            LicenseTier::Free => {
                let usage = self.get_monthly_ai_usage();
                if usage >= 10 {
                    Err(FeatureError::Upgrade {
                        feature: "AI config generation",
                        message: "You've used 10/10 AI generations this month. Upgrade to Studio ($29) for 100/month.".into(),
                        upgrade_url: "https://getconductor.studio".into(),
                    })
                } else {
                    Ok(())
                }
            }
            LicenseTier::Studio | LicenseTier::Cloud => Ok(()),
        }
    }

    pub fn can_access_marketplace(&self) -> Result<(), FeatureError> {
        match self.license {
            LicenseTier::Free => Err(FeatureError::Upgrade {
                feature: "Marketplace",
                message: "Marketplace requires Studio license ($29)".into(),
                upgrade_url: "https://getconductor.studio".into(),
            }),
            LicenseTier::Studio | LicenseTier::Cloud => Ok(()),
        }
    }

    pub fn can_use_cloud_sync(&self) -> Result<(), FeatureError> {
        match self.license {
            LicenseTier::Free | LicenseTier::Studio => Err(FeatureError::Upgrade {
                feature: "Cloud sync",
                message: "Cloud sync requires Cloud subscription ($25/year)".into(),
                upgrade_url: "https://getconductor.studio".into(),
            }),
            LicenseTier::Cloud => Ok(()),
        }
    }
}
```

**Cost Analysis**:

```
Year 1 Projections (10,000 users):

Studio licenses:
  8,000 × $29 = $232,000
  - Lemon Squeezy fee (5%): -$11,600
  Net: $220,400

Cloud subscriptions:
  1,600 × $25 = $40,000
  - Lemon Squeezy fee (5%): -$2,000
  Net: $38,000

Total Year 1 Revenue: $258,400
Total Platform Fees: $13,600
Net Revenue: $244,800

Compare to OpenSaaS approach:
  Stripe fees (2.9%): $7,888
  Hosting (Node.js): $420/year
  Total fees: $8,308

Lemon Squeezy is $5,292 more expensive BUT:
- No engineering time building payment infrastructure
- No server maintenance
- Built-in VAT handling (EU sales)
- License validation API included
- Better for desktop software
```

### 6.2 Marketplace Revenue Share (1 day)

**Architecture**:

```rust
// Marketplace template pricing
pub struct MarketplaceTemplate {
    pub id: String,
    pub name: String,
    pub author: String,
    pub price: Decimal,  // e.g., $10.00
    pub revenue_split: RevenueSplit,
}

pub struct RevenueSplit {
    pub platform_fee: Decimal,  // 5% (Lemon Squeezy)
    pub conductor_take: Decimal,  // 30%
    pub creator_receive: Decimal,  // 65%
}

// Example: $10 template
// Platform fee: $0.50 (5%)
// Conductor: $3.00 (30%)
// Creator: $6.50 (65%)
```

**Implementation**: Use Lemon Squeezy's affiliate system
- Each template creator gets affiliate link
- Lemon Squeezy tracks sales
- Auto-pays creators monthly
- Conductor keeps 30% via affiliate commission

---

## Part 7: Raycast Integration

### 7.1 "Conductor for Raycast" Extension (1-2 days)

**Why Build This?**

Based on research:
- Raycast has 100K+ active users (high-intent developer audience)
- Complementary, not competitive (keyboard launcher vs hardware automation)
- Free distribution via Raycast Store
- Low engineering effort (8-16 hours)

**Extension Structure**:

```typescript
// raycast-conductor/package.json
{
  "name": "conductor",
  "title": "Conductor",
  "description": "Control Conductor daemon from Raycast",
  "icon": "icon.png",
  "author": "amiable",
  "license": "MIT",
  "commands": [
    {
      "name": "reload-config",
      "title": "Reload Conductor Config",
      "subtitle": "Hot-reload config.toml",
      "mode": "no-view"
    },
    {
      "name": "switch-mode",
      "title": "Switch Mode",
      "subtitle": "Switch between mapping modes",
      "mode": "view"
    },
    {
      "name": "daemon-status",
      "title": "Daemon Status",
      "subtitle": "View Conductor daemon info",
      "mode": "view"
    },
    {
      "name": "midi-learn",
      "title": "Start MIDI Learn",
      "subtitle": "Enter MIDI Learn mode",
      "mode": "no-view"
    }
  ],
  "dependencies": {
    "@raycast/api": "^1.70.0"
  }
}
```

**Implementation**:

```typescript
// reload-config.tsx
import { showToast, Toast } from "@raycast/api";
import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

export default async function ReloadConfig() {
  try {
    const toast = await showToast({
      style: Toast.Style.Animated,
      title: "Reloading config...",
    });

    const { stdout, stderr } = await execAsync("conductorctl reload");

    if (stderr) {
      toast.style = Toast.Style.Failure;
      toast.title = "Failed to reload";
      toast.message = stderr;
    } else {
      toast.style = Toast.Style.Success;
      toast.title = "Config reloaded";
      toast.message = "Config applied in <10ms";
    }
  } catch (error) {
    await showToast({
      style: Toast.Style.Failure,
      title: "Error",
      message: error instanceof Error ? error.message : "Unknown error",
    });
  }
}
```

```typescript
// switch-mode.tsx
import { ActionPanel, List, Action, showToast, Toast } from "@raycast/api";
import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

interface Mode {
  name: string;
  color: string;
  active: boolean;
}

export default function SwitchMode() {
  const modes = getModes();

  return (
    <List>
      {modes.map((mode) => (
        <List.Item
          key={mode.name}
          title={mode.name}
          subtitle={mode.active ? "Active" : ""}
          icon={{ source: mode.active ? "✓" : "○" }}
          accessories={[{ text: mode.color }]}
          actions={
            <ActionPanel>
              <Action
                title="Switch to This Mode"
                onAction={async () => {
                  try {
                    await execAsync(`conductorctl switch-mode ${mode.name}`);
                    await showToast({
                      style: Toast.Style.Success,
                      title: `Switched to ${mode.name} mode`,
                    });
                  } catch (error) {
                    await showToast({
                      style: Toast.Style.Failure,
                      title: "Failed to switch mode",
                      message: error instanceof Error ? error.message : "Unknown error",
                    });
                  }
                }}
              />
            </ActionPanel>
          }
        />
      ))}
    </List>
  );
}

function getModes(): Mode[] {
  // Parse ~/.config/conductor/config.toml
  // Extract mode names
  // Detect active mode from daemon status
  return [
    { name: "Default", color: "blue", active: true },
    { name: "Development", color: "green", active: false },
    { name: "Media", color: "purple", active: false },
  ];
}
```

```typescript
// daemon-status.tsx
import { Detail } from "@raycast/api";
import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

export default function DaemonStatus() {
  const [status, setStatus] = React.useState<string>("Loading...");

  React.useEffect(() => {
    (async () => {
      try {
        const { stdout } = await execAsync("conductorctl status");
        setStatus(stdout);
      } catch (error) {
        setStatus(`Error: ${error.message}`);
      }
    })();
  }, []);

  return (
    <Detail
      markdown={`
# Conductor Daemon Status

\`\`\`
${status}
\`\`\`

## Quick Actions
- ⌘+R: Reload config
- ⌘+M: Switch mode
- ⌘+L: Start MIDI Learn
      `}
    />
  );
}
```

**Distribution**:

```bash
# Submit to Raycast Store
# 1. Fork raycast/extensions
# 2. Add conductor/ directory
# 3. Open PR
# 4. Raycast reviews (~1 week)
# 5. Merged to store

git clone https://github.com/raycast/extensions.git
cd extensions/extensions
mkdir conductor
cp -r ~/raycast-conductor/* conductor/
git add conductor
git commit -m "Add Conductor extension"
git push origin main
# Open PR on GitHub
```

**Marketing Value**:

```
Conservative estimate:
  100,000 Raycast users
  × 5% interested in hardware automation = 5,000
  × 20% try free version = 1,000 new users
  × 30% convert to Studio = 300 × $29 = $8,700

ROI:
  Engineering: 16 hours
  Revenue potential: $8,700+
  Hourly rate: $544/hour
```

### 7.2 Bidirectional Integration (2 hours)

**Conductor → Raycast** (trigger Raycast from hardware):

```rust
// conductor-core/src/actions.rs
pub enum Action {
    // ... existing actions
    OpenRaycast { query: Option<String> },
}

impl ActionExecutor {
    fn execute_open_raycast(&self, query: Option<&str>) -> Result<()> {
        let url = if let Some(q) = query {
            format!("raycast://extensions/search?query={}", urlencoding::encode(q))
        } else {
            "raycast://extensions/search".to_string()
        };

        Command::new("open")
            .arg(&url)
            .spawn()?;

        Ok(())
    }
}
```

**Config Example**:

```toml
# Press gamepad A button → opens Raycast with "git" search
[[modes.mappings]]
description = "Quick Git Commands via Raycast"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # A button

[modes.mappings.action]
type = "OpenRaycast"
query = "git"
```

**Use Case**: Hardware button → opens Raycast with pre-filled query

---

## Part 8: Marketing & Launch

### 8.1 Launch Announcement (1 day)

**Blog Post**: "Introducing Conductor: Beyond MIDI"

```markdown
# Introducing Conductor: Multi-Protocol Input Automation

**TL;DR**: MIDIMon has been rebranded to Conductor and now supports game controllers alongside MIDI devices.

## Why the Rebrand?

When we started MIDIMon in 2024, the focus was clear: turn MIDI controllers into productivity tools. But as we added gamepad support in v3.0, the name no longer reflected the product's capabilities.

**Conductor** orchestrates inputs from:
- 🎹 MIDI controllers (Maschine, Launchpad, APC Mini)
- 🎮 Game controllers (Xbox, PlayStation, Switch Pro)
- 🕹️ Custom HID devices (racing wheels, flight sticks, HOTAS)
- 🔮 Future: OSC, keyboard intercept, and more

## What's New in v4.0?

### 1. Multi-Protocol Architecture
Unified input manager supports MIDI + HID in the same config:

```toml
# MIDI pad for recording
[[modes.mappings]]
trigger = { type = "Note", note = 36 }
action = { type = "Keystroke", keys = "r", modifiers = ["cmd"] }

# Xbox A button for builds
[[modes.mappings]]
trigger = { type = "GamepadButton", button = 128 }
action = { type = "Shell", command = "cargo build" }
```

### 2. AI-Powered Configuration (Coming Q2)
Natural language config generation:

```
You: "Map A button to run cargo build"
Conductor: ✨ Generates valid TOML config
```

### 3. Raycast Integration
Control Conductor from Raycast (⌘+Space):
- Reload config
- Switch modes
- Start MIDI Learn
- View daemon status

### 4. Domain Strategy
- **getconductor.app** - Main site
- **getconductor.dev** - Documentation
- **getconductor.studio** - Premium tier
- **getconductor.gg** - Gaming focus

## Migration Guide

v4.0 is a breaking change. Follow these steps:

1. Export your config: `cp ~/.config/midimon/config.toml ~/backup.toml`
2. Install Conductor: `brew install conductor`
3. Copy config: `cp ~/backup.toml ~/.config/conductor/config.toml`
4. Update service: See [full migration guide](https://getconductor.dev/migration)

## What Happens to MIDIMon?

The `amiable-dev/midimon` repository is now archived. All future development happens at `amiable-dev/conductor`.

Existing v3.x users can continue using MIDIMon - it will keep working. But we recommend migrating to Conductor for:
- AI features (coming Q2)
- Raycast integration
- Future protocol support (OSC)
- Active development and support

## Download Conductor v4.0

- **macOS**: `brew install conductor`
- **Windows/Linux**: [getconductor.app](https://getconductor.app)
- **GitHub**: [amiable-dev/conductor](https://github.com/amiable-dev/conductor)

## Pricing

- **Free**: 10 AI generations/month, unlimited mappings
- **Studio**: $29 one-time (visual designer, 100 AI/month, marketplace)
- **Cloud**: $25/year (unlimited AI, cloud sync, voice control)

Thank you for your support! See you at the new repo 🚀

---

Questions? Comments? Join the discussion: https://github.com/amiable-dev/conductor/discussions
```

**Publish to**:
- getconductor.app/blog
- GitHub Discussions (pinned)
- Dev.to
- Medium (if applicable)

### 8.2 Community Outreach (Launch day)

**Reddit Posts**:

1. **/r/rust** - "Show Rust: Conductor - Multi-Protocol Input Automation (rebrand from MIDIMon)"
2. **/r/programming** - "Conductor: Turn any controller into a productivity tool"
3. **/r/MechanicalKeyboards** - "Use your Xbox controller as a macro pad (free, open source)"
4. **/r/Twitch** - "Stop buying Stream Decks: Use your gamepad instead"

**Hacker News Submission**:
```
Title: Conductor – Multi-protocol input automation for MIDI and game controllers
URL: https://getconductor.app
Text:
Open-source Rust/Tauri app that turns MIDI controllers and game controllers
into advanced productivity tools. Velocity-sensitive triggers, AI config
generation (coming Q2), <1ms latency. Rebrand from MIDIMon to reflect
multi-protocol support.
```

**ProductHunt Launch**:
```
Tagline: Turn any controller into a productivity powerhouse
Description: Transform MIDI controllers, Xbox/PlayStation gamepads, and custom
hardware into advanced automation tools with velocity sensitivity, AI-powered
config, and <1ms latency.

Topics: productivity, developer-tools, open-source, automation
```

**Twitter/X Thread**:
```
🎉 Introducing Conductor v4.0!

MIDIMon has been rebranded to reflect our multi-protocol vision:
🎹 MIDI controllers
🎮 Game controllers
🕹️ Custom HID devices

Same great product, bigger future.

Thread 👇

1/ Why the rebrand?

When we launched MIDIMon in 2024, it was MIDI-only. But v3.0 added gamepad
support, and the name didn't fit anymore.

"Conductor" orchestrates inputs from ANY hardware - not just MIDI.

2/ What's new in v4.0?

✅ Multi-protocol architecture
✅ AI config generation (Q2)
✅ Raycast integration
✅ Domain strategy (getconductor.{app,dev,studio,gg})
✅ Fresh repo: github.com/amiable-dev/conductor

3/ Save $270: Don't buy a Stream Deck

Your $30 Xbox controller + Conductor (free) = Stream Deck functionality

Check out getconductor.gg for gaming-focused templates 🎮

4/ Download now:

macOS: brew install conductor
Others: getconductor.app

Free tier: Unlimited mappings, 10 AI/month
Studio: $29 one-time (visual designer, marketplace)
Cloud: $25/year (unlimited AI, sync)
```

### 8.3 SEO & Search (Ongoing)

**Google Search Console**:
- Register getconductor.app
- Register getconductor.dev
- Submit sitemap.xml
- Monitor crawl errors

**301 Redirects** (if applicable):
```
# Old docs → new docs (if hosted on same server)
Redirect 301 /docs https://getconductor.dev
```

**Backlink Updates**:
- Update Reddit post history (edit with redirect)
- Update forum signatures (if any)
- Contact anyone who linked to midimon repo

**Keywords to Target**:
- "conductor input mapping"
- "gamepad macro software"
- "MIDI controller automation"
- "xbox controller macro pad"
- "turn controller into macro pad"
- "stream deck alternative"

---

## Part 9: Risk Mitigation

### 9.1 User Confusion (HIGH RISK)

**Symptoms**:
- "Where did MIDIMon go?"
- "Why can't I find the old repo?"
- "My config broke!"

**Mitigation**:
- ✅ 2-week pre-launch announcement
- ✅ Migration guide at getconductor.dev/migration
- ✅ Pinned issue in old repo with redirect
- ✅ Clear messaging: "MIDIMon → Conductor"

### 9.2 Lost SEO/Traffic (MEDIUM RISK)

**Symptoms**:
- Google still shows old "MIDIMon" results
- Documentation traffic drops
- Old backlinks broken

**Mitigation**:
- ✅ 301 redirects from old docs → new domains
- ✅ Update Google Search Console
- ✅ Monitor traffic with analytics
- ✅ Update external backlinks

### 9.3 Broken Third-Party Integrations (LOW RISK)

**Symptoms**:
- Plugins can't find `midimon-core` crate
- CI pipelines fail

**Mitigation**:
- ✅ Publish to crates.io with new names
- ✅ Consider publishing stub crate `midimon-core` that depends on `conductor-core` (optional)
- ✅ Notify plugin developers via GitHub Discussions

### 9.4 Name Collision (LOW RISK)

**Symptoms**:
- Another "Conductor" project exists
- Trademark issues

**Mitigation**:
- ✅ Check crates.io: "conductor" is available
- ✅ Check npm: "conductor" exists but inactive (not a concern)
- ✅ Check GitHub: "conductor" exists but different domain (audio software)
- ✅ Use full branding: "Conductor by Amiable" for clarity

---

## Part 10: Success Metrics

### Pre-Launch Checklist

- [ ] All code refactored and tested
- [ ] Documentation updated (60+ files)
- [ ] Domains configured (DNS + SSL)
- [ ] GitHub repo created (`amiable-dev/conductor`)
- [ ] Old repo archived with pinned redirect issue
- [ ] Announcement post drafted
- [ ] Migration guide published
- [ ] Lemon Squeezy store configured
- [ ] Raycast extension ready (optional for launch)

### Launch Day Checklist

- [ ] Push v4.0.0 to GitHub
- [ ] Publish to crates.io
- [ ] Deploy getconductor.app (Vercel)
- [ ] Deploy getconductor.dev (GitHub Pages)
- [ ] Deploy getconductor.gg (Vercel)
- [ ] Post announcement to blog
- [ ] Post to Reddit (3 subreddits)
- [ ] Submit to Hacker News
- [ ] Post Twitter/X thread
- [ ] Launch ProductHunt (optional, can delay)
- [ ] Update all external backlinks

### Post-Launch (1 week)

- [ ] Monitor GitHub issues for migration problems
- [ ] Respond to community feedback
- [ ] Track download metrics
- [ ] Monitor SEO rankings
- [ ] Update backlinks
- [ ] Submit Raycast extension PR (if not done)

### Success Metrics (30 days)

**Target Metrics**:
- GitHub stars: 500+ (was 200 for MIDIMon)
- Downloads: 5,000+ (first month)
- Studio conversions: 300 × $29 = $8,700
- Cloud subscriptions: 100 × $25 = $2,500
- Total revenue: $11,200

**Tracking**:
- Google Analytics (getconductor.app)
- GitHub Insights (conductor repo)
- Lemon Squeezy dashboard (revenue)
- Raycast extension metrics (if published)

---

## Timeline Summary

### Week 1: Codebase + Documentation
- **Day 1-2**: Package rename, Rust source updates (Phase 1.1-1.2)
- **Day 3**: System integration, config paths (Phase 1.3-1.4)
- **Day 4-5**: Documentation updates (Phase 2)
- **Day 6**: GUI strings (Phase 3)
- **Day 7**: Buffer day (catch-up)

### Week 2: Infrastructure + Launch
- **Day 8**: GitHub repo setup, crates.io (Phase 4)
- **Day 9**: DNS configuration, deploy sites (Phase 5.1-5.3)
- **Day 10**: Lemon Squeezy integration (Phase 6.1)
- **Day 11**: Raycast extension (Phase 7.1)
- **Day 12**: Final testing, QA
- **Day 13**: Pre-launch checklist
- **Day 14**: LAUNCH DAY 🚀

### Post-Launch
- **Week 3**: Community support, bug fixes
- **Week 4**: Raycast extension submission (if not done)
- **Q2 2025**: AI features (natural language config, conflict detection)

---

## Appendix A: File Change Checklist

### Critical Files (MUST CHANGE)

**Cargo.toml** (13 files):
- [ ] `/Cargo.toml` - Workspace root
- [ ] `/conductor-core/Cargo.toml`
- [ ] `/conductor-daemon/Cargo.toml`
- [ ] `/conductor-gui/src-tauri/Cargo.toml`
- [ ] 8 plugin Cargo.toml files

**Binary Names** (6 files):
- [ ] `conductor-daemon/src/main.rs` - `midimon` → `conductor`
- [ ] `conductor-daemon/src/bin/midimonctl.rs` → `conductorctl.rs`
- [ ] Man pages updated

**System Integration** (2 files):
- [ ] `com.amiable.conductor.plist` (macOS)
- [ ] `conductor.service` (Linux)

**GUI Config** (3 files):
- [ ] `conductor-gui/src-tauri/tauri.conf.json`
- [ ] `conductor-gui/ui/package.json`
- [ ] `conductor-gui/ui/index.html`

**Documentation** (60+ files):
- [ ] `README.md`
- [ ] `CLAUDE.md`
- [ ] `CONTRIBUTING.md`
- [ ] `docs-site/src/*.md` (all docs)

### Optional Files (CAN STAY)

**Protocol-Specific**:
- [ ] `midi_feedback.rs` - MIDI protocol (keep)
- [ ] `midi_output.rs` - MIDI protocol (keep)
- [ ] `midi_diagnostic` binary - MIDI tool (keep)
- [ ] `test_midi` binary - MIDI tool (keep)

---

## Appendix B: Lemon Squeezy vs OpenSaaS.sh

### Why NOT OpenSaaS.sh?

**Technical Incompatibility**:
- Requires Node.js backend - Conductor is pure Rust
- Uses Prisma ORM (JS) - Conductor uses TOML configs
- Wasp framework is JS-only - no Rust bindings
- Assumes web app hosting - Conductor is desktop-first

**Hidden Costs**:
- Node.js hosting: $20-50/month
- Database (Postgres): $15-25/month
- Total: $35-75/month base + Stripe fees (2.9%)

**Wrong Model**:
- Designed for SaaS subscriptions
- Conductor sells software licenses (one-time + annual)
- Desktop apps need different validation flow

### Why Lemon Squeezy?

**Perfect Fit for Desktop Software**:
- License key validation API built-in
- Hardware ID binding (anti-piracy)
- No server infrastructure required
- All-in pricing: 5% (includes payment + VAT + fraud)

**Better Economics** (until $10K/month):
```
At $1,000/month:
  OpenSaaS: $29 (Stripe) + $35 (hosting) = $64 (6.4%)
  Lemon Squeezy: $50 (5%) ✓ Better

At $5,000/month:
  OpenSaaS: $145 + $35 = $180 (3.6%)
  Lemon Squeezy: $250 (5%)
```

**Recommendation**: Use Lemon Squeezy until $10K/month revenue, then re-evaluate.

---

## Appendix C: Raycast Integration Benefits

### Why Build Raycast Extension?

**Discovery**:
- 100,000+ active Raycast users
- High-intent developer audience
- Free distribution (no fees)

**Complementary, Not Competitive**:
| Scenario | Raycast | Conductor |
|----------|---------|-----------|
| Launch app | Type search query | Press hardware button |
| Run command | Type command name | Muscle memory trigger |
| Complexity | Requires thinking | Zero-thought action |

**Both have value!** Users often use both together.

**ROI**:
```
Engineering: 16 hours
Potential reach: 5,000 users
Conversion: 300 Studio licenses = $8,700
Hourly rate: $544/hour
```

### Integration Opportunities

1. **Control Conductor from Raycast** ⭐⭐⭐⭐⭐
   - Reload config
   - Switch modes
   - Start MIDI Learn
   - View daemon status

2. **Open Raycast from Conductor** ⭐⭐⭐☆☆
   - Hardware button → opens Raycast with query
   - Combine hardware trigger + software launcher

3. **Cross-Promote in Docs** ⭐⭐⭐⭐☆
   - "Using Conductor with Raycast" guide
   - Show complementary workflows

---

## End of Plan

**Ready to execute?** Follow phases in order, track progress with GitHub issues/project board.

**Questions?** Review this plan, then ask before proceeding.

**Let's rebrand Conductor! 🚀**
