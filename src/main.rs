use molesim_lib::{Molecules, HEIGHT, WIDTH, MOLECULE_RADIUS};
use eframe::egui::{self};

use std::sync::{Arc, Mutex};
use std::thread::spawn;

const MOLECULE_DISPLAY_RADIUS: f64 = 5.0;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH as f32, HEIGHT as f32)),
        ..Default::default()
    };
    let mut sim = Arc::new(Mutex::new(Molecules::new()));

    let sim_clone = sim.clone();

    spawn(move || {
        loop {
            let mut handle = sim_clone.lock().unwrap();
            handle.next();
        }
    });

    eframe::run_native(
        "Molecule Simulator",
        options,
        Box::new(move |_cc| Box::new(MyApp {
            simulation: sim
        })),
    )
}

struct MyApp {
    simulation: Arc<Mutex<Molecules>>, // ! used to be called 'molecules'
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Molecule Simulator");
            let painter = ui.painter();
            let handle = self.simulation.lock().unwrap();
            for molecule in &handle.molecules {
                    painter.add(egui::Shape::circle_filled(
                        egui::pos2(molecule.position.x as f32, molecule.position.y as f32),
                        MOLECULE_DISPLAY_RADIUS as f32,
                        egui::Color32::from_rgb(255, 255, 255),
                    ));
            }
        });
        ctx.request_repaint();
    }
}
