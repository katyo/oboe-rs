use std::{
    marker::PhantomData,
    f32::consts::PI,
};

use apl::{
    EventHandler,
    Platform,
    AppConfig,
};

use sgl::{HasContext, Context};

use oboe::{
    DefaultStreamValues,
    AudioFeature,
    AudioDeviceInfo,
    AudioDeviceDirection,
    AudioStreamBuilder,
    PerformanceMode,
    SharingMode,
    Mono, Output,

    AudioStream,
    AudioOutputStream,
    AudioOutputCallback,
    AudioStreamAsync,
    DataCallbackResult,
};

pub struct Application<G: HasContext> {
    sine: Option<AudioStreamAsync<Output, SineWave>>,
    _phantom: PhantomData<G>,
}

impl<G: HasContext> Default for Application<G> {
    fn default() -> Self {
        Self {
            sine: None,
            _phantom: PhantomData,
        }
    }
}

impl<G: HasContext> EventHandler for Application<G> {
    type Context = G;

    fn resume(&mut self) {
        if let Err(error) = DefaultStreamValues::init() {
            eprintln!("Unable to init default stream values due to: {}", error);
        }

        println!("Default stream values:");
        println!("  Sample rate: {}", DefaultStreamValues::get_sample_rate());
        println!("  Frames per burst: {}", DefaultStreamValues::get_frames_per_burst());
        println!("  Channel count: {}", DefaultStreamValues::get_channel_count());

        println!("Audio features:");
        println!("  Low latency: {}", AudioFeature::LowLatency.has().unwrap());
        println!("  Output: {}", AudioFeature::Output.has().unwrap());
        println!("  Pro: {}", AudioFeature::Pro.has().unwrap());
        println!("  Microphone: {}", AudioFeature::Microphone.has().unwrap());
        println!("  Midi: {}", AudioFeature::Midi.has().unwrap());

        let devices = AudioDeviceInfo::request(AudioDeviceDirection::InputOutput)
            .unwrap();

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

        if self.sine.is_none() {
            let mut sine = AudioStreamBuilder::default()
                .set_performance_mode(PerformanceMode::LowLatency)
                .set_sharing_mode(SharingMode::Shared)
                .set_format::<f32>()
                .set_channel_count::<Mono>()
                .set_callback(SineWave::default())
                .open_stream()
                .unwrap();

            println!("{:?}", sine);

            sine.start().unwrap();
            self.sine = sine.into();
        }
    }

    fn suspend(&mut self) {
        if let Some(sine) = &mut self.sine {
            sine.stop().unwrap();
            self.sine = None;
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

    fn on_audio_ready(&mut self, stream: &mut dyn AudioOutputStream, frames: &mut [f32]) -> DataCallbackResult {
        if self.delta.is_none() {
            let sample_rate = stream.get_sample_rate() as f32;
            self.delta = (self.frequency * 2.0 * PI / sample_rate).into();
            println!("Prepare sine wave generator: samplerate={}, time delta={}", sample_rate, self.delta.unwrap());
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

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    println!("Starting oboe-rs demo...");

    /*DefaultStreamValues::request_from_properties();

    println!("Default stream values:");
    println!("  Sample rate: {}", DefaultStreamValues::get_sample_rate());
    println!("  Frames per burst: {}", DefaultStreamValues::get_frames_per_burst());
    println!("  Channel count: {}", DefaultStreamValues::get_channel_count());*/

    let config = AppConfig {
        title: "OBOE Demo".into(),
        icon: None,
    };

    let platform = Platform::new(config);

    let application = Application::<Context>::default();

    platform.run(application);
}
