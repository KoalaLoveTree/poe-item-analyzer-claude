//! Main application state

use egui::Context;
use poe_item_analyzer_api::parser::{PobDataParser, LutData};
use poe_item_analyzer_api::DataDownloader;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

/// Download files with progress reporting
async fn download_with_progress(
    temp_dir: PathBuf,
    tx: Sender<AsyncMessage>,
) -> Result<PathBuf, String> {
    eprintln!("DEBUG: Starting download_with_progress");

    // Create target directory
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| {
            eprintln!("DEBUG: Failed to create directory: {}", e);
            format!("Failed to create directory: {}", e)
        })?;

    eprintln!("DEBUG: Directory created: {}", temp_dir.display());

    // List of files to download
    let files = vec![
        "NodeIndexMapping.lua",
        "LegionPassives.lua",
        "LethalPride.zip",
        "BrutalRestraint.zip",
        "ElegantHubris.zip",
        "MilitantFaith.zip",
        "GloriousVanity.zip.part0",
        "GloriousVanity.zip.part1",
        "GloriousVanity.zip.part2",
        "GloriousVanity.zip.part3",
        "GloriousVanity.zip.part4",
    ];

    let total = files.len();
    let base_url = "https://raw.githubusercontent.com/PathOfBuildingCommunity/PathOfBuilding/master/src/Data/TimelessJewelData";

    eprintln!("DEBUG: Creating reqwest client");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| {
            eprintln!("DEBUG: Failed to create client: {}", e);
            format!("Failed to create HTTP client: {}", e)
        })?;

    eprintln!("DEBUG: Starting download loop for {} files", total);

    for (index, file_name) in files.iter().enumerate() {
        let current = index + 1;

        eprintln!("DEBUG: Downloading file {}/{}: {}", current, total, file_name);

        // Send progress update
        if let Err(e) = tx.send(AsyncMessage::DownloadProgress {
            current,
            total,
            file_name: file_name.to_string(),
        }) {
            eprintln!("DEBUG: Failed to send progress: {}", e);
        }

        let url = format!("{}/{}", base_url, file_name);
        eprintln!("DEBUG: URL: {}", url);

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| {
                eprintln!("DEBUG: Request failed: {}", e);
                format!("Failed to download {}: {}", file_name, e)
            })?;

        eprintln!("DEBUG: Response status: {}", response.status());

        if !response.status().is_success() {
            return Err(format!(
                "Failed to download {}: HTTP {}",
                file_name,
                response.status()
            ));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| {
                eprintln!("DEBUG: Failed to read bytes: {}", e);
                format!("Failed to read {}: {}", file_name, e)
            })?;

        eprintln!("DEBUG: Downloaded {} bytes", bytes.len());

        let file_path = temp_dir.join(file_name);
        std::fs::write(&file_path, &bytes)
            .map_err(|e| {
                eprintln!("DEBUG: Failed to write file: {}", e);
                format!("Failed to save {}: {}", file_name, e)
            })?;

        eprintln!("DEBUG: Saved to: {}", file_path.display());
    }

    eprintln!("DEBUG: All downloads complete!");

    // Concatenate GloriousVanity parts into single file
    eprintln!("DEBUG: Concatenating GloriousVanity parts...");
    let part_files = vec![
        "GloriousVanity.zip.part0",
        "GloriousVanity.zip.part1",
        "GloriousVanity.zip.part2",
        "GloriousVanity.zip.part3",
        "GloriousVanity.zip.part4",
    ];

    let mut glorious_vanity_data = Vec::new();
    for part in &part_files {
        let part_path = temp_dir.join(part);
        let part_data = std::fs::read(&part_path)
            .map_err(|e| format!("Failed to read {}: {}", part, e))?;
        glorious_vanity_data.extend_from_slice(&part_data);
        eprintln!("DEBUG: Added {} bytes from {}", part_data.len(), part);
    }

    let glorious_vanity_path = temp_dir.join("GloriousVanity.zip");
    std::fs::write(&glorious_vanity_path, &glorious_vanity_data)
        .map_err(|e| format!("Failed to write GloriousVanity.zip: {}", e))?;

    eprintln!("DEBUG: Created GloriousVanity.zip ({} bytes)", glorious_vanity_data.len());

    Ok(temp_dir)
}

