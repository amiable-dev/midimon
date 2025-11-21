# Conductor Branding Assets

**Status**: Post-v4.0 Rebrand (MIDIMon → Conductor)
**Last Updated**: 2025-01-21

## Current Assets Inventory

### Application Icons

**Tauri GUI Icon**:
- Location: `conductor-gui/src-tauri/icons/icon.png`
- Size: 1024x1024 recommended (current: unknown)
- Format: PNG
- Status: ✅ Exists (generic placeholder)
- **TODO**: Replace with Conductor-branded icon

### Documentation Site

**Favicons**:
- `docs-site/book/favicon.png` (5.5 KB)
- `docs-site/book/favicon.svg` (1.8 KB)
- Status: ✅ Exists (generic placeholder)
- **TODO**: Replace with Conductor-branded favicons

### Marketing Assets

**Hero Demo** (referenced in README.md):
- Expected location: `docs/images/hero-demo.gif`
- Status: ❌ **Missing**
- Purpose: Showcase velocity-sensitive RGB LED feedback
- **TODO**: Create GIF demo of Conductor in action

**Screenshots**:
- Status: ❌ **Missing**
- **TODO**: Create screenshots for:
  - GUI configuration interface
  - MIDI Learn mode
  - Event console
  - Per-app profiles view
  - Device templates selector
  - Gamepad configuration

## Recommended Assets for Complete Rebrand

### 1. Application Icon Set

**Primary App Icon** (Conductor Desktop GUI):
- [ ] `icon.png` - 1024x1024 PNG (source)
- [ ] `32x32.png` - macOS menu bar icon
- [ ] `128x128.png` - macOS dock icon
- [ ] `128x128@2x.png` - macOS Retina dock icon
- [ ] `icon.icns` - macOS bundle icon
- [ ] `icon.ico` - Windows bundle icon (16, 32, 48, 256 sizes)

**Suggested Design**:
- Concept 1: Conductor's baton with RGB spectrum trail
- Concept 2: Musical staff intersecting with gamepad D-pad
- Concept 3: Abstract wave forms with input device silhouettes
- Color palette: Vibrant spectrum (matches multi-protocol vision)
- Style: Modern, flat design with subtle gradients

### 2. Logo Variations

**Wordmark Logo**:
- [ ] `conductor-wordmark.svg` - Scalable text logo
- [ ] `conductor-wordmark-light.svg` - For dark backgrounds
- [ ] `conductor-wordmark-dark.svg` - For light backgrounds
- Fonts: Consider modern sans-serif (Inter, Poppins, or custom)

**Icon + Wordmark Combo**:
- [ ] `conductor-logo-horizontal.svg`
- [ ] `conductor-logo-vertical.svg`
- [ ] `conductor-logo-square.svg` (for social media)

**Icon Only** (for compact spaces):
- [ ] `conductor-icon.svg` - Just the mark, no text

### 3. Documentation Site Branding

**Favicons**:
- [ ] `favicon.ico` - 16x16, 32x32, 48x48 multi-size ICO
- [ ] `favicon.png` - 32x32 PNG
- [ ] `favicon.svg` - Scalable (preferred for modern browsers)
- [ ] `apple-touch-icon.png` - 180x180 for iOS
- [ ] `android-chrome-192x192.png` - Android home screen
- [ ] `android-chrome-512x512.png` - High-res Android

**Open Graph Images** (social media sharing):
- [ ] `og-image.png` - 1200x630 PNG
- [ ] `twitter-card.png` - 1200x600 PNG

### 4. Marketing & Demo Assets

**Hero Demo**:
- [ ] `hero-demo.gif` - Animated demo of key features
  - Show: Velocity sensitivity, LED feedback, mode switching
  - Duration: 10-15 seconds loop
  - Size: < 5MB optimized

**Feature Showcase GIFs**:
- [ ] `midi-learn-demo.gif` - MIDI Learn mode in action
- [ ] `velocity-demo.gif` - Soft/medium/hard press demonstration
- [ ] `led-feedback.gif` - RGB LED schemes cycling
- [ ] `gamepad-config.gif` - Configuring Xbox controller
- [ ] `hot-reload.gif` - Config change applied instantly

