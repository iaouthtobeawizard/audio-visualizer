use anyhow::{Context, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, Stream, StreamConfig,
};
use std::sync::mpsc::{self, Receiver, Sender};

pub struct AudioCapture {
    stream: Stream,
    receiver: Receiver<Vec<f32>>,
    sample_rate: f32,
}

impl AudioCapture {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .context("No audio input device")?;
        let supported = device
            .default_input_config()
            .context("failed to get default input config")?;
        let config: StreamConfig = supported.config();
        let sample_rate = config.sample_rate as f32;
        let sample_format = supported.sample_format();
        let (sender, receiver) = mpsc::channel();
        let stream = match sample_format {
            SampleFormat::F32 => Self::build_stream::<f32>(&device, &config, sender)?,
            SampleFormat::I16 => Self::build_stream::<i16>(&device, &config, sender)?,
            SampleFormat::U16 => Self::build_stream::<u16>(&device, &config, sender)?,
            format => {
                anyhow::bail!("unsupported audio format: {format:?}");
            }
        };
        stream.play().context("failed to start audio stream")?;
        Ok(Self {
            stream,
            receiver,
            sample_rate,
        })
    }

    fn build_stream<T>(
        device: &cpal::Device,
        config: &StreamConfig,
        sender: Sender<Vec<f32>>,
    ) -> Result<Stream>
    where
        T: cpal::SizedSample + cpal::Sample,
        f32: cpal::FromSample<T>,
    {
        let channels = config.channels as usize;
        let stream = device.build_input_stream(
            config.clone(),
            move |data: &[T], _| {
                let samples: Vec<f32> = data
                    .chunks(channels)
                    .map(|frame| {
                        frame
                            .iter()
                            .map(|sample| (*sample).to_sample::<f32>())
                            .sum::<f32>()
                            / channels as f32
                    })
                    .collect();
                let _ = sender.send(samples);
            },
            |error| {
                eprintln!("Audio strem error: {error}");
            },
            None,
        )?;
        Ok(stream)
    }
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
    pub fn recv(&self) -> Result<Vec<f32>> {
        self.receiver.recv().context("Audio Capture channel closed")
    }
}
