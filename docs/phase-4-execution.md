# Execute MIDIMon Phase 4: Tauri UI & Visual Configuration (v2.0.0)

## Context
- **Phase**: 4 of 5
- **Epic**: AMI-108 (Phase 4: Tauri UI & Visual Configuration)
- **Status**: Planned - Starts after Phase 3 complete
- **Duration**: 4-6 weeks (2026-01-27 to 2026-03-10)
- **Total Issues**: 26 issues (23 from AMI-108 + 3 menu bar issues moved)
- **Target Version**: v2.0.0

## Prerequisites Check

Before starting, verify:
- [ ] Phase 3 (AMI-107) marked complete
- [ ] Daemon infrastructure operational with hot-reload
- [ ] IPC server functional (<1ms latency)
- [ ] State persistence working (atomic writes)
- [ ] All blocking dependencies resolved (see Dependency Documents below)
- [ ] Documentation site infrastructure ready (AMI-258 complete)
- [ ] Team has capacity for 4-6 weeks of work

## Dependency Documents

- Read @docs/implementation-roadmap.md (Phase 4 section, Documentation Site Update Policy lines 159-219)
- Read @docs/linear-dependencies.md (Phase 4 dependencies, Epic dependencies)
- Read @docs/traceability-matrix.md (features TF1, TF3-TF10 requiring attention)
- Read @docs/architecture.md (v2.0 target architecture, Tauri integration)
- Read @docs/api-design.md (Tauri commands API)
- Read @docs/features.md (Target Features TF1-TF10 specifications)
- Read AMI-108 in Linear for detailed objectives

## Execution Instructions

### Step 1: Phase Analysis

#### 1.1 List All Child Issues

Use Linear MCP to retrieve all issues under AMI-108:

```
1. AMI-158: Add minimal menu bar icon using tray-icon crate (2 days)
2. AMI-159: Implement platform-specific menu bar (macOS/Linux/Windows) (3 days)
3. AMI-160: Add status display and quick actions (2 days)
4-9. Week 1-2: Tauri Setup & Basic UI (6 issues, to be created)
10-13. Week 3: MIDI Learn Mode (4 issues)
14-19. Week 4: Visual Config Editor (6 issues)
20-23. Week 5: Per-App Profiles (4 issues)
24-26. Week 6: Polish & Release (3 issues)
```

**Total**: 26 issues across 6 weeks

#### 1.2 Extract Dependency Relationships

From docs/linear-dependencies.md and AMI-108 description:

**Critical Path Dependencies**:
```
Phase 3 (AMI-107) complete
    ↓ blocks
AMI-108 (Phase 4 start)
    ↓ enables
Week 1-2: Tauri Setup + Menu Bar (AMI-158-160, 6 Tauri issues)
    ↓ blocks
Week 3: MIDI Learn (4 issues) - requires Tauri backend
    ↓ blocks
Week 4: Config Editor (6 issues) - requires MIDI Learn
    ↓ blocks
Week 5: Per-App Profiles (4 issues) - requires config editor
    ↓ blocks
Week 6: Polish (3 issues) - requires all above
```

**Documentation Site Infrastructure**:
```
AMI-258 (Docs Infrastructure) complete
    ↓ enables
All Phase 4 issues - documentation updates required per policy
```

**Feature Epic Dependencies**:
```
Epic 8: Visual Configuration & MIDI Learn (AMI-100)
    ↓ blocks
Epic 9: Configuration Management (AMI-101)
    ↓ blocks
Epic 10: Per-App Context Awareness (AMI-102)
```

#### 1.3 Create Dependency-Ordered Work Queue