**Screenshots** (PNG, 2x Retina):
- [ ] `gui-overview.png` - Main GUI interface
- [ ] `devices-view.png` - MIDI + HID devices listed
- [ ] `mappings-editor.png` - Visual mapping configuration
- [ ] `event-console.png` - Live event monitoring
- [ ] `templates-selector.png` - Device templates grid
- [ ] `settings-panel.png` - Preferences and auto-start

### 5. Repository Assets

**README Header**:
- [ ] `readme-hero.png` - Large banner for GitHub README
- Suggested size: 1280x640 PNG
- Content: Logo + tagline + key visual

**Badges**:
- ✅ Already using shields.io badges (License, Rust, Build Status, etc.)
- Consider adding:
  - [ ] Version badge
  - [ ] Downloads badge
  - [ ] Discord/community badge

### 6. Brand Guidelines (Future)

**Color Palette**:
```
Primary: #4A9EFF (Blue - MIDI/digital)
Secondary: #7C3AED (Purple - hybrid/unified)
Accent: #10B981 (Green - gamepad/HID)
Success: #4ADE80
Warning: #FBBF24
Error: #EF4444
Background: #1E1E1E (Dark mode)
Text: #E0E0E0 (Light on dark)
```

**Typography**:
- Headings: Inter Bold / Poppins SemiBold
- Body: Inter Regular / System Sans
- Code: JetBrains Mono / Fira Code

**Voice & Tone**:
- Professional yet approachable
- Technical accuracy without jargon overload
- Emphasize multi-protocol flexibility
- Highlight performance and low latency

## Asset Creation Workflow

### Option 1: Design Tool (Recommended)
1. Use Figma, Sketch, or Affinity Designer
2. Create 1024x1024 icon at vector resolution
3. Export to PNG at multiple sizes
4. Use ImageMagick/sips to generate platform-specific formats:
```bash
# macOS icon bundle
iconutil -c icns conductor-icon.iconset

# Windows ICO
convert icon-{16,32,48,256}.png icon.ico
```

### Option 2: AI-Assisted Generation
1. Use Midjourney/DALL-E 3 for initial concepts
2. Refine in vector editor (Inkscape/Illustrator)
3. Export production assets

### Option 3: Commission Designer
- Recommended for production-ready brand identity
- Budget: $200-500 for icon + logo variations
- Platforms: Fiverr, 99designs, Dribbble

## Repository URLs Update

**Current State**: Code uses `amiable-dev/conductor` URLs in badges, but repository is still `amiable-dev/midimon`

**Options**:
1. **Keep midimon repo**: Update all badges back to `amiable-dev/midimon`
2. **Migrate to conductor repo**: Create fresh repo, archive old one
3. **Rename repo**: Use GitHub's rename feature (redirects automatically)

**Recommendation**: Option 3 (Rename) is cleanest - preserves history, auto-redirects old URLs

## Priority Action Items

### Immediate (This Week):
1. ✅ Update Tauri bundle descriptions (DONE: multi-protocol messaging)
2. [ ] Decide on repository strategy (rename vs fresh repo)
3. [ ] Update README badges if keeping midimon repo name

### Short-Term (Next 2 Weeks):
4. [ ] Create basic Conductor icon (1024x1024 PNG)
5. [ ] Generate platform-specific icons from source
6. [ ] Update favicons for documentation site
7. [ ] Create 1-2 demo GIFs for README

### Medium-Term (Next Month):
8. [ ] Commission or design full logo variations
9. [ ] Create comprehensive screenshot library
10. [ ] Develop brand guidelines document
11. [ ] Create social media Open Graph images

## Notes

- **Icon Design Philosophy**: Should visually represent multi-protocol unification (MIDI + HID + future OSC)
- **Avoid**: Music-only iconography (no longer just MIDI-focused)
- **Consider**: Abstract representations of "conducting" multiple input streams
- **Target Audience**: Developers, musicians, streamers, power users
- **Platforms**: Primarily macOS, secondarily Linux/Windows

---

**Questions for Design Review**:
1. Should the icon be protocol-agnostic or subtly reference MIDI heritage?
2. Flat design vs subtle 3D/depth?
3. Color: Single primary color or multi-color spectrum?
4. Wordmark: All lowercase "conductor" or title case "Conductor"?
