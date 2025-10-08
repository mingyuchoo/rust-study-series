use crate::domain::MessageType;
use crate::presentation::controllers::AppController;
use crate::presentation::ui::{AppTab, show_auth_tab, show_people_tab, show_query_tab, show_session_tab};
use egui::{Color32, RichText};

pub struct SurrealDbApp {
    controller: AppController,
}

impl SurrealDbApp {
    pub fn new(controller: AppController) -> Self {
        Self {
            controller,
        }
    }

    fn show_messages(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.label(RichText::new("Messages").heading().color(Color32::WHITE));

        for message in &self.controller.state.messages {
            let color = match message.msg_type {
                | MessageType::Success => Color32::from_rgb(0, 200, 0),
                | MessageType::Error => Color32::from_rgb(200, 0, 0),
            };

            let elapsed = message.timestamp.elapsed().as_secs();
            ui.horizontal(|ui| {
                ui.label(RichText::new(&message.content).color(color));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("{}s ago", elapsed)).small().color(Color32::GRAY));
                });
            });
        }
    }
}

impl eframe::App for SurrealDbApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle responses
        self.controller.handle_response();

        // Top panel with tabs and status
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("SurrealDB Manager");

                ui.separator();

                // Tab buttons
                if ui.selectable_label(self.controller.state.current_tab == AppTab::Session, "Session").clicked() {
                    self.controller.state.current_tab = AppTab::Session;
                }
                if ui
                    .selectable_label(self.controller.state.current_tab == AppTab::Authentication, "Auth")
                    .clicked()
                {
                    self.controller.state.current_tab = AppTab::Authentication;
                }
                if ui.selectable_label(self.controller.state.current_tab == AppTab::People, "People").clicked() {
                    self.controller.state.current_tab = AppTab::People;
                }
                if ui.selectable_label(self.controller.state.current_tab == AppTab::Query, "Query").clicked() {
                    self.controller.state.current_tab = AppTab::Query;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Connection status
                    let status_color = if self.controller.state.connection_status.contains("Connected") {
                        Color32::from_rgb(0, 200, 0)
                    } else {
                        Color32::from_rgb(200, 0, 0)
                    };
                    ui.label(RichText::new(&self.controller.state.connection_status).color(status_color));

                    if self.controller.state.is_loading {
                        ui.spinner();
                    }
                });
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| match self.controller.state.current_tab {
            | AppTab::People => show_people_tab(&mut self.controller, ui),
            | AppTab::Authentication => show_auth_tab(&mut self.controller, ui),
            | AppTab::Query => show_query_tab(&mut self.controller, ui),
            | AppTab::Session => show_session_tab(&mut self.controller, ui),
        });

        // Bottom panel for messages
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.show_messages(ui);
        });
    }
}