```
Phase 4 Work Queue:

Priority 0 (Start Immediately - Week 1):
- AMI-158: Add minimal menu bar icon - No dependencies, blocks AMI-159, AMI-160
- [Tauri-1]: Create midimon-gui Tauri v2 project - No dependencies, blocks all Tauri work

Priority 1 (After P0 Complete - Week 1-2):
- AMI-159: Platform-specific menu bar - Depends on AMI-158
- AMI-160: Status display & quick actions - Depends on AMI-159
- [Tauri-2]: Implement Tauri backend commands - Depends on [Tauri-1]
- [Tauri-3]: Create basic UI shell with navigation - Depends on [Tauri-1]
- [Tauri-4]: Add device connection panel - Depends on [Tauri-3]
- [Tauri-5]: Implement status bar - Depends on [Tauri-3]
- [Tauri-6]: Set up frontend state management - Depends on [Tauri-3]

Priority 2 (After P1 Complete - Week 3):
- [MIDILearn-1]: Implement MIDI Learn backend - Depends on [Tauri-2]
- [MIDILearn-2]: Create Learn UI flow - Depends on [MIDILearn-1], [Tauri-3]
- [MIDILearn-3]: Support all trigger types learning - Depends on [MIDILearn-1]
- [MIDILearn-4]: Add auto-fill trigger config - Depends on [MIDILearn-2], [MIDILearn-3]

Priority 3 (After P2 Complete - Week 4):
- [ConfigEditor-1]: Build mode editor UI - Depends on [Tauri-3]
- [ConfigEditor-2]: Build mapping list UI - Depends on [ConfigEditor-1]
- [ConfigEditor-3]: Create visual trigger selector - Depends on [ConfigEditor-2]
- [ConfigEditor-4]: Create visual action selector - Depends on [ConfigEditor-2]
- [ConfigEditor-5]: Add keystroke picker - Depends on [ConfigEditor-4]
- [ConfigEditor-6]: Implement live preview - Depends on [ConfigEditor-3], [ConfigEditor-4]

Priority 4 (After P3 Complete - Week 5):
- [PerApp-1]: Implement frontmost app detection - Depends on daemon (Phase 3)
- [PerApp-2]: Create profile switching system - Depends on [PerApp-1], [ConfigEditor-1]
- [PerApp-3]: Build per-app profiles UI - Depends on [PerApp-2], [ConfigEditor-1]
- [PerApp-4]: Add profile import/export - Depends on [PerApp-3]

Priority 5 (After P4 Complete - Week 6):
- AMI-185: Create device template system (TF5) - Depends on [ConfigEditor-1]
- [Polish-1]: Add live event console (TF9) - Depends on [Tauri-3]
- [Polish-2]: Build settings panel with auto-start (TF8) - Depends on [Tauri-3]

Parallel Groups:
Group A (Week 1): AMI-158, [Tauri-1] (can be done simultaneously)
Group B (Week 1-2): AMI-159, AMI-160, [Tauri-2-6] (after Group A, parallel within group)
Group C (Week 3): [MIDILearn-1-4] (sequential within group)
Group D (Week 4): [ConfigEditor-1-6] (some parallelizable after ConfigEditor-2)
Group E (Week 5): [PerApp-1-4] (sequential PerApp-1→2, then parallel PerApp-3+4)
Group F (Week 6): AMI-185, [Polish-1], [Polish-2] (all parallel)

Critical Path: [Tauri-1] → [Tauri-2] → [MIDILearn-1] → [MIDILearn-2] → [ConfigEditor-1] → [ConfigEditor-2] → [PerApp-1] → [PerApp-2] (28-35 days)
```

### Step 2: Create Execution Plan

#### Ready to Start (No Blockers)

**Week 1 - Foundation (7 days)**:
1. AMI-158: Add minimal menu bar icon (2 days)
   - Can start immediately
   - Uses tray-icon crate
   - Platform-specific icon states (running/stopped/error)

2. [Tauri-1]: Create midimon-gui Tauri v2 project (2 days)
   - Can start immediately in parallel with AMI-158
   - Set up workspace structure
   - Configure Tauri v2 with modern security settings

**Estimate**: 2-3 days if parallelized, 4 days if sequential

#### Blocked Issues

**Week 1-2 - Tauri Setup (Blocked by Week 1)**:
- AMI-159: Platform-specific menu bar - Blocked by AMI-158 (needs tray icon foundation)
- AMI-160: Status display & quick actions - Blocked by AMI-159 (needs menu structure)
- [Tauri-2-6]: All Tauri backend/UI work - Blocked by [Tauri-1] (needs project structure)

**Clearing**: Will unblock when AMI-158 and [Tauri-1] complete (~day 3)

