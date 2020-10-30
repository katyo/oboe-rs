use std::f32::consts::PI;

use oboe::{
    AudioDeviceDirection, AudioDeviceInfo, AudioFeature, AudioOutputCallback, AudioOutputStream,
    AudioOutputStreamSafe, AudioStream, AudioStreamAsync, AudioStreamBuilder, DataCallbackResult,
    DefaultStreamValues, Mono, Output, PerformanceMode, SharingMode,
};

/// Sine-wave generator stream
#[derive(Default)]
pub struct SineGen {
    stream: Option<AudioStreamAsync<Output, SineWave>>,
}

impl SineGen {
    /// Create and start audio stream
    pub fn try_start(&mut self) {
        if self.stream.is_none() {
            let mut stream = AudioStreamBuilder::default()
                .set_performance_mode(PerformanceMode::LowLatency)
                .set_sharing_mode(SharingMode::Shared)
                .set_format::<f32>()
                .set_channel_count::<Mono>()
                .set_callback(SineWave::default())
                .open_stream()
                .unwrap();

            log::debug!("start stream: {:?}", stream);

            stream.start().unwrap();

            self.stream = Some(stream);
        }
    }

    /// Pause audio stream
    pub fn try_pause(&mut self) {
        if let Some(stream) = &mut self.stream {
            log::debug!("pause stream: {:?}", stream);
            stream.pause().unwrap();
        }
    }

    /// Stop and remove audio stream
    pub fn try_stop(&mut self) {
        if let Some(stream) = &mut self.stream {
            log::debug!("stop stream: {:?}", stream);
            stream.stop().unwrap();
            self.stream = None;
        }
    }
}

pub struct SineWave {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: Option<f32>,
}

impl Default for SineWave {
    fn default() -> Self {
        println!("init SineWave generator");
        Self {
            frequency: 440.0,
            gain: 0.5,
            phase: 0.0,
            delta: None,
        }
    }
}

impl Drop for SineWave {
    fn drop(&mut self) {
        println!("drop SineWave generator");
    }
}

impl AudioOutputCallback for SineWave {
    type FrameType = (f32, Mono);

    fn on_audio_ready(
        &mut self,
        stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [f32],
    ) -> DataCallbackResult {
        if self.delta.is_none() {
            let sample_rate = stream.get_sample_rate() as f32;
            self.delta = (self.frequency * 2.0 * PI / sample_rate).into();
            println!(
                "Prepare sine wave generator: samplerate={}, time delta={}",
                sample_rate,
                self.delta.unwrap()
            );
        }

        let delta = self.delta.unwrap();

        for frame in frames {
            *frame = self.gain * self.phase.sin();
            self.phase += delta;
            while self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }
        }
        DataCallbackResult::Continue
    }
}

/// Print device's audio info
pub fn audio_probe() {
    if let Err(error) = DefaultStreamValues::init() {
        eprintln!("Unable to init default stream values due to: {}", error);
    }

    println!("Default stream values:");
    println!("  Sample rate: {}", DefaultStreamValues::get_sample_rate());
    println!(
        "  Frames per burst: {}",
        DefaultStreamValues::get_frames_per_burst()
    );
    println!(
        "  Channel count: {}",
        DefaultStreamValues::get_channel_count()
    );

    println!("Audio features:");
    println!("  Low latency: {}", AudioFeature::LowLatency.has().unwrap());
    println!("  Output: {}", AudioFeature::Output.has().unwrap());
    println!("  Pro: {}", AudioFeature::Pro.has().unwrap());
    println!("  Microphone: {}", AudioFeature::Microphone.has().unwrap());
    println!("  Midi: {}", AudioFeature::Midi.has().unwrap());

    let devices = AudioDeviceInfo::request(AudioDeviceDirection::InputOutput).unwrap();

    println!("Audio Devices:");

    for device in devices {
        println!("{{");
        println!("  Id: {}", device.id);
        println!("  Type: {:?}", device.device_type);
        println!("  Direction: {:?}", device.direction);
        println!("  Address: {}", device.address);
        println!("  Product name: {}", device.product_name);
        println!("  Channel counts: {:?}", device.channel_counts);
        println!("  Sample rates: {:?}", device.sample_rates);
        println!("  Formats: {:?}", device.formats);
        println!("}}");
    }
}
