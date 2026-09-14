# Audio Visualizer

A native Rust audio visualizer engine that captures system audio through PipeWire, performs real-time FFT analysis, converts the spectrum into logarithmic frequency bands, and applies smoothing for responsive visualization.


## Architecture

```text
PipeWire
   ↓
System Audio Capture
   ↓
Stereo → Mono
   ↓
FFT
   ↓
Frequency Bands
   ↓
Attack / Decay Smoothing
   ↓
VisualizerFrame
   ↓
Output / IPC
