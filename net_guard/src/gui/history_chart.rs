//! History chart showing historical traffic data

use crate::storage::database::{HourlyData, DailyData};
use crate::utils::format_bytes;

pub struct HistoryChart {
    view_mode: ViewMode,
    hourly_data: Vec<HourlyData>,
    daily_data: Vec<DailyData>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Hourly,
    Daily,
}

impl HistoryChart {
    pub fn new() -> Self {
        Self {
            view_mode: ViewMode::Hourly,
            hourly_data: Vec::new(),
            daily_data: Vec::new(),
        }
    }

    pub fn update_hourly(&mut self, data: &[HourlyData]) {
        self.hourly_data = data.to_vec();
    }

    pub fn update_daily(&mut self, data: &[DailyData]) {
        self.daily_data = data.to_vec();
    }

    pub fn toggle_view(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Hourly => ViewMode::Daily,
            ViewMode::Daily => ViewMode::Hourly,
        };
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.selectable_label(self.view_mode == ViewMode::Hourly, "Hourly").clicked() {
                self.view_mode = ViewMode::Hourly;
            }
            if ui.selectable_label(self.view_mode == ViewMode::Daily, "Daily").clicked() {
                self.view_mode = ViewMode::Daily;
            }
        });
        
        match self.view_mode {
            ViewMode::Hourly => {
                if self.hourly_data.is_empty() {
                    ui.label("No hourly data available");
                } else {
                    let total_in: u64 = self.hourly_data.iter().map(|h| h.bytes_in).sum();
                    let total_out: u64 = self.hourly_data.iter().map(|h| h.bytes_out).sum();
                    ui.label(format!("Last {} hours: ↑ {} ↓ {}", 
                        self.hourly_data.len(),
                        format_bytes(total_out),
                        format_bytes(total_in)
                    ));
                    
                    egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                        let max_val = self.hourly_data.iter()
                            .map(|h| h.bytes_in.max(h.bytes_out))
                            .max()
                            .unwrap_or(1);
                        
                        for h in self.hourly_data.iter().rev().take(12) {
                            let in_ratio = h.bytes_in as f32 / max_val as f32;
                            let out_ratio = h.bytes_out as f32 / max_val as f32;
                            
                            ui.horizontal(|ui| {
                                ui.label(format_bytes(h.bytes_in));
                                let mut pb_in = egui::ProgressBar::new(in_ratio as f64);
                                pb_in = pb_in.fill(egui::Color32::BLUE);
                                ui.add(pb_in);
                                ui.add_space(10.0);
                                let mut pb_out = egui::ProgressBar::new(out_ratio as f64);
                                pb_out = pb_out.fill(egui::Color32::GREEN);
                                ui.add(pb_out);
                                ui.label(format_bytes(h.bytes_out));
                            });
                        }
                    });
                }
            }
            ViewMode::Daily => {
                if self.daily_data.is_empty() {
                    ui.label("No daily data available");
                } else {
                    let total_in: u64 = self.daily_data.iter().map(|d| d.bytes_in).sum();
                    let total_out: u64 = self.daily_data.iter().map(|d| d.bytes_out).sum();
                    ui.label(format!("Last {} days: ↑ {} ↓ {}", 
                        self.daily_data.len(),
                        format_bytes(total_out),
                        format_bytes(total_in)
                    ));
                    
                    egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                        let max_val = self.daily_data.iter()
                            .map(|d| d.bytes_in.max(d.bytes_out))
                            .max()
                            .unwrap_or(1);
                        
                        for d in self.daily_data.iter().rev() {
                            let in_ratio = d.bytes_in as f32 / max_val as f32;
                            let out_ratio = d.bytes_out as f32 / max_val as f32;
                            
                            ui.horizontal(|ui| {
                                ui.label(&d.day);
                                let mut pb_in = egui::ProgressBar::new(in_ratio as f64);
                                pb_in = pb_in.fill(egui::Color32::BLUE);
                                ui.add(pb_in);
                                ui.add_space(10.0);
                                let mut pb_out = egui::ProgressBar::new(out_ratio as f64);
                                pb_out = pb_out.fill(egui::Color32::GREEN);
                                ui.add(pb_out);
                            });
                        }
                    });
                }
            }
        }
    }
}
