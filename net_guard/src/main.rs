//! NetGuard - macOS Network Traffic Monitor
//! 
//! A native macOS desktop application for monitoring network traffic,
////! with process-level ranking and historical data visualization.

mod collector;
mod gui;
mod storage;
mod utils;

use std::sync::Arc;
use parking_lot::Mutex;
use collector::{TrafficCollector, ProcessTraffic};
use storage::Database;

/// Application state shared across GUI
pub struct AppState {
    pub traffic_collector: TrafficCollector,
    pub database: Database,
    pub current_traffic: Arc<Mutex<(u64, u64)>>, // (bytes_in, bytes_out)
    pub process_list: Arc<Mutex<Vec<ProcessTraffic>>>,
}

impl AppState {
    pub fn new() -> Result<Self, String> {
        let database = Database::new().map_err(|e| e.to_string())?;
        let traffic_collector = TrafficCollector::new();
        
        Ok(Self {
            traffic_collector,
            database,
            current_traffic: Arc::new(Mutex::new((0, 0))),
            process_list: Arc::new(Mutex::new(Vec::new())),
        })
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("NetGuard - Network Traffic Monitor"),
        ..Default::default()
    };

    eframe::run_native(
        "NetGuard",
        options,
        Box::new(|_cc| Ok(Box::new(gui::NetGuardApp::new()?))),
    )
}
