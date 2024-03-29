use atomic_float::AtomicF32;
use std::{
    f32::consts::PI,
    marker::PhantomData,
    sync::{atomic::Ordering, Arc},
};

use oboe::{
    AudioDeviceDirection, AudioDeviceInfo, AudioFeature, AudioOutputCallback, AudioOutputStream,
    AudioOutputStreamSafe, AudioStream, AudioStreamAsync, AudioStreamBase, AudioStreamBuilder,
    AudioStreamSafe, DataCallbackResult, DefaultStreamValues, Mono, Output, PerformanceMode,
    Result, SharingMode, Stereo, StreamState,
};

pub enum GenState {
    Started,
    Paused,
    Stopped,
}

/// Sine-wave generator stream
#[derive(Default)]
pub struct SineGen {
    stream: Option<AudioStreamAsync<Output, SineWave<f32, Mono>>>,
    param: Arc<SineParam>,
}

impl core::ops::Deref for SineGen {
    type Target = SineParam;

    fn deref(&self) -> &Self::Target {
        &self.param
    }
}

impl SineGen {
    pub fn state(&self) -> GenState {
        if let Some(stream) = &self.stream {
            if matches!(stream.get_state(), StreamState::Paused) {
                GenState::Paused
            } else {
                GenState::Started
            }
        } else {
            GenState::Stopped
        }
    }

    /// Create and start audio stream
    pub fn start(&mut self) -> Result<()> {
        if let Some(stream) = &mut self.stream {
            stream.start()?;
        } else {
            let mut stream = AudioStreamBuilder::default()
                .set_performance_mode(PerformanceMode::LowLatency)
                .set_sharing_mode(SharingMode::Shared)
                .set_format::<f32>()
                .set_channel_count::<Mono>()
                .set_callback(SineWave::<f32, Mono>::new(&self.param))
                .open_stream()?;

            log::debug!("start stream: {:?}", stream);

            self.param.set_sample_rate(stream.get_sample_rate() as _);

            stream.start()?;

            self.stream = Some(stream);
        }

        Ok(())
    }

    /// Pause audio stream
    pub fn pause(&mut self) -> Result<()> {
        if let Some(stream) = &mut self.stream {
            log::debug!("pause stream: {:?}", stream);
            stream.pause()?;
        }
        Ok(())
    }

    /// Stop and remove audio stream
    pub fn stop(&mut self) -> Result<()> {
        if let Some(stream) = &mut self.stream {
            log::debug!("stop stream: {:?}", stream);
            stream.stop()?;
            self.stream = None;
        }
        Ok(())
    }
}

pub struct SineParam {
    frequency: AtomicF32,
    gain: AtomicF32,
    sample_rate: AtomicF32,
    delta: AtomicF32,
}

impl Default for SineParam {
    fn default() -> Self {
        Self {
            frequency: AtomicF32::new(440.0),
            gain: AtomicF32::new(0.5),
            sample_rate: AtomicF32::new(0.0),
            delta: AtomicF32::new(0.0),
        }
    }
}

impl SineParam {
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate.load(Ordering::SeqCst)
    }

    pub fn set_sample_rate(&self, sample_rate: f32) {
        let frequency = self.frequency.load(Ordering::Acquire);
        let delta = frequency * 2.0 * PI / sample_rate;

        self.delta.store(delta, Ordering::Release);
        self.sample_rate.store(sample_rate, Ordering::SeqCst);

        println!("Prepare sine wave generator: samplerate={sample_rate}, time delta={delta}");
    }

    pub fn frequency(&self) -> f32 {
        self.frequency.load(Ordering::SeqCst)
    }

    pub fn set_frequency(&self, frequency: f32) {
        let sample_rate = self.sample_rate.load(Ordering::SeqCst);
        let delta = frequency * 2.0 * PI / sample_rate;

        self.delta.store(delta, Ordering::SeqCst);
        self.frequency.store(frequency, Ordering::SeqCst);
    }

    pub fn gain(&self) -> f32 {
        self.gain.load(Ordering::SeqCst)
    }

    pub fn set_gain(&self, gain: f32) {
        self.gain.store(gain, Ordering::SeqCst);
    }
}

pub struct SineWave<F, C> {
    param: Arc<SineParam>,
    phase: f32,
    marker: PhantomData<(F, C)>,
}

impl<F, C> Drop for SineWave<F, C> {
    fn drop(&mut self) {
        println!("drop SineWave generator");
    }
}

impl<F, C> SineWave<F, C> {
    pub fn new(param: &Arc<SineParam>) -> Self {
        println!("init SineWave generator");
        Self {
            param: param.clone(),
            phase: 0.0,
            marker: PhantomData,
        }
    }
}

impl<F, C> Iterator for SineWave<F, C> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let delta = self.param.delta.load(Ordering::Relaxed);
        let gain = self.param.gain.load(Ordering::Relaxed);

        let frame = gain * self.phase.sin();

        self.phase += delta;
        while self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        Some(frame)
    }
}

impl AudioOutputCallback for SineWave<f32, Mono> {
    type FrameType = (f32, Mono);

    fn on_audio_ready(
        &mut self,
        _stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [f32],
    ) -> DataCallbackResult {
        for frame in frames {
            *frame = self.next().unwrap();
        }
        DataCallbackResult::Continue
    }
}

impl AudioOutputCallback for SineWave<f32, Stereo> {
    type FrameType = (f32, Stereo);

    fn on_audio_ready(
        &mut self,
        _stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [(f32, f32)],
    ) -> DataCallbackResult {
        for frame in frames {
            frame.0 = self.next().unwrap();
            frame.1 = frame.0;
        }
        DataCallbackResult::Continue
    }
}

/// Print device's audio info
pub fn print_info() {
    if let Err(error) = DefaultStreamValues::init() {
        eprintln!("Unable to init default stream values due to: {error}");
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