**Week 3 - MIDI Learn (Blocked by Week 1-2)**:
- All MIDI Learn issues - Blocked by [Tauri-2] (need backend commands) and [Tauri-3] (need UI shell)

**Clearing**: Will unblock when Tauri setup complete (~day 10)

**Week 4 - Config Editor (Blocked by Week 3)**:
- Config editor issues - Blocked by MIDI Learn completion (need learn integration)

**Clearing**: Will unblock when MIDI Learn complete (~day 17)

**Week 5 - Per-App Profiles (Blocked by Week 4)**:
- Per-app issues - Blocked by config editor (need profile management UI)

**Clearing**: Will unblock when config editor complete (~day 24)

**Week 6 - Polish (Blocked by Week 5)**:
- Polish issues - Blocked by per-app profiles complete
- Can parallelize all 3 polish issues

**Clearing**: Will unblock when per-app profiles complete (~day 31)

#### Parallel Work Groups

**Group A: Week 1 Foundation (Parallel)**
- AMI-158: Menu bar icon (Developer A)
- [Tauri-1]: Tauri project setup (Developer B)
- **Rationale**: Independent work - one focuses on native menu bar, other on Tauri scaffold

**Group B: Week 1-2 Tauri Infrastructure (Mostly Parallel)**
- AMI-159: Menu bar implementation (Developer A) - after AMI-158
- AMI-160: Status display (Developer A) - after AMI-159
- [Tauri-2]: Backend commands (Developer B) - after [Tauri-1]
- [Tauri-3-6]: UI components (Developer C) - after [Tauri-1]
- **Rationale**: Menu bar work is sequential (A), but Tauri backend/UI can be parallel (B+C)

**Group C: Week 3 MIDI Learn (Partially Parallel)**
- [MIDILearn-1]: Backend session (Developer B)
- [MIDILearn-2]: Learn UI flow (Developer C) - depends on MIDILearn-1
- [MIDILearn-3]: Trigger types support (Developer B) - parallel with MIDILearn-2
- [MIDILearn-4]: Auto-fill config (Developer C) - after MIDILearn-2+3
- **Rationale**: Backend/UI separation allows some parallelization

**Group D: Week 4 Config Editor (Partially Parallel)**
- [ConfigEditor-1]: Mode editor (Developer A)
- [ConfigEditor-2]: Mapping list (Developer A) - after ConfigEditor-1
- [ConfigEditor-3-4]: Trigger/action selectors (Developers B+C) - after ConfigEditor-2 (parallel)
- [ConfigEditor-5-6]: Keystroke picker + preview (Developer B) - after ConfigEditor-3+4
- **Rationale**: Base UI first, then selectors parallel, then integration

**Group E: Week 5 Per-App Profiles (Partially Parallel)**
- [PerApp-1]: Frontmost app detection (Developer B) - daemon work
- [PerApp-2]: Profile switching (Developer B) - after PerApp-1
- [PerApp-3]: Per-app UI (Developer C) - parallel with PerApp-2 (just UI, no backend)
- [PerApp-4]: Import/export (Developer A) - after PerApp-2+3
- **Rationale**: Backend sequential, UI can start early

**Group F: Week 6 Polish (Fully Parallel)**
- AMI-185: Device templates (Developer A)
- [Polish-1]: Live event console (Developer B)
- [Polish-2]: Settings panel (Developer C)
- **Rationale**: All independent features, maximum parallelization

#### Critical Path Analysis

**Longest dependency chain**: 28-35 days
```
[Tauri-1] (2d) → [Tauri-2] (2d) → [MIDILearn-1] (2d) → [MIDILearn-2] (3d) →
[ConfigEditor-1] (2d) → [ConfigEditor-2] (2d) → [PerApp-1] (2d) → [PerApp-2] (2d) →
[PerApp-3] (3d) → Testing (5-7d)
```

**Bottleneck issues** (block the most downstream work):
1. [Tauri-1]: Blocks all 25 remaining issues
2. [Tauri-2]: Blocks 18 issues (all MIDI Learn, Config Editor, Per-App)
3. [MIDILearn-1]: Blocks 14 issues (all Config Editor, Per-App, some Polish)
4. [ConfigEditor-1]: Blocks 10 issues (all Per-App, some Polish)

