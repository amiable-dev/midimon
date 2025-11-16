# Security Fix: User-Specific Socket Paths

## Vulnerability Fixed

**MEDIUM Severity**: IPC socket in shared /tmp directory vulnerable on multi-user systems

### Previous Implementation (INSECURE)

```rust
// VULNERABLE CODE (Fixed in this commit)
pub fn get_socket_path() -> Result<PathBuf> {
    Ok(PathBuf::from("/tmp/midimon.sock"))  // ❌ INSECURE
}
```

### Security Issues with /tmp

1. **Multi-user conflicts**: Any user can create `/tmp/midimon.sock` first
2. **Symlink attacks**: Attacker creates symlink pointing to important file
3. **Race conditions**: Time-of-check to time-of-use vulnerabilities
4. **No user isolation**: All users share the same socket path
5. **Predictable paths**: Easy for attackers to target

## New Implementation (SECURE)

### Socket Path Selection Strategy

#### Linux
1. **XDG_RUNTIME_DIR/midimon/midimon.sock** (preferred)
   - Per-user tmpfs managed by systemd
   - Mode 0700 by default (owner-only)
   - Automatically cleaned on logout
   - Example: `/run/user/1000/midimon/midimon.sock`

2. **~/.midimon/run/midimon.sock** (fallback)
   - User home directory
   - Created with mode 0700
   - Persists across reboots

#### macOS
- **~/Library/Application Support/midimon/run/midimon.sock**
  - Platform convention for application data
  - User-specific directory
  - Created with mode 0700

#### Windows
- **\\.\pipe\midimon**
  - Named pipes are user-namespaced by OS
  - Inherent per-user isolation

### Security Features Implemented

#### 1. User Isolation
```rust
// Linux: XDG_RUNTIME_DIR is per-user tmpfs
if let Ok(xdg_runtime) = std::env::var("XDG_RUNTIME_DIR") {
    let dir = PathBuf::from(xdg_runtime).join("midimon");
    return Ok(dir);
}
```

#### 2. Secure Directory Permissions
```rust
// Create with owner-only permissions (0700)
std::fs::create_dir_all(&socket_dir)?;
let perms = std::fs::Permissions::from_mode(0o700);
std::fs::set_permissions(&socket_dir, perms)?;
```

#### 3. Ownership Validation
```rust
// Verify directory is owned by current user
let current_uid = unsafe { libc::getuid() };
let dir_uid = metadata.uid();

if dir_uid != current_uid {
    return Err(DaemonError::StatePersistence(
        "Directory owned by different user - security risk"
    ));
}
```

#### 4. Permission Enforcement
```rust
// Detect and fix insecure permissions
let mode = metadata.mode();
let world_perms = mode & 0o007;  // World: rwx
let group_perms = mode & 0o070;  // Group: rwx

if world_perms != 0 || group_perms != 0 {
    // Fix to owner-only (0700)
    perms.set_mode(0o700);
    std::fs::set_permissions(&socket_dir, perms)?;
}
```

## Attack Scenarios Prevented

### 1. Symlink Attack
**Before**: Attacker creates `/tmp/midimon.sock` as symlink to `/etc/passwd`
**After**: Socket is in user-specific directory, attacker cannot access

### 2. Socket Hijacking
**Before**: User A creates socket, User B starts daemon and overwrites
**After**: Each user has isolated socket in their own directory

### 3. Race Condition
**Before**: Check socket doesn't exist → attacker creates it → daemon uses it
**After**: Ownership validation prevents using attacker-controlled directory

### 4. Privilege Escalation
**Before**: Root daemon uses `/tmp/midimon.sock` created by attacker
**After**: Ownership check fails, daemon refuses to start

## Testing

### Automated Tests
```bash
cargo test --package midimon-daemon daemon::state::tests
```

Tests verify:
- Socket path is NOT in `/tmp`
- Directory has 0700 permissions
- Path is user-specific (XDG_RUNTIME_DIR or home directory)

### Manual Verification

#### Linux
```bash
# Check XDG_RUNTIME_DIR (should be /run/user/$UID)
echo $XDG_RUNTIME_DIR

# Start daemon and verify socket location
./target/release/midimon
ls -la $XDG_RUNTIME_DIR/midimon/

# Expected output:
# drwx------ 2 user user  60 Nov 16 12:00 .
# srwxr-xr-x 1 user user   0 Nov 16 12:00 midimon.sock
```

#### macOS
```bash
# Start daemon
./target/release/midimon

# Verify socket location
ls -la ~/Library/Application\ Support/midimon/run/

# Expected output:
# drwx------ 2 user staff  64 Nov 16 12:00 .
# srwxr-xr-x 1 user staff   0 Nov 16 12:00 midimon.sock
```

## XDG Base Directory Specification

The implementation follows the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/latest/):

- **XDG_RUNTIME_DIR**: Per-user directory for runtime files
- **Ownership**: Must be owned by the user, mode 0700
- **Lifetime**: Cleaned on user logout
- **Storage**: tmpfs (not persistent across reboots)

## Backward Compatibility

### Migration Path

Existing installations using `/tmp/midimon.sock` will:
1. Start daemon with new socket path
2. Old socket file remains unused
3. Client tools (`midimonctl`) automatically use new path
4. No manual intervention required

### Cleanup Old Socket (Optional)

```bash
# Remove old socket (optional, won't interfere)
rm -f /tmp/midimon.sock
rm -rf /tmp/midimon/
```

## Dependencies

Added `libc` crate for Unix UID checking:

```toml
# Unix system calls (for permission checking)
[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

## Platform Support

| Platform | Socket Path | Security |
|----------|-------------|----------|
| Linux (systemd) | `$XDG_RUNTIME_DIR/midimon/midimon.sock` | ✅ Per-user tmpfs, mode 0700 |
| Linux (fallback) | `~/.midimon/run/midimon.sock` | ✅ User home, mode 0700 |
| macOS | `~/Library/Application Support/midimon/run/midimon.sock` | ✅ User dir, mode 0700 |
| Windows | `\\.\pipe\midimon` | ✅ OS-level user isolation |

## Security Checklist

- [x] Socket NOT in shared `/tmp` directory
- [x] User-specific directory path
- [x] Directory permissions 0700 (owner-only)
- [x] Ownership validation on existing directories
- [x] Permission enforcement/fixing
- [x] XDG Base Directory Specification compliance
- [x] Platform-specific secure defaults
- [x] Comprehensive test coverage
- [x] Clear error messages

## References

- [CWE-377: Insecure Temporary File](https://cwe.mitre.org/data/definitions/377.html)
- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/latest/)
- [OWASP: Insecure Temporary Files](https://owasp.org/www-community/vulnerabilities/Insecure_Temporary_File)
- [Linux systemd: User Runtime Directory](https://www.freedesktop.org/software/systemd/man/pam_systemd.html)

## Code Location

File: `/Users/christopherjoseph/projects/amiable/midimon/midimon-daemon/src/daemon/state.rs`

Functions:
- `get_socket_path()` - Main entry point for socket path retrieval
- `get_runtime_dir()` - Platform-specific runtime directory selection
