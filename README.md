# Audio Visualizer

A native Rust audio visualizer engine designed to provide real-time frequency
analysis for desktop applications.

The project is standalone and does not depend on any particular UI framework.

## Architecture

```text
Audio Source
     │
     ▼
Audio Capture
     │
     ▼
    PCM
     │
     ▼
    FFT
     │
     ▼
Frequency Bands
     │
     ▼
Smoothing
     │
     ▼
VisualizerFrame
     │
     ├── Terminal
     ├── IPC
     └── Other clients