/// Messages from async tasks
enum AsyncMessage {
    DownloadProgress { current: usize, total: usize, file_name: String },
    DownloadComplete(Result<PathBuf, String>),
    ParseComplete(Result<LutData, String>),
}

/// Main application state
pub struct AnalyzerApp {
    /// Parser test tab state
    parser_test: ParserTestState,
    /// Channel receiver for async messages
    rx: Receiver<AsyncMessage>,
    /// Channel sender for async messages
    tx: Sender<AsyncMessage>,
}

/// State for parser testing UI
struct ParserTestState {
    /// Selected data directory path
    data_dir: String,
    /// Parsed LUT data (if successful)
    parsed_data: Option<LutData>,
    /// Error message (if parsing failed)
    error_message: Option<String>,
    /// Whether parsing is in progress
    parsing: bool,
    /// Whether downloading is in progress
    downloading: bool,
    /// Download progress
    download_progress: Option<(usize, usize, String)>, // (current, total, current_file)
    /// Parsing log messages
    log_messages: Vec<String>,
}

impl Default for ParserTestState {
    fn default() -> Self {
        // Default to temp directory for downloads
        let temp_dir = std::env::temp_dir().join("poe-item-analyzer-test");

        Self {
            data_dir: temp_dir.display().to_string(),
            parsed_data: None,
            error_message: None,
            parsing: false,
            downloading: false,
            download_progress: None,
            log_messages: Vec::new(),
        }
    }
}

impl AnalyzerApp {
    /// Create a new application
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = channel();

        let mut app = Self {
            parser_test: ParserTestState::default(),
            rx,
            tx,
        };

        // Check if data already exists
        app.check_existing_data();

