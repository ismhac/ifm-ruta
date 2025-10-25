//! Unified IFM-Ruta Application
//!
//! A single executable that can run as both:
//! 1. MCP Server (for Cursor integration)
//! 2. GUI Application (for interactive feedback)

use include_dir::{include_dir, Dir};
use serde::Deserialize;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};

use ifm_ruta_core::{
    models::AppError,
    services::{ConversationStorage, EventBusImpl, ProcessManagerImpl, SettingsManagerImpl},
    utils::init_logging,
};

// Include fonts directory
static FONTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/fonts");

// Re-export MCP modules from the mcp package
mod mcp;
mod tools;

use mcp::server::MCPRequest;
use mcp::MCPServer;
use tools::InteractiveFeedbackTool;

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
                        println!(
                            "Loading session: {} with {} messages",
                            session.session_id,
                            session.messages.len()
                        );
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
        self.conversations
            .lock()
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
}

/// Application state for GUI mode
struct App {
    project_directory: String,
    summary: String,
    feedback: String,
    conversation_manager: ConversationManager,
    cursor_context: Option<CursorContext>,
    error_message: Option<String>,
}

impl App {
    fn new(
        project_directory: String,
        summary: String,
        cursor_context: Option<CursorContext>,
    ) -> Self {
        // Use real conversation storage
        let conversation_manager =
            ConversationManager::new_with_storage(100, Path::new(&project_directory));

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
        self.conversation_manager
            .add_conversation("user".to_string(), feedback);
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
        println!();

        // Close application
        std::process::exit(0);
    }

    fn render_richtext_content(&self, ui: &mut eframe::egui::Ui, content: &str) {
        let text = content.to_string();

        // Handle code blocks
        if text.contains("```") {
            let parts: Vec<&str> = text.split("```").collect();
            for (i, part) in parts.iter().enumerate() {
                if i % 2 == 0 {
                    // Regular text
                    if !part.is_empty() {
                        self.render_markdown_text(ui, part);
                    }
                } else {
                    // Code block
                    ui.add_space(4.0);
                    ui.group(|ui| {
                        ui.code(*part);
                    });
                    ui.add_space(4.0);
                }
            }
        } else {
            // No code blocks, just render markdown text
            self.render_markdown_text(ui, &text);
        }
    }

