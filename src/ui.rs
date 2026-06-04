#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::Vec3;
use crate::dangan_one::BASE_ADDRESS;
use crate::dangan_one::dr_funcs;
use eframe::{EventLoopBuilderHook, egui};
use winit::platform::windows::EventLoopBuilderExtWindows;

pub fn show() -> eframe::Result {
    let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(|event_loop_builder| {
        event_loop_builder.with_any_thread(true);
    }));
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([200.0, 100.0]),
        event_loop_builder,
        ..Default::default()
    };
    eframe::run_native(
        "Mod helper",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp {}))),
    )
}

pub struct MyApp {}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let pos_address = *BASE_ADDRESS + 0x33cc80;
            let position_ptr = pos_address as *mut Vec3;
            let mut position = unsafe { *position_ptr };

            ui.label("Position");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut position.x));
                ui.label("x");
                ui.add(egui::DragValue::new(&mut position.y));
                ui.label("y");
                ui.add(egui::DragValue::new(&mut position.z));
                ui.label("z");
            });

            if ui.button("Reset position").clicked() {
                dr_funcs::set_player_pos(0.0, -263.0, 1.0, 0.0);
            }
            ui.request_repaint_after_secs(1.0);
        });
    }
}

pub fn spawn() {
    show().expect("Failed to start UI")
}
