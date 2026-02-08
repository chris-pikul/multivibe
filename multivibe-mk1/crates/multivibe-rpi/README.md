# Multivibe RPi Module

This crate acts as the "Hardware Abstraction Layer" for the Pi. It is only
compiled when the puck feature is active. It handles the low-level "Heavy Lifting"
required to turn a Pi into a real-time appliance.

- ALSA Integration: Direct interface with the PCM1808 ADC via I2S.
- Real-time Scheduling: Utilizes audio_thread_priority and affinity to pin threads to isolated CPU cores.
- GPIO Management: Logic for physical buttons (Power) and switches (WiFi Mode).