    fn render_markdown_text(&self, ui: &mut eframe::egui::Ui, text: &str) {
        let mut current_text = text.to_string();

        // Handle bold text (**text**)
        while let Some(start) = current_text.find("**") {
            if let Some(end) = current_text[start + 2..].find("**") {
                let end_pos = start + 2 + end;
                let before = &current_text[..start];
                let bold_text = &current_text[start + 2..end_pos];
                let after = &current_text[end_pos + 2..];

                if !before.is_empty() {
                    ui.label(
                        eframe::egui::RichText::new(before)
                            .size(13.0)
                            .color(eframe::egui::Color32::from_gray(220)),
                    );
                }

                ui.label(
                    eframe::egui::RichText::new(bold_text)
                        .size(13.0)
                        .color(eframe::egui::Color32::from_gray(220))
                        .strong(),
                );

                current_text = after.to_string();
            } else {
                break;
            }
        }

        // Handle italic text (*text*)
        while let Some(start) = current_text.find("*") {
            if let Some(end) = current_text[start + 1..].find("*") {
                let end_pos = start + 1 + end;
                let before = &current_text[..start];
                let italic_text = &current_text[start + 1..end_pos];
                let after = &current_text[end_pos + 1..];

                if !before.is_empty() {
                    ui.label(
                        eframe::egui::RichText::new(before)
                            .size(13.0)
                            .color(eframe::egui::Color32::from_gray(220)),
                    );
                }

                ui.label(
                    eframe::egui::RichText::new(italic_text)
                        .size(13.0)
                        .color(eframe::egui::Color32::from_gray(220))
                        .italics(),
                );

                current_text = after.to_string();
            } else {
                break;
            }
        }

        // Render remaining text
        if !current_text.is_empty() {
            ui.label(
                eframe::egui::RichText::new(current_text)
                    .size(13.0)
                    .color(eframe::egui::Color32::from_gray(220)),
            );
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Left panel - Conversation history
        eframe::egui::SidePanel::left("conversation_panel")
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
                eframe::egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 50.0)
                    .show(ui, |ui| {
                        for entry in self.conversation_manager.get_conversations() {
                            // Create a frame with better styling
                            let frame = eframe::egui::Frame::group(ui.style())
                                .fill(match entry.role.as_str() {
                                    "user" => eframe::egui::Color32::from_rgba_premultiplied(
                                        30, 30, 30, 200,
                                    ),
                                    "assistant" => eframe::egui::Color32::from_rgba_premultiplied(
                                        20, 20, 40, 200,
                                    ),
                                    _ => eframe::egui::Color32::from_rgba_premultiplied(
                                        25, 25, 25, 200,
                                    ),
                                })
                                .stroke(eframe::egui::Stroke::new(
                                    1.0,
                                    eframe::egui::Color32::from_gray(60),
                                ))
                                .rounding(eframe::egui::Rounding::same(8.0))
                                .inner_margin(eframe::egui::Margin::same(12.0));

                            frame.show(ui, |ui| {
                                ui.vertical(|ui| {
                                    // Header with role and timestamp
                                    ui.horizontal(|ui| {
                                        // Role badge
                                        let role_color = match entry.role.as_str() {
                                            "user" => {
                                                eframe::egui::Color32::from_rgb(100, 150, 255)
                                            }
                                            "assistant" => {
                                                eframe::egui::Color32::from_rgb(100, 255, 150)
                                            }
                                            _ => eframe::egui::Color32::from_rgb(200, 200, 200),
                                        };

                                        ui.colored_label(role_color, entry.role.to_uppercase());
                                        ui.with_layout(
                                            eframe::egui::Layout::right_to_left(
                                                eframe::egui::Align::Center,
                                            ),
                                            |ui| {
                                                ui.label(
                                                    eframe::egui::RichText::new(&entry.timestamp)
                                                        .size(10.0)
                                                        .color(eframe::egui::Color32::from_gray(
                                                            150,
                                                        )),
                                                );
                                            },
                                        );
                                    });

                                    ui.add_space(8.0);

                                    // Content with richtext support
                                    let content = &entry.content;

                                    // Check if content contains markdown-like formatting
                                    if content.contains("```")
                                        || content.contains("**")
                                        || content.contains("*")
                                    {
                                        // Parse and display with richtext formatting
                                        self.render_richtext_content(ui, content);
                                    } else {
                                        // Regular text with better formatting
                                        ui.label(
                                            eframe::egui::RichText::new(content)
                                                .size(13.0)
                                                .color(eframe::egui::Color32::from_gray(220)),
                                        );
                                    }
                                });
                            });

                            ui.add_space(8.0);
                        }
                    });
            });

        // Right panel - Feedback input with fixed layout
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            // Use vertical layout with fixed height for main content
            ui.vertical(|ui| {
                ui.heading("Interactive Feedback MCP");
                ui.add_space(10.0);

                // Scrollable content area with fixed height
                eframe::egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 120.0) // Reserve space for buttons
                    .show(ui, |ui| {
                        // Project information with improved styling
                        let project_frame = eframe::egui::Frame::group(ui.style())
                            .fill(eframe::egui::Color32::from_rgba_premultiplied(20, 30, 20, 200))
                            .stroke(eframe::egui::Stroke::new(1.0, eframe::egui::Color32::from_rgb(100, 255, 150)))
                            .rounding(eframe::egui::Rounding::same(8.0))
                            .inner_margin(eframe::egui::Margin::same(12.0));

                        project_frame.show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(eframe::egui::RichText::new("📁")
                                        .size(16.0));
                                    ui.label(eframe::egui::RichText::new("Project Information")
                                        .size(14.0)
                                        .color(eframe::egui::Color32::from_rgb(100, 255, 150))
                                        .strong());
                                });

                                ui.add_space(8.0);

                                ui.horizontal(|ui| {
                                    ui.label(eframe::egui::RichText::new("Project:")
                                        .size(12.0)
                                        .color(eframe::egui::Color32::from_gray(180)));
                                    ui.label(eframe::egui::RichText::new(&self.project_directory)
                                        .size(12.0)
                                        .color(eframe::egui::Color32::from_gray(220)));
                                });

                                ui.horizontal(|ui| {
                                    ui.label(eframe::egui::RichText::new("Summary:")
                                        .size(12.0)
                                        .color(eframe::egui::Color32::from_gray(180)));
                                });

                                // Summary with text wrapping
                                ui.label(eframe::egui::RichText::new(&self.summary)
                                    .size(12.0)
                                    .color(eframe::egui::Color32::from_gray(220)));
                            });
                        });

                        ui.add_space(10.0);

                        // Cursor context information with improved styling
                        if let Some(context) = &self.cursor_context {
                            let context_frame = eframe::egui::Frame::group(ui.style())
                                .fill(eframe::egui::Color32::from_rgba_premultiplied(20, 20, 40, 200))
                                .stroke(eframe::egui::Stroke::new(1.0, eframe::egui::Color32::from_rgb(150, 100, 255)))
                                .rounding(eframe::egui::Rounding::same(8.0))
                                .inner_margin(eframe::egui::Margin::same(12.0));

                            context_frame.show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(eframe::egui::RichText::new("🔧")
                                            .size(16.0));
                                        ui.label(eframe::egui::RichText::new("Cursor MCP Context")
                                            .size(14.0)
                                            .color(eframe::egui::Color32::from_rgb(150, 100, 255))
                                            .strong());
                                    });

                                    ui.add_space(8.0);

                                    ui.horizontal(|ui| {
                                        ui.label(eframe::egui::RichText::new("Tool:")
                                            .size(12.0)
                                            .color(eframe::egui::Color32::from_gray(180)));
                                        ui.label(eframe::egui::RichText::new(&context.tool_name)
                                            .size(12.0)
                                            .color(eframe::egui::Color32::from_gray(220)));
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(eframe::egui::RichText::new("Method:")
                                            .size(12.0)
                                            .color(eframe::egui::Color32::from_gray(180)));
                                        ui.label(eframe::egui::RichText::new(&context.method)
                                            .size(12.0)
                                            .color(eframe::egui::Color32::from_gray(220)));
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(eframe::egui::RichText::new("Request ID:")
                                            .size(12.0)
                                            .color(eframe::egui::Color32::from_gray(180)));
                                        ui.label(eframe::egui::RichText::new(context.request_id.to_string())
                                            .size(12.0)
                                            .color(eframe::egui::Color32::from_gray(220)));
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(eframe::egui::RichText::new("Timestamp:")
                                            .size(12.0)
                                            .color(eframe::egui::Color32::from_gray(180)));
                                        ui.label(eframe::egui::RichText::new(&context.timestamp)
                                            .size(12.0)
                                            .color(eframe::egui::Color32::from_gray(220)));
                                    });

                                    ui.add_space(8.0);

                                    ui.label(eframe::egui::RichText::new("Arguments:")
                                        .size(12.0)
                                        .color(eframe::egui::Color32::from_gray(180)));

                                    ui.group(|ui| {
                                        ui.code(format!("{}", context.arguments));
                                    });
                                });
                            });
                            ui.add_space(10.0);
                        }

                        // Feedback input section with improved styling
                        let feedback_frame = eframe::egui::Frame::group(ui.style())
                            .fill(eframe::egui::Color32::from_rgba_premultiplied(20, 20, 20, 200))
                            .stroke(eframe::egui::Stroke::new(1.0, eframe::egui::Color32::from_rgb(100, 150, 255)))
                            .rounding(eframe::egui::Rounding::same(8.0))
                            .inner_margin(eframe::egui::Margin::same(16.0));

                        feedback_frame.show(ui, |ui| {
                            ui.vertical(|ui| {
                                // Header with icon and title
                                ui.horizontal(|ui| {
                                    ui.label(eframe::egui::RichText::new("💬")
                                        .size(20.0));
                                    ui.label(eframe::egui::RichText::new("Your Feedback")
                                        .size(16.0)
                                        .color(eframe::egui::Color32::from_rgb(100, 150, 255))
                                        .strong());
                                });

                                ui.add_space(8.0);

                                // Description
                                ui.label(eframe::egui::RichText::new("Please provide your feedback for the development task:")
                                    .size(12.0)
                                    .color(eframe::egui::Color32::from_gray(180)));

                                ui.add_space(12.0);

                                // Text input with better styling
                                let text_edit = eframe::egui::TextEdit::multiline(&mut self.feedback)
                                    .hint_text("Enter your feedback here...\n\nSupports multiline text and markdown formatting:\n• **bold text**\n• *italic text*\n• ```code blocks```\n\nUse Ctrl+Enter to submit.")
                                    .font(eframe::egui::TextStyle::Body)
                                    .desired_width(ui.available_width())
                                    .desired_rows(8);

                                ui.add_sized(
                                    [ui.available_width(), 200.0],
                                    text_edit
                                );

                                ui.add_space(8.0);

                                // Character count and tips
                                ui.horizontal(|ui| {
                                    ui.label(eframe::egui::RichText::new(format!("Characters: {}", self.feedback.len()))
                                        .size(10.0)
                                        .color(eframe::egui::Color32::from_gray(120)));

                                    ui.with_layout(eframe::egui::Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                                        ui.label(eframe::egui::RichText::new("💡 Tip: Use markdown for better formatting")
                                            .size(10.0)
                                            .color(eframe::egui::Color32::from_gray(120)));
                                    });
                                });
                            });
                        });
                    });

                // Fixed bottom section with improved buttons
                ui.add_space(15.0);

                // Error message with better styling
                if let Some(error) = &self.error_message {
                    let error_frame = eframe::egui::Frame::group(ui.style())
                        .fill(eframe::egui::Color32::from_rgba_premultiplied(60, 20, 20, 200))
                        .stroke(eframe::egui::Stroke::new(1.0, eframe::egui::Color32::from_rgb(255, 100, 100)))
                        .rounding(eframe::egui::Rounding::same(6.0))
                        .inner_margin(eframe::egui::Margin::same(12.0));

                    error_frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(eframe::egui::RichText::new("⚠️")
                                .size(16.0));
                            ui.label(eframe::egui::RichText::new(error)
                                .size(13.0)
                                .color(eframe::egui::Color32::from_rgb(255, 200, 200)));
                        });
                    });
                    ui.add_space(10.0);
                }

                // Buttons with improved styling
                ui.horizontal(|ui| {
                    // Submit button
                    let submit_button = eframe::egui::Button::new(eframe::egui::RichText::new("✅ Submit Feedback")
                        .size(14.0)
                        .color(eframe::egui::Color32::WHITE))
                        .fill(eframe::egui::Color32::from_rgb(50, 150, 50))
                        .min_size(eframe::egui::Vec2::new(150.0, 35.0));

                    if ui.add(submit_button).clicked() {
                        self.submit_feedback();
                    }

                    ui.add_space(10.0);

                    // Cancel button
                    let cancel_button = eframe::egui::Button::new(eframe::egui::RichText::new("❌ Cancel")
                        .size(14.0)
                        .color(eframe::egui::Color32::WHITE))
                        .fill(eframe::egui::Color32::from_rgb(150, 50, 50))
                        .min_size(eframe::egui::Vec2::new(100.0, 35.0));

                    if ui.add(cancel_button).clicked() {
                        self.cancel_feedback();
                    }

                    // Spacer to push buttons to the left
                    ui.with_layout(eframe::egui::Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                        ui.label(eframe::egui::RichText::new("Press Ctrl+Enter to submit quickly")
                            .size(10.0)
                            .color(eframe::egui::Color32::from_gray(120)));
                    });
                });
            });

            // Keyboard shortcuts
            if ctx.input(|i| i.key_pressed(eframe::egui::Key::Enter) && i.modifiers.ctrl) {
                self.submit_feedback();
            }

            if ctx.input(|i| i.key_pressed(eframe::egui::Key::Escape)) {
                self.cancel_feedback();
            }
        });
    }
}

