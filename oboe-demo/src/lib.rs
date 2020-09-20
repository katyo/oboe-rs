#[cfg(feature = "audio")]
mod audio;
mod window;

#[cfg(feature = "audio")]
use audio::*;
use window::*;

#[derive(Default)]
struct DemoApp {
    #[cfg(feature = "audio")]
    sine: SineGen,
}

impl App for DemoApp {
    fn start(&mut self) {
        #[cfg(feature = "audio")]
        {
            audio_probe();
            self.sine.try_start();
        }
    }
    fn stop(&mut self) {
        #[cfg(feature = "audio")]
        {
            self.sine.try_stop();
        }
    }
}

#[ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "rust-oboe-demo"))]
fn main() {
    println!("Starting oboe-rs demo...");
    log::info!("Starting rust oboe demo...");

    run(DemoApp::default());

    println!("Stopped rust oboe demo...");
    log::info!("Stopped rust oboe demo...");
}
