# Security Fix Summary: Socket Path Isolation

## Issue Fixed
**MEDIUM Severity**: Hardcoded socket path in shared `/tmp` directory

## Changes Made

### 1. Socket Path Selection (state.rs)

#### Before (INSECURE)
```rust
pub fn get_socket_path() -> Result<PathBuf> {
    Ok(PathBuf::from("/tmp/midimon.sock"))  // ❌ Vulnerable
}
```

#### After (SECURE)
```rust
pub fn get_socket_path() -> Result<PathBuf> {
    // Linux: $XDG_RUNTIME_DIR/midimon/midimon.sock
    // macOS: ~/Library/Application Support/midimon/run/midimon.sock
    // Windows: \\.\pipe\midimon

    let socket_dir = get_runtime_dir()?;

    // Create with 0700 permissions
    // Validate ownership
    // Enforce secure permissions

    Ok(socket_dir.join("midimon.sock"))
}
```

### 2. Runtime Directory Selection

New function `get_runtime_dir()` implements:
- **Linux**: XDG_RUNTIME_DIR → ~/.midimon/run fallback
- **macOS**: Application Support directory
- **Windows**: Not applicable (uses named pipes)

### 3. Security Validations

Added three security layers:

1. **Directory Creation**: Mode 0700 (owner-only)
2. **Ownership Check**: Verify UID matches current user
3. **Permission Enforcement**: Auto-fix insecure permissions

### 4. Dependencies

Added to `midimon-daemon/Cargo.toml`:
```toml
[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

### 5. Test Updates

Enhanced `test_get_socket_path()` to verify:
- Path NOT in `/tmp`
- User-specific directory
- Correct permissions (0700)
- Platform-specific paths

Added `test_get_runtime_dir()` for new function.

## Socket Paths by Platform

| Platform | Path | Security |
|----------|------|----------|
| Linux (systemd) | `/run/user/1000/midimon/midimon.sock` | Per-user tmpfs, 0700 |
| Linux (fallback) | `~/.midimon/run/midimon.sock` | User home, 0700 |
| macOS | `~/Library/Application Support/midimon/run/midimon.sock` | User dir, 0700 |
| Windows | `\\.\pipe\midimon` | OS-level isolation |

## Vulnerabilities Prevented

1. ✅ **Multi-user conflicts**: Each user has isolated socket
2. ✅ **Symlink attacks**: Ownership validation
3. ✅ **Race conditions**: Atomic directory creation
4. ✅ **Predictable paths**: User-specific paths
5. ✅ **Privilege escalation**: UID verification

## Testing

All tests pass:
```bash
cargo test --package midimon-daemon --lib
# Result: 52 passed; 0 failed; 1 ignored
```

Specific security tests:
- `test_get_socket_path()` - Validates secure paths
- `test_get_runtime_dir()` - Validates directory selection

## Backward Compatibility

**No breaking changes**:
- Old socket (`/tmp/midimon.sock`) is simply abandoned
- New daemon uses new path automatically
- `midimonctl` uses `get_socket_path()` function (updated)
- No migration required

## Code Review Checklist

- [x] No shared `/tmp` usage
- [x] User-specific directories
- [x] 0700 permissions enforced
- [x] Ownership validation
- [x] XDG specification compliance
- [x] Cross-platform support
- [x] Comprehensive tests
- [x] Documentation complete

## Files Modified

1. `midimon-daemon/src/daemon/state.rs`
   - Updated `get_socket_path()` (60 lines → 100 lines with docs)
   - Added `get_runtime_dir()` (new function, 30 lines)
   - Updated tests (25 lines → 80 lines)

2. `midimon-daemon/Cargo.toml`
   - Added `libc = "0.2"` for Unix platforms

3. Documentation (new files)
   - `SECURITY_SOCKET_ISOLATION.md` - Detailed security documentation
   - `SECURITY_FIX_SUMMARY.md` - This file

## Performance Impact

**Negligible**:
- Directory creation: One-time cost on first run
- Ownership check: Adds ~0.1ms to socket path resolution
- No impact on runtime performance

## Security Standards Compliance

- ✅ **XDG Base Directory Specification** (Linux)
- ✅ **CWE-377 Mitigation** (Insecure Temporary File)
- ✅ **OWASP Best Practices** (Secure file operations)
- ✅ **Principle of Least Privilege** (Owner-only access)

## Deployment Notes

**For users:**
- Automatic migration (no action required)
- Old socket can be manually removed: `rm -f /tmp/midimon.sock`

**For developers:**
- Run tests to verify: `cargo test --package midimon-daemon`
- Check socket path: See `SECURITY_SOCKET_ISOLATION.md`

## Next Steps (Optional Enhancements)

1. **Socket cleanup**: Remove socket on daemon exit
2. **Socket reuse**: Handle stale socket files
3. **Monitoring**: Log socket creation/validation
4. **Documentation**: Update user-facing docs with new paths

## References

- XDG Base Directory: https://specifications.freedesktop.org/basedir-spec/latest/
- CWE-377: https://cwe.mitre.org/data/definitions/377.html
- OWASP Temp Files: https://owasp.org/www-community/vulnerabilities/Insecure_Temporary_File
