//! Traffic panel showing real-time upload/download speeds

use std::collections::VecDeque;
use crate::utils::format_speed;

pub struct TrafficPanel {
    speed_in: u64,
    speed_out: u64,
    history: VecDeque<(f32, f32)>,
}

impl TrafficPanel {
    pub fn new() -> Self {
        Self {
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
    
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("↑ Upload:");
            ui.label(egui::RichText::new(format_speed(self.speed_in)).color(egui::Color32::GREEN));
            
            ui.add_space(30.0);
            
            ui.label("↓ Download:");
            ui.label(egui::RichText::new(format_speed(self.speed_out)).color(egui::Color32::BLUE));
        });
        
        ui.add_space(10.0);
        
        let max_speed = (self.speed_in.max(self.speed_out) as f32).max(1.0);
        
        ui.horizontal(|ui| {
            ui.label("↑");
            let ratio = self.speed_in as f32 / max_speed;
            ui.add(egui::ProgressBar::new(ratio as f64).fill(egui::Color32::GREEN));
            ui.label(format_speed(self.speed_in));
        });
        
        ui.horizontal(|ui| {
            ui.label("↓");
            let ratio = self.speed_out as f32 / max_speed;
            ui.add(egui::ProgressBar::new(ratio as f64).fill(egui::Color32::BLUE));
            ui.label(format_speed(self.speed_out));
        });
    }
}
