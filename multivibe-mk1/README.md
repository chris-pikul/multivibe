# Multivibe Mk.I

**Multivibe Mk.I** is a low-latency, "hacker-grade" audio appliance designed to
bridge the gap between a TV and your neighbors' (or friends') ears. It captures
high-fidelity analog or digital audio and broadcasts it over a local Wi-Fi network,
allowing anyone in the room to listen privately on their own smartphones via a
web-based interface.

Built as a dedicated "Puck," the Mk.I utilizes the **Raspberry Pi Zero 2 WH** and
a high-resolution **I2S ADC** to provide a real-time, high-priority streaming
experience that out-performs generic consumer solutions.

## 🛠 Features

### Current (Mk.I)

- **Dual Input Support:** High-resolution stereo input via **3.5mm Aux** or **RCA** (utilizing a PCM1808 I2S ADC).
- **Ultra-Low Latency:** Real-time audio capture loop pinned to isolated CPU cores using Rust's `SCHED_FIFO` priorities.
- **Web-Native Listening:** No apps required. Users connect to the local "Multivibe" network and listen through a modern web browser.
- **Dual-Mode Networking:**
    - **Station Mode:** Connects to your existing home network.
    - **Access Point (AP) Mode:** Creates its own Wi-Fi network for "standalone" use.
- **Single-Binary Appliance:** Entire OS (Alpine Linux), server, and UI are bundled into a single, robust executable.
- **"Diskless" Operation:** Runs entirely from RAM to prevent SD card corruption from sudden power loss (e.g., when the TV turns off).

### Planned (Mk.II & Future)

- **Standalone Desktop Client:** Virtual audio routing for Mac/PC users to stream system audio directly without the hardware Puck.
- **ESP32 Port:** A ultra-low-cost Mk.II version focused on even smaller form factors and reduced power consumption.
- **Integrated Messaging:** A lightweight, localized chat system bundled into the listening interface for "silent movie nights."

## 🏗 Project Architecture

This repository is managed as a **Cargo Workspace** to allow for clean separation
between hardware-specific drivers and the core audio engine.

- `bin/multivibe-server`: The main application. Orchestrates the web server (Axum) and ties the audio core to the network.
- `crates/multivibe-core`: The hardware-agnostic "Brain." Handles PCM processing, buffer logic, and messaging protocols.
- `crates/multivibe-rpi`: Mk.I specific drivers. Manages ALSA, I2S clocks, and GPIO pin interactions for the Raspberry Pi.

## 🚀 Development Plan

### Phase 1: The Foundations (Current)

- [x] Project naming and repository scaffolding.
- [x] Selection of the PCM1808 ADC (ASHATA) hardware.
- [x] Implementation of Feature Flags for `puck` vs. `software` releases.
- [ ] Define the `AudioSource` trait for cross-platform simulation.

### Phase 2: Simulation & Core Logic

- [ ] Develop "Software Mode" using **BlackHole** (macOS) to simulate TV input.
- [ ] Build the high-priority UDP/WebSocket broadcast engine in Rust.
- [ ] Create the "Hardware Mock" layer to simulate GPIO buttons and switches on dev machines.

### Phase 3: Hardware Integration

- [ ] Assemble the Pi Zero 2 WH and ASHATA ADC (solderless "Puck" assembly).
- [ ] Configure **Alpine Linux** for diskless, read-only operation.
- [ ] Fine-tune CPU affinity and thread scheduling on the ARMv7 target.

### Phase 4: UI & Polishing

- [ ] Build the mobile-first listening interface (bundled into the binary).
- [ ] Implement the "WiFi Mode" toggle and Graceful Shutdown via physical buttons.
- [ ] Final stress testing with multiple concurrent listeners.

## 💻 Development Environment

To work on this project without the hardware "Puck," you can run in software mode
on macOS:

1. **Install Audio Loopback:** `brew install blackhole-2ch`
2. **Configure Output:** Set Mac output to a Multi-Output Device (Speakers + BlackHole).
3. **Run Development Server:**

```bash
cargo run -p multivibe-server --features software
```

## 📄 License

Licensed under **Creative Commons Attribution-NonCommercial 4.0 International (CC-BY-NC-4.0)**.
_You are free to share and adapt this for personal use. Commercial use or mass production without explicit permission is prohibited._
