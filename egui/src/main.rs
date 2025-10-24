//! Interactive Feedback MCP - egui GUI Application
//! 
//! A native Rust GUI application for collecting user feedback
//! in the Interactive Feedback MCP workflow.

use eframe::egui;
use serde::Deserialize;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::path::Path;
use include_dir::{include_dir, Dir};
use ifm_ruta_core::services::ConversationStorage;

// Include fonts directory
static FONTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/fonts");

#[derive(Deserialize, Clone)]
struct ConversationEntry {
    role: String,
    content: String,
    timestamp: String,
}

impl ConversationEntry {
    fn new(role: String, content: String) -> Self {
        Self {
            role,
            content,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Thread-safe conversation manager using Rust patterns
struct ConversationManager {
    conversations: Arc<Mutex<VecDeque<ConversationEntry>>>,
    max_size: usize,
    storage: Option<ConversationStorage>,
}

impl ConversationManager {
    // Removed unused new() method
    
    fn new_with_storage(max_size: usize, project_directory: &Path) -> Self {
        let storage = ConversationStorage::new(project_directory);
        let mut manager = Self {
            conversations: Arc::new(Mutex::new(VecDeque::new())),
            max_size,
            storage: Some(storage),
        };
        
        // Load real conversation history
        manager.load_conversation_history();
        manager
    }
    
    fn load_conversation_history(&mut self) {
        if let Some(ref storage) = self.storage {
            // Load all conversation sessions from storage
            match storage.get_project_sessions() {
                Ok(sessions) => {
                    println!("Loaded {} conversation sessions", sessions.len());
                    for session in sessions {
                        println!("Loading session: {} with {} messages", session.session_id, session.messages.len());
                        // Add all messages from this session
                        for message in session.messages {
                            self.add_conversation(message.role, message.content);
                        }
                    }
                }
                Err(e) => {
                    println!("Error loading conversation sessions: {}", e);
                }
            }
        } else {
            println!("No storage available");
        }
    }

    fn add_conversation(&self, role: String, content: String) {
        if let Ok(mut conversations) = self.conversations.lock() {
            conversations.push_back(ConversationEntry::new(role, content));
            
            // Auto-trim to max_size
            while conversations.len() > self.max_size {
                conversations.pop_front();
            }
        }
    }

    fn get_conversations(&self) -> Vec<ConversationEntry> {
        self.conversations.lock()
            .map(|conv| conv.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn clear(&self) {
        if let Ok(mut conversations) = self.conversations.lock() {
            conversations.clear();
        }
    }
}

#[derive(Deserialize)]
struct CursorContext {
    method: String,
    tool_name: String,
    arguments: serde_json::Value,
    timestamp: String,
    request_id: u64,
    // conversation_content: Option<ConversationContent>, // Removed unused field
}

/// Application state
struct App {
    project_directory: String,
    summary: String,
    feedback: String,
    conversation_manager: ConversationManager,
    cursor_context: Option<CursorContext>,
    error_message: Option<String>,
}

impl App {
    fn new(project_directory: String, summary: String, cursor_context: Option<CursorContext>) -> Self {
        // Use real conversation storage
        let conversation_manager = ConversationManager::new_with_storage(100, Path::new(&project_directory));
        
        Self {
            project_directory,
            summary,
            feedback: String::new(),
            conversation_manager,
            cursor_context,
            error_message: None,
        }
    }

    fn add_user_feedback(&mut self, feedback: String) {
        self.conversation_manager.add_conversation("user".to_string(), feedback);
    }

    fn submit_feedback(&mut self) {
        if self.feedback.trim().is_empty() {
            self.error_message = Some("Please enter your feedback".to_string());
            return;
        }

        // Add user feedback to conversation history
        self.add_user_feedback(self.feedback.clone());

        // Output feedback to stdout for MCP to capture
        println!("{}", self.feedback);
        
        // Close application
        std::process::exit(0);
    }

    fn cancel_feedback(&mut self) {
        // Output empty feedback
        println!("");
        
        // Close application
        std::process::exit(0);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Left panel - Conversation history
        egui::SidePanel::left("conversation_panel")
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Previous Conversation");
                    if ui.button("Clear").clicked() {
                        self.conversation_manager.clear();
                    }
                });
                ui.add_space(5.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for entry in self.conversation_manager.get_conversations() {
                        ui.group(|ui| {
                            ui.label(format!("{}: {}", entry.role, entry.timestamp));
                            ui.label(&entry.content);
                        });
                    }
                });
            });
        
        // Right panel - Feedback input with fixed layout
        egui::CentralPanel::default().show(ctx, |ui| {
            // Use vertical layout with fixed height for main content
            ui.vertical(|ui| {
                ui.heading("Interactive Feedback MCP");
                ui.add_space(10.0);

                // Scrollable content area with fixed height
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 120.0) // Reserve space for buttons
                    .show(ui, |ui| {
                        // Project information
                        ui.group(|ui| {
                            ui.heading("Project Information");
                            ui.label(format!("Project: {}", self.project_directory));
                            ui.label(format!("Summary: {}", self.summary));
                        });

                        ui.add_space(10.0);

                        // Cursor context information
                        if let Some(context) = &self.cursor_context {
                            ui.group(|ui| {
                                ui.heading("Cursor MCP Context");
                                ui.label(format!("Tool: {}", context.tool_name));
                                ui.label(format!("Method: {}", context.method));
                                ui.label(format!("Request ID: {}", context.request_id));
                                ui.label(format!("Timestamp: {}", context.timestamp));
                                ui.label("Arguments:");
                                ui.code(format!("{}", context.arguments));
                            });
                            ui.add_space(10.0);
                        }

                        // Feedback input section
                        ui.group(|ui| {
                            ui.heading("Your Feedback");
                            ui.label("Please provide your feedback:");
                            
                            // Fixed height text input
                            ui.add_sized(
                                [ui.available_width(), 200.0], // Fixed height
                                egui::TextEdit::multiline(&mut self.feedback)
                                    .hint_text("Enter your feedback here...\n\nSupports multiline text.\nUse Ctrl+Enter to submit.")
                                    .font(egui::TextStyle::Body)
                            );
                        });
                    });

                // Fixed bottom section with buttons
                ui.add_space(10.0);
                
                // Error message
                if let Some(error) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, error);
                }

                // Buttons - always visible at bottom
                ui.horizontal(|ui| {
                    if ui.button("Submit Feedback").clicked() {
                        self.submit_feedback();
                    }

                    if ui.button("Cancel").clicked() {
                        self.cancel_feedback();
                    }
                });
            });