        app
    }

    /// Check if data already exists and auto-parse if it does
    fn check_existing_data(&mut self) {
        let temp_dir = std::env::temp_dir().join("poe-item-analyzer-test");

        if !temp_dir.exists() {
            return;
        }

        // Check if required files exist
        let required_files = vec![
            "NodeIndexMapping.lua",
            "LegionPassives.lua",
        ];

        let all_exist = required_files.iter().all(|f| temp_dir.join(f).exists());

        if all_exist {
            self.parser_test.log_messages.push("✓ Found existing data files".to_string());
            self.parser_test.data_dir = temp_dir.display().to_string();
            // Auto-parse existing data
            self.parse_directory();
        }
    }

    /// Process async messages
    fn process_messages(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AsyncMessage::DownloadProgress { current, total, file_name } => {
                    self.parser_test.download_progress = Some((current, total, file_name.clone()));

                    // Only log when starting a new file
                    if current > 0 && current <= total {
                        let last_log = self.parser_test.log_messages.last();
                        let new_log = format!("  [{}/{}] Downloading: {}", current, total, file_name);

                        // Update or add log message
                        if let Some(last) = last_log {
                            if last.contains(&format!("[{}/{}]", current, total)) {
                                // Replace the last message
                                if let Some(last_msg) = self.parser_test.log_messages.last_mut() {
                                    *last_msg = new_log;
                                }
                            } else {
                                self.parser_test.log_messages.push(new_log);
                            }
                        } else {
                            self.parser_test.log_messages.push(new_log);
                        }
                    }
                }
                AsyncMessage::DownloadComplete(result) => {
                    self.parser_test.downloading = false;
                    self.parser_test.download_progress = None;

                    match result {
                        Ok(path) => {
                            self.parser_test.log_messages.push("✓ Download complete!".to_string());
                            self.parser_test.data_dir = path.display().to_string();

                            // Automatically parse after download
                            self.parser_test.log_messages.push("Starting parse...".to_string());
                            self.parse_directory();
                        }
                        Err(e) => {
                            self.parser_test.log_messages.push(format!("✗ Download failed: {}", e));
                            self.parser_test.error_message = Some(e);
                        }
                    }
                }
                AsyncMessage::ParseComplete(result) => {
                    self.parser_test.parsing = false;

                    match result {
                        Ok(data) => {
                            self.parser_test.log_messages.push("✓ Parsing successful!".to_string());
                            self.parser_test.log_messages.push(format!("  - {} node indices", data.node_indices.len()));
                            self.parser_test.log_messages.push(format!("  - {} modifiers", data.modifiers.len()));
                            self.parser_test.log_messages.push(format!("  - {} jewel types", data.jewels.len()));

                            for (jewel_type, jewel_data) in &data.jewels {
                                self.parser_test.log_messages.push(format!(
                                    "  - {}: {} seeds parsed",
                                    jewel_type,
                                    jewel_data.lookup_table.len()
                                ));
                            }

                            self.parser_test.parsed_data = Some(data);
                        }
                        Err(e) => {
                            self.parser_test.log_messages.push(format!("✗ {}", e));
                            self.parser_test.error_message = Some(e);
                        }
                    }
                }
            }
        }
    }

    /// Render the parser test tab
    fn render_parser_test(&mut self, ui: &mut egui::Ui) {
        ui.heading("Parser Test - PoB Data");
        ui.add_space(10.0);

        let has_data = self.parser_test.parsed_data.is_some();
        let is_busy = self.parser_test.downloading || self.parser_test.parsing;

        // Show status
        if has_data {
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::GREEN, "✓ Data loaded");

                if ui.add_enabled(!is_busy, egui::Button::new("🔄 Re-download")).clicked() {
                    self.download_and_parse();
                }
            });
        } else if !is_busy {
            ui.label("No data loaded. Click below to download:");
            ui.add_space(5.0);

            if ui.button("🚀 Download & Parse Data").clicked() {
                self.download_and_parse();
            }
        }

        ui.add_space(5.0);

        // Progress bars
        if self.parser_test.downloading {
            if let Some((current, total, file_name)) = &self.parser_test.download_progress {
                ui.label(format!("Downloading: {} ({}/{})", file_name, current, total));
                let progress = *current as f32 / *total as f32;
                ui.add(egui::ProgressBar::new(progress).show_percentage());
            } else {
                ui.label("Initializing download...");
                ui.add(egui::ProgressBar::new(0.0));
            }
        } else if self.parser_test.parsing {
            ui.label("Parsing data files...");
            ui.add(egui::ProgressBar::new(0.5).show_percentage());
        }

        ui.add_space(10.0);
        ui.separator();

        // Display results
        if let Some(error) = &self.parser_test.error_message {
            ui.colored_label(egui::Color32::RED, "❌ Error:");
            ui.label(error);
            ui.add_space(10.0);
        }

        if let Some(data) = &self.parser_test.parsed_data {
            ui.heading("📊 Parsed Data Summary");
            ui.add_space(5.0);

            egui::Grid::new("parser_stats_grid")
                .num_columns(2)
                .spacing([20.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Version:");
                    ui.label(&data.version);
                    ui.end_row();

                    ui.label("Node Indices:");
                    ui.label(format!("{}", data.node_indices.len()));
                    ui.end_row();

                    ui.label("Modifiers:");
                    ui.label(format!("{}", data.modifiers.len()));
                    ui.end_row();

                    ui.label("Jewel Types:");
                    ui.label(format!("{}", data.jewels.len()));
                    ui.end_row();
                });

            ui.add_space(10.0);

            // Display jewel details
            if !data.jewels.is_empty() {
                ui.heading("💎 Jewel Data");
                ui.add_space(5.0);

                egui::ScrollArea::vertical()
                    .id_source("jewel_data_scroll")
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for (idx, (jewel_type, jewel_data)) in data.jewels.iter().enumerate() {
                            ui.group(|ui| {
                                ui.strong(jewel_type);

                                ui.horizontal(|ui| {
                                    ui.label("Seed Range:");
                                    ui.monospace(format!("{} - {}",
                                        jewel_data.seed_range.0,
                                        jewel_data.seed_range.1
                                    ));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Seeds with data:");
                                    ui.monospace(format!("{}", jewel_data.lookup_table.len()));
                                });

                                // Show sample seed data
                                if let Some((seed, node_mods)) = jewel_data.lookup_table.iter().next() {
                                    ui.horizontal(|ui| {
                                        ui.label("Sample seed:");
                                        ui.monospace(format!("{} ({} nodes)", seed, node_mods.len()));
                                    });
                                }
                            });

                            if idx < data.jewels.len() - 1 {
                                ui.add_space(5.0);
                            }
                        }
                    });
            }
        }

        // Log messages
        if !self.parser_test.log_messages.is_empty() {
            ui.add_space(10.0);
            ui.separator();

            ui.collapsing("📋 Parse Log", |ui| {
                egui::ScrollArea::vertical()
                    .id_source("parse_log_scroll")
                    .max_height(200.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for (idx, msg) in self.parser_test.log_messages.iter().enumerate() {
                            ui.label(msg);
                            if idx < self.parser_test.log_messages.len() - 1 {
                                ui.add_space(2.0);
                            }
                        }
                    });
            });
        }
    }

    /// Download data from GitHub and parse it
    fn download_and_parse(&mut self) {
        eprintln!("DEBUG: download_and_parse called");

        self.parser_test.downloading = true;
        self.parser_test.error_message = None;
        // Don't clear parsed_data here - keep it until new data is ready
        self.parser_test.log_messages.clear();
        self.parser_test.download_progress = None;

        let temp_dir = std::env::temp_dir().join("poe-item-analyzer-test");
        self.parser_test.log_messages.push(format!("Download directory: {}", temp_dir.display()));
        self.parser_test.log_messages.push("Starting download...".to_string());

        let tx = self.tx.clone();

        eprintln!("DEBUG: Spawning thread with tokio runtime");

        // Spawn a thread with its own tokio runtime
        std::thread::spawn(move || {
            eprintln!("DEBUG: Thread started, creating tokio runtime");

            let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

            eprintln!("DEBUG: Running async task on runtime");
            rt.block_on(async move {
                eprintln!("DEBUG: Async task started");
                let result = download_with_progress(temp_dir.clone(), tx.clone()).await;
                eprintln!("DEBUG: Download result: {:?}", result.is_ok());
                if let Err(e) = tx.send(AsyncMessage::DownloadComplete(result)) {
                    eprintln!("DEBUG: Failed to send complete message: {}", e);
                }
            });

            eprintln!("DEBUG: Thread finishing");
        });

        eprintln!("DEBUG: Thread spawned");
    }

    /// Parse the selected directory
    fn parse_directory(&mut self) {
        self.parser_test.parsing = true;
        self.parser_test.error_message = None;
        self.parser_test.parsed_data = None;

        let path = PathBuf::from(&self.parser_test.data_dir);

        self.parser_test.log_messages.push(format!("Parsing directory: {}", path.display()));

        // First, list what files we have
        match std::fs::read_dir(&path) {
            Ok(entries) => {
                self.parser_test.log_messages.push("Files found:".to_string());
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        self.parser_test.log_messages.push(format!(
                            "  - {} ({} bytes)",
                            entry.file_name().to_string_lossy(),
                            metadata.len()
                        ));
                    }
                }
            }
            Err(e) => {
                self.parser_test.log_messages.push(format!("Cannot read directory: {}", e));
            }
        }

        match PobDataParser::parse_directory(&path) {
            Ok(data) => {
                self.parser_test.log_messages.push("✓ Parsing successful!".to_string());
                self.parser_test.log_messages.push(format!("  - {} node indices", data.node_indices.len()));
                self.parser_test.log_messages.push(format!("  - {} modifiers", data.modifiers.len()));
                self.parser_test.log_messages.push(format!("  - {} jewel types", data.jewels.len()));

                for (jewel_type, jewel_data) in &data.jewels {
                    self.parser_test.log_messages.push(format!(
                        "  - {}: {} seeds parsed",
                        jewel_type,
                        jewel_data.lookup_table.len()
                    ));
                }

                self.parser_test.parsed_data = Some(data);
            }
            Err(e) => {
                let error_msg = format!("Failed to parse: {}", e);
                self.parser_test.log_messages.push(format!("✗ {}", error_msg));
                self.parser_test.error_message = Some(e.to_string());
            }
        }

        self.parser_test.parsing = false;
    }
}

impl eframe::App for AnalyzerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Process async messages
        self.process_messages();

        // Request repaint if operations are in progress
        if self.parser_test.downloading || self.parser_test.parsing {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PoE Item Analyzer - Parser Testing");
            ui.separator();

            self.render_parser_test(ui);
        });
    }
}
