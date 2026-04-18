//! GUI module using egui
//! 
//! Main window, panels, and UI components.

mod traffic_panel;
mod process_list;
mod history_chart;

pub use traffic_panel::TrafficPanel;
pub use process_list::ProcessList;
pub use history_chart::HistoryChart;

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::collector::{ProcessTraffic, TrafficCollector};
use crate::storage::Database;
use crate::utils::{format_speed, format_bytes};

/// Main application state
pub struct NetGuardApp {
    traffic_panel: TrafficPanel,
    process_list: ProcessList,
    history_chart: HistoryChart,
    collector: TrafficCollector,
    database: Database,
    // Current traffic data
    current_bytes_in: u64,
    current_bytes_out: u64,
    // Speed calculation
    last_bytes_in: u64,
    last_bytes_out: u64,
    speed_in: u64,
    speed_out: u64,
    // Traffic history for chart (last 60 samples)
    traffic_history: VecDeque<(f32, f32)>,
}

impl NetGuardApp {
    pub fn new() -> Result<Self, String> {
        let database = Database::new().map_err(|e| e.to_string())?;
        let collector = TrafficCollector::new();
        
        Ok(Self {
            traffic_panel: TrafficPanel::new(),
            process_list: ProcessList::new(),
            history_chart: HistoryChart::new(),
            collector,
            database,
            current_bytes_in: 0,
            current_bytes_out: 0,
            last_bytes_in: 0,
            last_bytes_out: 0,
            speed_in: 0,
            speed_out: 0,
            traffic_history: VecDeque::with_capacity(60),
        })
    }
}

impl eframe::App for NetGuardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Collect traffic data
        if let Ok(processes) = self.collector.collect() {
            // Calculate speed
            let total_in: u64 = processes.iter().map(|p| p.bytes_in).sum();
            let total_out: u64 = processes.iter().map(|p| p.bytes_out).sum();
            
            if self.last_bytes_in > 0 {
                self.speed_in = total_in.saturating_sub(self.last_bytes_in);
                self.speed_out = total_out.saturating_sub(self.last_bytes_out);
            }
            
            self.last_bytes_in = total_in;
            self.last_bytes_out = total_out;
            self.current_bytes_in = total_in;
            self.current_bytes_out = total_out;
            
            // Add to history
            if self.traffic_history.len() >= 60 {
                self.traffic_history.pop_front();
            }
            self.traffic_history.push_back((self.speed_in as f32, self.speed_out as f32));
            
            // Update UI
            self.traffic_panel.update(self.speed_in, self.speed_out, &self.traffic_history);
            self.process_list.update(&processes);
            
            // Record to database periodically (every 60 seconds would be ideal)
            // For now, just record on each update
            self.database.record_traffic(self.current_bytes_in, self.current_bytes_out).ok();
            
            // Record process snapshots
            for p in &processes {
                self.database.record_process_snapshot(p.pid, &p.name, p.bytes_in, p.bytes_out).ok();
            }
        }
        
        // Load history for chart
        if let Ok(hourly) = self.database.get_hourly_history(24) {
            self.history_chart.update_hourly(&hourly);
        }
        
        // Set dark mode based on system preference
        ctx.set_visuals(egui::Visuals::dark());
        
        // Main layout
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌐 NetGuard - Network Traffic Monitor");
            ui.separator();
            
            // Traffic panel
            ui.add(self.traffic_panel.as_mut());
            
            ui.separator();
            
            // Two column layout: process list and history chart
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("📊 Process Ranking");
                    ui.add(self.process_list.as_mut());
                });
                
                ui.separator();
                
                ui.vertical(|ui| {
                    ui.heading("📈 History");
                    if let Ok(daily) = self.database.get_daily_history(7) {
                        self.history_chart.update_daily(&daily);
                    }
                    ui.add(self.history_chart.as_mut());
                });
            });
        });
        
        // Request repaint at ~10 FPS
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}
