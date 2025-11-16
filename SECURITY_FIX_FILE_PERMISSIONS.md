# Security Fix: File Permissions on State and Socket Files

## Issue Summary

**Severity**: MEDIUM
**Category**: File Permission Vulnerability
**Status**: ✅ FIXED

### Vulnerabilities Addressed

1. **State files created with world-readable permissions** (0644)
   - Risk: Sensitive daemon state exposed to all local users
   - Impact: Information disclosure, privacy violation

2. **Socket directory in shared /tmp with loose permissions**
   - Risk: Symlink attacks, race conditions, unauthorized IPC access
   - Impact: Privilege escalation, unauthorized daemon control

3. **No ownership validation on existing directories**
   - Risk: Directory takeover by malicious users
   - Impact: Denial of service, data corruption

## Security Fixes Implemented

### 1. State File Permissions (`midimon-daemon/src/daemon/state.rs`)

**Lines Fixed**: 123-163, 170-200

**Changes**:
- Set state files to **0600 (rw-------)** immediately after creation
- Applied in both async save and emergency save paths
- Unix-only implementation using `std::os::unix::fs::PermissionsExt`
- Windows skips permission setting (NTFS has different ACL model)

**Code Example**:
```rust
// Set secure permissions (owner-only read/write) on Unix
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    let mut perms = tokio::fs::metadata(&temp_file).await?.permissions();
    perms.set_mode(0o600); // rw-------
    tokio::fs::set_permissions(&temp_file, perms).await?;
}
```

### 2. Socket Directory Security (`midimon-daemon/src/daemon/state.rs`)

**Lines Fixed**: 273-422

**Changes**:
- **Moved socket from /tmp to user-specific directories**:
  - macOS: `~/Library/Application Support/midimon/run/midimon.sock`
  - Linux: `$XDG_RUNTIME_DIR/midimon/midimon.sock` (preferred) or `~/.midimon/run/midimon.sock`
  - Windows: Named pipe (inherently user-isolated)

- **Directory permissions**: 0700 (rwx------) - owner-only access
- **Socket file permissions**: 0600 (rw-------) - owner-only read/write
- **Ownership validation**: Verify directory owner matches current user
- **Auto-fix insecure permissions** on existing directories

**Security Benefits**:
- Prevents symlink attacks (user-specific directories)
- Prevents race conditions (atomic directory creation)
- Prevents unauthorized IPC access (owner-only permissions)
- Prevents directory takeover (ownership validation)

### 3. State Directory Security (`midimon-daemon/src/daemon/state.rs`)

**Lines Fixed**: 203-271

**Changes**:
- Set state directory to **0700 (rwx------)**
- Validate directory ownership matches current user
- Auto-fix insecure permissions on existing directories
- Fail with clear error if directory owned by different user

**Error Message Example**:
```
State directory "/home/user/.midimon" is owned by UID 1001 but current user is UID 1000.
This is a security risk. Please remove the directory or fix ownership.
```

### 4. IPC Socket Permissions (`midimon-daemon/src/daemon/ipc.rs`)

**Lines Fixed**: 77-101

**Changes**:
- Set socket file to **0600 (rw-------)** after listener creation
- Graceful fallback if permission setting fails (with warning)
- Updated documentation to reflect new security model

### 5. Platform-Specific Implementation

**All permission code uses `#[cfg(unix)]`**:
- Unix (macOS/Linux): Full permission enforcement
- Windows: Skipped (uses OS-native ACL model)
- No breaking changes to cross-platform compatibility

## Testing

### Test Updates

1. **Socket path tests** updated to reflect new user-specific paths
2. **All 52 tests pass** (1 ignored for CI)
3. **No breaking changes** to existing functionality

### Validation

Run tests to verify security fixes:
```bash
# Test state management
cargo test --package midimon-daemon --lib daemon::state

# Test IPC socket creation
cargo test --package midimon-daemon --lib daemon::ipc

# Run all daemon tests
cargo test --package midimon-daemon --lib
```

## Dependencies Added

**`libc = "0.2"`** (Unix only)
- Required for `getuid()` to validate directory ownership
- Platform-specific dependency in `Cargo.toml`

```toml
[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

## Security Improvements Summary

| File | Old Permissions | New Permissions | Protection |
|------|----------------|-----------------|------------|
| State files | 0644 (world-readable) | 0600 (owner-only) | Privacy |
| State directory | 0755 (world-accessible) | 0700 (owner-only) | Access control |
| Socket directory | /tmp with 0777 | User-specific 0700 | Isolation |
| Socket file | 0644 (default) | 0600 (owner-only) | IPC security |

## Impact

### Security
- ✅ Prevents information disclosure
- ✅ Prevents unauthorized IPC access
- ✅ Prevents symlink attacks
- ✅ Prevents directory takeover
- ✅ Prevents race conditions

### Compatibility
- ✅ Backward compatible (existing functionality unchanged)
- ✅ Cross-platform (Unix security, Windows no-op)
- ✅ Auto-migrates insecure existing directories
- ✅ Clear error messages for ownership issues

### Performance
- ✅ Negligible overhead (one-time permission setting)
- ✅ No impact on runtime performance
- ✅ No impact on IPC latency

## Recommendations

### For Users
1. **Remove existing /tmp/midimon.sock** if present
2. **Verify permissions** after upgrade:
   ```bash
   ls -la ~/.midimon/state.json
   ls -la ~/Library/Application\ Support/midimon/run/
   ```
3. **Expected permissions**:
   - State file: `-rw-------` (0600)
   - Directories: `drwx------` (0700)

### For Developers
1. **Test on multi-user systems** to verify isolation
2. **Test permission migration** with existing insecure directories
3. **Verify ownership validation** prevents takeover attacks

## Verification Checklist

- [x] State files created with 0600 permissions
- [x] Emergency save sets 0600 permissions
- [x] Socket directory uses user-specific path
- [x] Socket directory has 0700 permissions
- [x] Socket file has 0600 permissions
- [x] Directory ownership validated before use
- [x] Insecure permissions auto-fixed with warnings
- [x] Clear error for wrong ownership
- [x] Unix-only implementation (Windows unaffected)
- [x] All tests passing
- [x] No breaking changes

## Files Modified

1. **midimon-daemon/src/daemon/state.rs** - State and socket security
2. **midimon-daemon/src/daemon/ipc.rs** - Socket permission setting
3. **midimon-daemon/Cargo.toml** - Added libc dependency

## Lines of Code Changed

- **state.rs**: ~150 lines added/modified
- **ipc.rs**: ~20 lines added/modified
- **Cargo.toml**: 3 lines added

## References

- CWE-732: Incorrect Permission Assignment for Critical Resource
- OWASP: Insecure File Permissions
- XDG Base Directory Specification (Linux)
- Apple File System Programming Guide (macOS)