/// Run the MCP server
fn run_mcp_server() -> Result<(), AppError> {
    // Initialize logging
    init_logging(tracing::Level::INFO)?;

    // Initialize core services
    let settings_manager = std::sync::Arc::new(SettingsManagerImpl::new());
    let process_manager = std::sync::Arc::new(ProcessManagerImpl::new());
    #[allow(clippy::arc_with_non_send_sync)]
    let event_bus = std::sync::Arc::new(EventBusImpl::new());

    // Create MCP server
    let mut server = MCPServer::new(settings_manager, process_manager, event_bus);

    // Register tools
    server.register_tool(Box::new(InteractiveFeedbackTool::new()));

    // Run the server with stdin/stdout like Go
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    for line in lines {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse JSON request like Go
        let request: MCPRequest = serde_json::from_str(line)?;

        // Handle request and send response like Go
        if let Some(response) = server.handle_request(request)? {
            println!("{}", serde_json::to_string(&response)?);
        }
        // Notifications don't get responses (per JSON-RPC 2.0 spec)
    }

    Ok(())
}

/// Run the GUI application
fn run_gui_app(project_directory: String, summary: String, cursor_context: Option<CursorContext>) {
    println!("egui GUI started with project: {}", project_directory);
    println!("egui GUI started with summary: {}", summary);

    // Initialize logging
    env_logger::init();

    // Create app
    let app = App::new(project_directory, summary, cursor_context);

    // Run the GUI
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
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
            let mut fonts = eframe::egui::FontDefinitions::default();

            // Load Noto Sans font for Vietnamese support
            if let Some(font_data) = FONTS_DIR.get_file("NotoSans-Regular.ttf") {
                fonts.font_data.insert(
                    "noto_sans".to_owned(),
                    eframe::egui::FontData::from_static(font_data.contents()),
                );

                // Use Noto Sans as primary font
                fonts
                    .families
                    .get_mut(&eframe::egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "noto_sans".to_owned());
            }

            cc.egui_ctx.set_fonts(fonts);

            // Set light theme
            cc.egui_ctx.set_visuals(eframe::egui::Visuals::light());

            // Increase text and button sizes
            cc.egui_ctx.style_mut(|style| {
                // Increase font sizes
                style.text_styles.insert(
                    eframe::egui::TextStyle::Heading,
                    eframe::egui::FontId::new(24.0, eframe::egui::FontFamily::Proportional),
                );
                style.text_styles.insert(
                    eframe::egui::TextStyle::Body,
                    eframe::egui::FontId::new(16.0, eframe::egui::FontFamily::Proportional),
                );
                style.text_styles.insert(
                    eframe::egui::TextStyle::Button,
                    eframe::egui::FontId::new(16.0, eframe::egui::FontFamily::Proportional),
                );
                style.text_styles.insert(
                    eframe::egui::TextStyle::Small,
                    eframe::egui::FontId::new(14.0, eframe::egui::FontFamily::Proportional),
                );

                // Increase button and spacing sizes
                style.spacing.button_padding = eframe::egui::vec2(12.0, 8.0);
                style.spacing.item_spacing = eframe::egui::vec2(8.0, 6.0);
                style.spacing.window_margin = eframe::egui::Margin::same(12.0);
            });

            Ok(Box::new(app))
        }),
    );
}

fn main() -> Result<(), AppError> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if running as MCP server
    if args.len() > 1 && args[1] == "--mcp-server" {
        // Run as MCP server
        run_mcp_server()?;
        return Ok(());
    }

    // Check if running as GUI with arguments
    if args.len() > 1 {
        let project_directory = args[1].clone();
        let summary = if args.len() > 2 {
            args[2].clone()
        } else {
            "No summary provided".to_string()
        };

        // Run as GUI application
        run_gui_app(project_directory, summary, None);
        return Ok(());
    }

    // Default: show help
    println!("IFM-Ruta Unified Application");
    println!("Usage:");
    println!(
        "  {} --mcp-server                    # Run as MCP server for Cursor",
        args[0]
    );
    println!(
        "  {} <project_dir> [summary]         # Run as GUI application",
        args[0]
    );
    println!(
        "  {}                                 # Show this help",
        args[0]
    );

    Ok(())
}
