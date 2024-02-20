mod audio;

use eframe::{egui, NativeOptions};

use audio::{print_info, GenState, SineGen};

#[cfg(target_os = "android")]
use egui_winit::winit;
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use eframe::Renderer;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    std::env::set_var("RUST_BACKTRACE", "full");
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = NativeOptions {
        event_loop_builder: Some(Box::new(|builder| {
            builder.with_android_app(app);
        })),
        renderer: Renderer::Wgpu,
        ..Default::default()
    };
    DemoApp::run(options).unwrap();
}

#[derive(Default)]
pub struct DemoApp {
    pub gen: SineGen,
    pub error: Option<String>,
}

impl DemoApp {
    pub fn run(options: NativeOptions) -> Result<(), eframe::Error> {
        print_info();

        eframe::run_native(
            "egui-android-demo",
            options,
            Box::new(|_cc| Box::<DemoApp>::default()),
        )
    }

    pub fn result(&mut self, result: oboe::Result<()>) {
        self.error = result.err().map(|err| err.to_string());
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new("oboe-rs: Sine-wave generator").heading());

            ui.separator();

            ui.label("Start/stop generator:");

            ui.horizontal(|ui| {
                let state = self.gen.state();

                if ui.add_enabled(!matches!(state, GenState::Started), egui::Button::new("Start")).clicked() {
                    let res = self.gen.start();
                    self.result(res);
                }

                if ui.add_enabled(!matches!(state, GenState::Stopped), egui::Button::new("Stop")).clicked() {
                    let res = self.gen.stop();
                    self.result(res);
                }

                if ui.add_enabled(matches!(state, GenState::Started), egui::Button::new("Pause")).clicked() {
                    let res = self.gen.pause();
                    self.result(res);
                }
            });

            ui.separator();

            ui.label("You can select frequency:");

            ui.horizontal(|ui| {
                if ui.button("100 Hz").clicked() {
                    self.gen.set_frequency(100.0);
                }
                if ui.button("440 Hz").clicked() {
                    self.gen.set_frequency(440.0);
                }
                if ui.button("1 KHz").clicked() {
                    self.gen.set_frequency(1000.0);
                }
            });

            ui.label("Also you can change frequency:");

            ui.add(egui::Slider::from_get_set(50.0..=5000.0, |set: Option<f64>| {
                if let Some(freq) = set {
                    self.gen.set_frequency(freq as _);
                }
                self.gen.frequency() as _
            }).text("Frequency"));

            ui.separator();

            ui.label("You can adjust gain.");

            ui.add(egui::Slider::from_get_set(0.0..=1.0, |set: Option<f64>| {
                if let Some(gain) = set {
                    self.gen.set_gain(gain as _);
                }
                self.gen.gain() as _
            }).text("Gain"));

            ui.separator();

            ui.label(if let Some(error) = &self.error {
                egui::RichText::new(error).color(egui::Color32::RED)
            } else {
                egui::RichText::new("OK").color(egui::Color32::GREEN)
            });

            ui.separator();

            ui.label("If you don't hear the sound or something looks broken, please report the problem here:");
            ui.hyperlink("https://github.com/katyo/oboe-rs/issues");

            ui.separator();

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
        });
    }
}