**Risk factors**:
- **Tauri v2 Learning Curve**: First-time Tauri v2 usage could cause delays
- **Platform-Specific Menu Bar**: macOS/Linux/Windows differences in AMI-159
- **MIDI Learn Complexity**: Session management and timeout handling in [MIDILearn-1]
- **Frontmost App Detection**: Platform-specific APIs in [PerApp-1]

**Mitigation**:
- Start Tauri v2 research/prototyping before Phase 4 official start
- Plan platform-specific testing time for menu bar
- Prototype MIDI Learn session before full implementation
- Research frontmost app APIs early (macOS NSWorkspace, Linux wmctrl, Windows GetForegroundWindow)

#### Estimated Timeline

**Optimistic**: 28 days (4 weeks)
- Assumes: Perfect parallelization, no blockers, experienced Tauri developers
- Risk: Unrealistic for first Tauri project

**Realistic**: 35-42 days (5-6 weeks)
- Assumes: Some parallelization, minor blockers resolved quickly
- Accounts for: Learning curve, testing, documentation time
- **Target Completion**: 2026-03-10 (matches AMI-108)

**Pessimistic**: 49-56 days (7-8 weeks)
- Assumes: Serial work, significant Tauri v2 blockers, platform issues
- Risk: Would delay v2.0 release
- Mitigation: Start early, prototype risky areas, get Tauri help from community

**Recommended Approach**: Target **Realistic** timeline (5-6 weeks) with pessimistic buffer

### Step 3: Execute Work (After User Approval)

For each issue in dependency order:

#### Pre-Execution Check
- [ ] All dependencies resolved (check Linear status)
- [ ] Required context gathered (specs, designs, Tauri docs references)
- [ ] Documentation site structure ready for updates
- [ ] Tauri v2 dependencies installed and tested

#### Execution (Per Issue)

1. **Update Linear**: Set issue status to "In Progress"

2. **Do the work**:
   - **Week 1 (AMI-158, [Tauri-1])**: Menu bar icon + Tauri project setup
   - **Week 1-2 (AMI-159-160, Tauri-2-6)**: Menu bar UI + Tauri infrastructure
   - **Week 3 ([MIDILearn-1-4])**: MIDI Learn implementation (TF1)
   - **Week 4 ([ConfigEditor-1-6])**: Visual config editor (TF4)
   - **Week 5 ([PerApp-1-4])**: Per-app profiles (TF6, TF7)
   - **Week 6 (AMI-185, Polish-1-2)**: Device templates (TF5), polish features

