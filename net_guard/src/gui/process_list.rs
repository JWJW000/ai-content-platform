//! Process ranking list showing traffic by process

use crate::collector::ProcessTraffic;
use crate::utils::format_bytes;

pub struct ProcessList {
    processes: Vec<ProcessTraffic>,
}

impl ProcessList {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    pub fn update(&mut self, processes: &[ProcessTraffic]) {
        // Sort by total traffic descending
        let mut sorted = processes.to_vec();
        sorted.sort_by(|a, b| b.total().cmp(&a.total()));
        
        // Keep top 20 processes
        sorted.truncate(20);
        self.processes = sorted;
    }
}

impl egui::Widget for ProcessList {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            egui::Grid::new("process_grid")
                .num_columns(4)
                .spacing([10.0, 5.0])
                .show(ui, |ui| {
                    // Header
                    ui.label(egui::RichText::new("Process").strong());
                    ui.label(egui::RichText::new("↑ Up").strong());
                    ui.label(egui::RichText::new("↓ Down").strong());
                    ui.label(egui::RichText::new("Total").strong());
                    ui.end_row();
                    
                    // Process rows
                    for (idx, p) in self.processes.iter().enumerate() {
                        // Highlight top 3
                        let name_color = match idx {
                            0 => egui::Color32::GOLD,
                            1 => egui::Color32::SILVER,
                            2 => egui::Color32::from_rgb(205, 127, 50), // bronze
                            _ => egui::Color32::WHITE,
                        };
                        
                        ui.label(egui::RichText::new(&p.name).color(name_color));
                        ui.label(egui::RichText::new(format_bytes(p.bytes_out)).color(egui::Color32::GREEN));
                        ui.label(egui::RichText::new(format_bytes(p.bytes_in)).color(egui::Color32::BLUE));
                        ui.label(format_bytes(p.total()));
                        ui.end_row();
                    }
                });
        }).response
    }
}
