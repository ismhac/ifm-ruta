//! Interactive Feedback MCP - egui GUI Application
//! 
//! A native Rust GUI application for collecting user feedback
//! in the Interactive Feedback MCP workflow.

use eframe::egui;
use serde::Deserialize;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use include_dir::{include_dir, Dir};

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
}

impl ConversationManager {
    fn new(max_size: usize) -> Self {
        Self {
            conversations: Arc::new(Mutex::new(VecDeque::new())),
            max_size,
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
struct ConversationContent {
    user_message: String,
    assistant_response: String,
}

#[derive(Deserialize)]
struct CursorContext {
    method: String,
    tool_name: String,
    arguments: serde_json::Value,
    timestamp: String,
    request_id: u64,
    conversation_content: Option<ConversationContent>,
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
        let conversation_manager = ConversationManager::new(5);
        
        // Add current conversation from cursor context
        if let Some(context) = &cursor_context {
            // Add real conversation content if available
            if let Some(conv_content) = &context.conversation_content {
                conversation_manager.add_conversation(
                    "user".to_string(),
                    conv_content.user_message.clone()
                );
                conversation_manager.add_conversation(
                    "assistant".to_string(),
                    conv_content.assistant_response.clone()
                );
            } else {
                // Fallback to tool call info
                conversation_manager.add_conversation(
                    "user".to_string(),
                    format!("MCP Tool Call: {} - {}", context.tool_name, summary)
                );
            }
        } else {
            // Add fallback conversation for testing
            conversation_manager.add_conversation(
                "user".to_string(),
                format!("User request: {}", summary)
            );
            conversation_manager.add_conversation(
                "assistant".to_string(),
                "MCP tool called successfully".to_string()
            );
        }
        
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
        
        // Right panel - Feedback input
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Interactive Feedback MCP");
            ui.add_space(10.0);

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
            }

            ui.add_space(10.0);

            // Feedback input
            ui.group(|ui| {
                ui.heading("Your Feedback");
                ui.label("Please provide your feedback:");
                
                // Improved multiline text input
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        [ui.available_width(), 300.0],
                        egui::TextEdit::multiline(&mut self.feedback)
                            .hint_text("Enter your feedback here...\n\nSupports multiline text.\nUse Ctrl+Enter to submit.")
                            .font(egui::TextStyle::Body)
                    );
                });

                // Error message
                if let Some(error) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, error);
                }

                ui.add_space(10.0);

                // Buttons
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
    let mut cursor_context: Option<CursorContext> = None;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--project-directory" {
            if let Some(dir) = args.get(i + 1) {
                project_directory = dir.clone();
                i += 1;
            }
        } else if args[i] == "--summary" {
            if let Some(s) = args.get(i + 1) {
                summary = s.clone();
                i += 1;
            }
        } else if args[i] == "--cursor-context" {
            if let Some(context_str) = args.get(i + 1) {
                if let Ok(context) = serde_json::from_str::<CursorContext>(context_str) {
                    cursor_context = Some(context);
                }
                i += 1;
            }
        }
        i += 1;
    }

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

    eframe::run_native(
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