3. **Update docs-site/**: Per Documentation Site Update Policy (CRITICAL)

   **For GUI Features** (TF3, TF4):
   - [ ] Create `docs-site/src/getting-started/gui-quick-start.md`
   - [ ] Update `docs-site/src/getting-started/quick-start.md` with GUI option
   - [ ] Add screenshots to `docs-site/src/assets/screenshots/`
   - [ ] Update all `docs-site/src/configuration/` guides for visual editor

   **For MIDI Learn** (TF1):
   - [ ] Create `docs-site/src/getting-started/midi-learn.md` tutorial
   - [ ] Add `docs-site/src/configuration/midi-learn-examples.md`
   - [ ] Include animated GIFs showing MIDI Learn workflow
   - [ ] Update troubleshooting for MIDI Learn timeout issues

   **For Per-App Profiles** (TF6, TF7):
   - [ ] Create `docs-site/src/configuration/per-app-profiles.md` guide
   - [ ] Add `docs-site/src/configuration/frontmost-app-detection.md`
   - [ ] Include examples for popular apps (VSCode, Logic Pro, Figma)
   - [ ] Update troubleshooting for app detection issues

   **For Device Templates** (TF5):
   - [ ] Update `docs-site/src/devices/` with template creation guide
   - [ ] Add `docs-site/src/devices/template-format.md` reference
   - [ ] Document 4+ device templates (Mikro MK3, Launchpad, etc.)

   **For Auto-Start** (TF8):
   - [ ] Update `docs-site/src/getting-started/installation.md`
   - [ ] Add platform-specific auto-start instructions

   **For Live Event Console** (TF9):
   - [ ] Update `docs-site/src/troubleshooting.md` with console usage
   - [ ] Add `docs-site/src/reference/event-console.md` reference

4. **Verify**: Run `cd docs-site && mdbook build` to verify no errors

5. **Validate Definition of Done** (REQUIRED before marking "Done"):
   - [ ] All acceptance criteria from Linear issue are met
   - [ ] All success criteria checkboxes are completed
   - [ ] All required deliverables are created and committed
   - [ ] Tests pass locally (if code changes)
   - [ ] Test coverage ≥80% with 100% tests passing (if code changes)
   - [ ] All GitHub Actions pass (build, lint, test, security, docs)
   - [ ] Reviewed GitHub Actions logs for warnings or issues
   - [ ] **Documentation site updated per policy** (ALL GUI features)
   - [ ] **Screenshots/GIFs added** for visual features
   - [ ] Code/content reviewed (self-review minimum, PR review ideal)
   - [ ] No known critical bugs or incomplete implementations
   - [ ] Manually verified the implementation works as specified
   - [ ] **Tauri app builds successfully** on macOS (Intel + ARM)

6. **Update Linear**: Set issue status to "Done" (only after validation)

7. **Report Progress**: Log completion and next steps

#### Post-Execution
- Unblock dependent issues
- Update work queue
- Report progress to user

### Step 4: Phase Completion

When all 26 issues complete:

#### Verification Checklist
- [ ] All 26 child issues of AMI-108 marked "Done"
- [ ] All success criteria met (see AMI-108 in Linear):
  - [ ] Visual config editor works for all features
  - [ ] MIDI Learn mode works reliably
  - [ ] Per-app profile switching automatic
  - [ ] Device templates for 4+ popular controllers
  - [ ] Auto-start installation working
  - [ ] User documentation complete with screenshots
  - [ ] Beta user feedback positive
  - [ ] All v0.1.0 features still work
  - [ ] Latency <1ms, memory <20MB
  - [ ] **Documentation site comprehensively updated for v2.0**
- [ ] Test coverage targets achieved (≥80%)
- [ ] **Documentation site fully updated with v2.0 features**
- [ ] No critical bugs or blockers remain
- [ ] Performance metrics within targets

#### Documentation Updates
- [ ] Update docs/traceability-matrix.md with phase completion
- [ ] Update docs/implementation-roadmap.md progress tracking
- [ ] Update AMI-108 description with actual completion date
- [ ] Create phase completion summary document
- [ ] **Publish docs-site/ v2.0 documentation to GitHub Pages**

#### Phase Closeout
1. **Update Linear**: Set AMI-108 status to "Done"
2. **Create Git Tag**: `git tag -a v2.0.0 -m "Phase 4: Tauri UI & Visual Configuration Complete"`
3. **Phase Review Meeting**: Schedule with stakeholders
4. **Lessons Learned**: Document what went well/poorly with Tauri v2
5. **Next Phase Kickoff**: Prepare Phase 5 execution (Advanced Features)

## Documentation Site Policy

**CRITICAL**: All issues MUST update docs-site/ per policy in docs/implementation-roadmap.md lines 159-219

### Phase 4 Specific Updates Required

**ALL Issues Must**:
- Update relevant pages in docs-site/ with feature documentation
- Add screenshots/GIFs for all GUI features
- Include configuration examples where applicable
- Update troubleshooting section if new issues possible
- Verify mdbook build succeeds locally
- Review generated HTML for formatting and clarity

**TF3 (Menu Bar UI)**:
- Add `docs-site/src/getting-started/menu-bar.md`
- Document menu actions (pause, reload, configure, quit)
- Include macOS/Linux/Windows differences

**TF4 (Visual Config Editor)**:
- Create comprehensive `docs-site/src/configuration/visual-editor/` section
- Add tutorials for each editor component
- Include video walkthroughs or animated GIFs

**TF1 (MIDI Learn)**:
- Create `docs-site/src/getting-started/midi-learn.md` tutorial
- Document timeout behavior, cancellation, trigger type support
- Include troubleshooting for common MIDI Learn issues

**TF6+TF7 (Per-App Profiles)**:
- Create `docs-site/src/configuration/per-app-profiles.md`
- Document frontmost app detection per platform
- Include example profiles for popular applications

**TF5 (Device Templates)**:
- Update `docs-site/src/devices/` section extensively
- Document template format and creation process
- Add 4+ device template references

### Definition of Done Template for Phase 4

```markdown
### Documentation Site Updates
- [ ] Updated relevant pages in docs-site/ with feature documentation
- [ ] Added screenshots/GIFs for GUI features (use macOS for primary screenshots)
- [ ] Created tutorials in docs-site/src/getting-started/ where applicable
- [ ] Updated all relevant docs-site/src/configuration/ pages
- [ ] Added reference documentation if new trigger/action types added
- [ ] Updated troubleshooting section with new issues and resolutions
- [ ] Verified mdbook build succeeds locally (`cd docs-site && mdbook build`)
- [ ] Reviewed generated HTML for formatting, clarity, and broken links
- [ ] Tested all configuration examples in documentation
```

## Progress Reporting

After each issue completion, report:

```
Phase 4 Progress Report
=====================================
Date: [YYYY-MM-DD]
Elapsed: [X] days / 35-42 total

Completed: [X/26] issues
- AMI-158: Add minimal menu bar icon ✅
- [Tauri-1]: Create midimon-gui project ✅
- ... (list completed issues)

In Progress: [List]
- AMI-159: Platform-specific menu bar - 60% complete
- [Tauri-2]: Tauri backend commands - 30% complete

Blocked: [List with blockers]
- [MIDILearn-1]: MIDI Learn backend - Blocked by [Tauri-2] (clearing in 2 days)

Next Up: [Next 3 issues in queue]
1. AMI-160: Status display - Ready after AMI-159 complete
2. [Tauri-3]: UI shell - Ready after [Tauri-1] complete
3. [Tauri-4]: Device connection panel - Ready after [Tauri-3] complete

Estimated Completion: 2026-03-10 (on track / at risk / delayed)

Risks/Blockers:
- Tauri v2 learning curve causing 20% slowdown on [Tauri-2]
- Platform-specific menu bar issues on Linux (researching libappindicator)
```

## Questions to Confirm

Before beginning execution, confirm:

1. **Approach**: Should I proceed with Step 1 (Analyze Phase)?
2. **Approval Gate**: Do you want to review the execution plan before I start work?
3. **Execution Mode**:
   - **Hybrid** (recommended): Parallel where possible, sequential for critical path
   - Sequential (one issue at a time, safer but slower)
   - Parallel (multiple issues simultaneously, faster but riskier)
4. **Reporting Frequency**: How often should I report progress?
   - After each issue (26 reports)
   - Daily summary (recommended for 6-week phase)
   - Weekly summary (6 reports)
   - Milestone-based (after each week's work complete)
5. **Autonomy Level**:
   - **Group approval** (recommended): Confirm before each week's parallel group
   - Full autonomy (execute all issues automatically)
   - Issue-by-issue approval (confirm before each issue)

## Emergency Procedures

### If Blocked

1. **Document the blocker clearly** in Linear issue
2. **Identify alternative work** (non-blocked issues in same or different week)
3. **Escalate if blocker is external** (Tauri v2 bug, platform API unavailable)
4. **Consider workarounds or temporary solutions** (mock data, feature flags)

**Example**: If [MIDILearn-1] blocked by Tauri IPC issue:
- Alternative work: Start [ConfigEditor-1] UI mockups
- Escalate: File Tauri issue, ask on Discord
- Workaround: Use HTTP endpoint temporarily

### If Scope Changes

1. **Pause execution** of affected issues
2. **Document scope change request** with rationale
3. **Update work queue and timeline** (may extend Phase 4)
4. **Get user approval** before continuing with new scope

**Example**: If user requests "Virtual MIDI output" mid-phase:
- Pause: Current work continues
- Document: Create new issue, estimate 1-2 weeks
- Update: Timeline extends to 7-8 weeks
- Approve: Get confirmation before adding to queue

### If Quality Issues Found

1. **Stop and report the issue** in Linear and progress report
2. **Assess impact on dependent work** (does it block downstream issues?)
3. **Create fix-it issue if needed** (add to queue)
4. **Adjust timeline if significant** (delay phase completion)

**Example**: If MIDI Learn has critical timeout bug:
- Stop: Don't mark [MIDILearn-1] as "Done"
- Assess: Blocks [MIDILearn-2-4] and [ConfigEditor-*]
- Create: AMI-XXX: Fix MIDI Learn timeout (2 days)
- Adjust: Phase 4 extends by 2 days

---

## Phase-Specific Customization for Phase 4

### Focus Areas

**Week 1-2**: Tauri v2 Foundation
- First Tauri v2 project in MIDIMon
- Platform-specific menu bar (macOS NSStatusItem, Linux libappindicator, Windows system tray)
- IPC communication patterns (invoke commands, events)
- Security considerations (CSP, Tauri allowlist)

**Week 3**: MIDI Learn Innovation
- Session management for learn mode (timeout, cancel)
- Real-time MIDI event capture and display
- Auto-fill trigger configuration from learned event
- Support for all trigger types (velocity, long press, chord, etc.)

**Week 4**: Visual Configuration Revolution
- Drag-and-drop action library
- Visual trigger selector (dropdown/wizard)
- Live preview of mappings
- Keystroke picker with modifier support

**Week 5**: Per-App Intelligence
- Frontmost app detection (platform-specific)
- Automatic profile switching
- Per-app profile UI (create, edit, assign)
- Profile import/export (JSON format)

**Week 6**: Production Polish
- Device template system (community-shareable)
- Live event console (debugging tool)
- Settings panel with auto-start
- Beta testing and feedback incorporation

### Key Success Metrics

**Performance Targets**:
- UI responsiveness: <16ms (60fps)
- MIDI Learn timeout: 10 seconds default
- Profile switch latency: <50ms
- App detection latency: <50ms
- GUI memory: <50MB
- Daemon memory: <20MB (from Phase 3)

**Quality Targets**:
- Zero regressions from v0.1.0 features
- All MIDI devices from v0.1.0 still work
- Tauri app builds on macOS Intel + ARM
- Menu bar works on macOS, Linux (GNOME, KDE), Windows

**Documentation Targets**:
- 100% of GUI features documented with screenshots
- MIDI Learn tutorial with video/GIF walkthrough
- Per-app profile guide with 5+ app examples
- Device template creation guide with 4+ templates
- Comprehensive troubleshooting for GUI issues

### Platform Considerations

**macOS (Primary Platform)**:
- NSStatusItem for menu bar (native look)
- NSWorkspace for frontmost app detection
- LaunchAgent for auto-start (from Phase 3)
- Universal binary (Intel + ARM)

**Linux (Secondary Platform)**:
- libayatana-appindicator for menu bar (modern)
- libappindicator3 fallback (legacy)
- wmctrl for frontmost app detection (X11)
- systemd user service for auto-start

**Windows (Tertiary Platform)**:
- System tray icon with WinAPI
- GetForegroundWindow for frontmost app
- Registry startup entry for auto-start
- Windows 10/11 support

---

## Related Documents

- docs/implementation-roadmap.md - Overall project roadmap
- docs/linear-dependencies.md - Dependency graph
- docs/traceability-matrix.md - Feature tracking (TF1, TF3-TF10)
- docs/architecture.md - v2.0 target architecture
- docs/api-design.md - Tauri commands API
- docs/features.md - Target Features specifications
- docs/PRD-main.md - Product requirements
- docs/phase-execution-guide.md - Quick reference
- docs/PHASE_EXECUTION_README.md - System overview
- docs/phase-execution-template.md - Template reference

---

**End of Phase 4 Execution Guide**

**Version**: 1.0
**Created**: 2025-11-14
**AMI-108 Sub-Issues Moved**: AMI-158, AMI-159, AMI-160
**Total Issues**: 26 (23 from AMI-108 + 3 menu bar)
**Maintained By**: MIDIMon Project Team
