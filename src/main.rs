use molesim_lib::{Molecules, HEIGHT, WIDTH};
use eframe::egui::{self};

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH as f32, HEIGHT as f32)),
        ..Default::default()
    };
    eframe::run_native(
        "Molecule Simulator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    molecules: Molecules,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            molecules: Molecules::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.molecules.next();
            ui.heading("Molecule Simulator");
            let painter = ui.painter();
            for molecule in &self.molecules.molecules {
                painter.add(egui::Shape::circle_filled(
                    egui::pos2(molecule.position.x as f32, molecule.position.y as f32),
                    3.0,
                    egui::Color32::from_rgb(255, 255, 255),
                ));
            }
        });
        ctx.request_repaint();
    }
}
