# Multivibe Server

The final binary that runs on the device. It glues the core logic to a web-accessible
interface. It is designed to be "Diskless Alpine" friendly, meaning it bundles
everything it needs into a single static executable.

- Axum Web Server: Handles the HTTP API for settings and the WebSocket server for neighbors.
- UI Bundling: Uses include_dir to bake the frontend assets (HTML/JS/CSS) directly into the binary.
- Mode Switching: Managed via Cargo features (puck vs. software) to allow development on macOS while targeting Linux/ARMv7.
