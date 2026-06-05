# Changelog

## [0.5.3] - 2026-06-04

### Fixed
- App no longer crashes on Linux when a global shortcut is already registered by the system — each shortcut is now registered individually and failures are silently skipped

### Known Issues
- Global keyboard shortcuts (Ctrl+/-, Ctrl+0, F11) may not work on Linux depending on the desktop environment — use the system tray icon instead
- Ubuntu/Debian users must use the `.deb` installer — the `.AppImage` has rendering issues on Ubuntu

## [0.5.2] - 2026-06-04

### Added
- Frosted glass card on splash screen with iris reveal animation
- Staggered fade-in for logo, title, tagline and button on splash screen
- Card collapses into divider on transition to setup form (CRT-style)
- F11 fullscreen toggle via global shortcut
- Beginner-friendly tooltip rewrites for Protocol, Address and Port fields
- Star count increased to 180 for denser constellation background

### Changed
- Zoom debounce improved to prevent key-repeat issues
- Static text elements (title, tagline) no longer show text cursor on hover
- Tooltip language simplified — technical terms replaced with plain language

### Fixed
- **Critical:** Tray icon loaded via relative path causing immediate crash in production builds
- package.json version updated to match app version

## [0.5.0] - 2026-06-04

### Added
- Animated splash screen with constellation background
- Choreographed two-column setup form with growing divider and slide-in animation
- Silent connection probing on startup and after navigation — no WebView2 error page flash
- Mid-session health checks (10s in dev, 60s in production)
- 502/503 status code detection for Odysseus instances behind Nginx Proxy Manager
- Reconnect screen on connection dropout with "Attempt to reconnect" and "change address" options
- Distinct error messages for first-time connection failure vs mid-session disconnect
- OS-aware tooltips on Protocol, Address and Port fields
- System tray icon with Zoom In, Zoom Out, Reset Zoom, Disconnect and Quit options
- Global keyboard shortcuts: Ctrl+= (zoom in), Ctrl+- (zoom out), Ctrl+0 (reset zoom), F11 (fullscreen)
- Zoom debounce to prevent key-repeat double firing
- Legal disclaimer pinned to bottom of splash screen
- Color palette matched to Odysseus default theme

### Changed
- Complete UI redesign — replaced single URL input with structured protocol/address/port form
- Protocol dropdown (http/https) with tooltip explaining when to use each
- Port field is optional — leave blank for reverse proxy setups, defaults to 7000 for direct connections
- "Set sail" button disabled until address field is populated
- Error banner uses distinct red color separate from the accent palette

### Fixed
- WebView2 error pages no longer flash before returning to setup screen
- App no longer navigates to saved URL on startup without first verifying it is reachable

## [0.1.1] - 2024-01-01

### Added
- Initial release
- Basic URL input and navigation to Odysseus instance
- Saved URL persistence across app restarts
