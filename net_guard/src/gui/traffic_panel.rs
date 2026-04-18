//! Traffic panel showing real-time upload/download speeds

use std::collections::VecDeque;
use crate::utils::{format_speed, format_bytes};

pub struct TrafficPanel {
    bytes_in: u64,
    bytes_out: u64,
    speed_in: u64,
    speed_out: u64,
    history: VecDeque<(f32, f32)>,
}

impl TrafficPanel {
    pub fn new() -> Self {
        Self {
            bytes_in: 0,
            bytes_out: 0,
            speed_in: 0,
            speed_out: 0,
            history: VecDeque::with_capacity(60),
        }
    }

    pub fn update(&mut self, speed_in: u64, speed_out: u64, history: &VecDeque<(f32, f32)>) {
        self.speed_in = speed_in;
        self.speed_out = speed_out;
        self.history = history.clone();
    }
}

impl super::GuiComponent for TrafficPanel {
    fn as_mut(&mut self) -> &mut dyn eframe::egui::Widget {
        self
    }
}

/// Trait for GUI components
pub trait GuiComponent {
    fn as_mut(&mut self) -> &mut dyn egui::Widget;
}

impl egui::Widget for TrafficPanel {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            // Speed display
            ui.horizontal(|ui| {
                ui.label("↑ Upload:");
                ui.label(egui::RichText::new(format_speed(self.speed_in)).color(egui::Color32::GREEN));
                
                ui.add_space(30.0);
                
                ui.label("↓ Download:");
                ui.label(egui::RichText::new(format_speed(self.speed_out)).color(egui::Color32::BLUE));
            });
            
            // Simple speed visualization
            ui.add_space(10.0);
            
            let max_speed = self.speed_in.max(self.speed_out).max(1) as f32;
            
            // Upload bar
            ui.horizontal(|ui| {
                ui.label("↑");
                let upload_ratio = self.speed_in as f32 / max_speed;
                egui::Widget::ui(&mut egui::ProgressBar::new(upload_ratio as f64).fill(egui::Color32::GREEN), ui);
                ui.label(format_speed(self.speed_in));
            });
            
            // Download bar
            ui.horizontal(|ui| {
                ui.label("↓");
                let download_ratio = self.speed_out as f32 / max_speed;
                egui::Widget::ui(&mut egui::ProgressBar::new(download_ratio as f64).fill(egui::Color32::BLUE), ui);
                ui.label(format_speed(self.speed_out));
            });
        }).response
    }
}
