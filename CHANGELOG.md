# Changelog

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
