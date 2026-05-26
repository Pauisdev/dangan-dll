#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{EventLoopBuilderHook, egui};
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::sync::Mutex;
use winit::platform::windows::EventLoopBuilderExtWindows;

use crate::FUNCS;
use crate::Vec3;

pub static APP: Lazy<MyApp> = Lazy::new(|| MyApp {
    position: Arc::new(Mutex::new(Vec3::one())),
});

pub fn show() -> eframe::Result {
    let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(|event_loop_builder| {
        event_loop_builder.with_any_thread(true);
    }));
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([200.0, 100.0]),
        event_loop_builder,
        ..Default::default()
    };
    let app = MyApp {
        position: APP.position.clone(),
    };
    eframe::run_native("Mod helper", options, Box::new(|_cc| Ok(Box::new(app))))
}

pub struct MyApp {
    pub position: Arc<Mutex<Vec3>>,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Position");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut self.position.lock().unwrap().x));
                ui.label("x");
                ui.add(egui::DragValue::new(&mut self.position.lock().unwrap().y));
                ui.label("y");
                ui.add(egui::DragValue::new(&mut self.position.lock().unwrap().z));
                ui.label("z");
            });

            if ui.button("Reset position").clicked() {
                let funcs = FUNCS.lock().unwrap();
                if funcs.set_player_pos.is_some() {
                    let set_player_pos = funcs.set_player_pos.unwrap();
                    set_player_pos(0.0, 0.0, -263.0, 0.0)
                }
            }
            ui.request_repaint_after_secs(1.0);
        });
    }
}