            // Keyboard shortcuts
            if ctx.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.ctrl) {
                self.submit_feedback();
            }

            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.cancel_feedback();
            }
        });
    }
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut project_directory = "Unknown Project".to_string();
    let mut summary = "No summary provided".to_string();
    let cursor_context: Option<CursorContext> = None;

    // Simple argument parsing: first arg is project directory, second is summary
    if args.len() > 1 {
        project_directory = args[1].clone();
    }
    if args.len() > 2 {
        summary = args[2].clone();
    }

    println!("egui GUI started with project: {}", project_directory);
    println!("egui GUI started with summary: {}", summary);

    // Initialize logging
    env_logger::init();

    // Create app
    let app = App::new(project_directory, summary, cursor_context);

    // Run the GUI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 700.0])
            .with_min_inner_size([800.0, 500.0])
            .with_decorations(true)
            .with_transparent(false),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Interactive Feedback MCP",
        options,
        Box::new(|cc| {
            // Configure fonts for Vietnamese support
            let mut fonts = egui::FontDefinitions::default();
            
            // Load Noto Sans font for Vietnamese support
            if let Some(font_data) = FONTS_DIR.get_file("NotoSans-Regular.ttf") {
                fonts.font_data.insert(
                    "noto_sans".to_owned(),
                    egui::FontData::from_static(font_data.contents()),
                );
                
                // Use Noto Sans as primary font
                fonts.families.get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "noto_sans".to_owned());
            }
            
            cc.egui_ctx.set_fonts(fonts);
            
            // Set light theme
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            
            // Increase text and button sizes
            cc.egui_ctx.style_mut(|style| {
                // Increase font sizes
                style.text_styles.insert(egui::TextStyle::Heading, egui::FontId::new(24.0, egui::FontFamily::Proportional));
                style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional));
                style.text_styles.insert(egui::TextStyle::Button, egui::FontId::new(16.0, egui::FontFamily::Proportional));
                style.text_styles.insert(egui::TextStyle::Small, egui::FontId::new(14.0, egui::FontFamily::Proportional));
                
                // Increase button and spacing sizes
                style.spacing.button_padding = egui::vec2(12.0, 8.0);
                style.spacing.item_spacing = egui::vec2(8.0, 6.0);
                style.spacing.window_margin = egui::Margin::same(12.0);
            });
            
            Ok(Box::new(app))
        }),
    );
}
