//! Launcher UI surfaces.

use std::time::Duration;

use eframe::egui;

use crate::state::{LauncherPage, LauncherState};

const NAV_BAR_WIDTH: f32 = 92.0;
const TOP_SECTION_RATIO: f32 = 0.50;
const SECTION_PADDING: f32 = 8.0;
const SECTION_GAP: f32 = 8.0;
const BOTTOM_SECTION_OUTER_MARGIN: f32 = 16.0;
const BOTTOM_SECTION_ITEM_GAP: f32 = 12.0;
const NAV_ICON_SIZE: f32 = 44.0;
const NAV_ICON_FONT_SIZE: f32 = 20.0;
const HOME_ICON: &str = "⌂";
const MODPACK_CONFIGURATOR_ICON: &str = "▣";
const DOWNLOAD_MANAGER_ICON: &str = "⬇";
const SETTINGS_ICON: &str = "⚙";
const FAVORITE_ITEM_HEIGHT: f32 = 40.0;
const FAVORITE_ITEM_GAP: f32 = 6.0;
const FAVORITE_ICON_SIZE: f32 = 40.0;
const FAVORITE_ICON_FONT_SIZE: f32 = 18.0;
const FAVORITE_LABEL_HOVER_DELAY_SECONDS: f64 = 0.55;

pub struct LauncherUi;

impl LauncherUi {
    pub fn draw(ui: &mut egui::Ui, state: &mut LauncherState) {
        egui::Panel::left("launcher_navigation")
            .resizable(false)
            .exact_size(NAV_BAR_WIDTH)
            .show_separator_line(true)
            .show_inside(ui, |ui| {
                Self::draw_navigation(ui, state);
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            Self::draw_blank_page(ui, state);
        });
    }

    fn draw_navigation(ui: &mut egui::Ui, state: &mut LauncherState) {
        let nav_width = ui.available_width();
        let nav_height = ui.available_height();
        let top_height = (nav_height * TOP_SECTION_RATIO).round();
        let bottom_height = (nav_height - top_height).max(0.0);

        ui.allocate_ui_with_layout(egui::vec2(nav_width, top_height), egui::Layout::top_down(egui::Align::Min), |ui| {
            Self::draw_top_section(ui, state);
        });

        ui.allocate_ui_with_layout(egui::vec2(nav_width, bottom_height), egui::Layout::bottom_up(egui::Align::Min), |ui| {
            Self::draw_bottom_section(ui, state);
        });
    }

    fn draw_top_section(ui: &mut egui::Ui, state: &mut LauncherState) {
        ui.add_space(SECTION_PADDING);
        ui.spacing_mut().item_spacing = egui::vec2(0.0, SECTION_GAP);

        let home_selected = state.page() == LauncherPage::Home;
        let home_response = Self::centered_icon_button(ui, Self::nav_icon_label(HOME_ICON, home_selected), home_selected, NAV_ICON_SIZE).on_hover_text("Home");
        if home_response.clicked() {
            state.select_home();
        }

        ui.add_space(SECTION_GAP);
        let scroll_height = (ui.available_height() - SECTION_PADDING).max(0.0);
        let now_seconds = ui.ctx().input(|input| input.time);
        let mut hovered_favorite_index: Option<usize> = None;
        egui::ScrollArea::vertical()
            .max_height(scroll_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, FAVORITE_ITEM_GAP);

                for (index, modpack_name) in state.favorite_modpacks().iter().enumerate() {
                    let selected = state.page() == LauncherPage::FavoriteModpack(index);
                    let icon = Self::favorite_icon(index);
                    let icon_color = Self::favorite_icon_color(index, selected);
                    let icon_label = egui::RichText::new(icon).size(FAVORITE_ICON_FONT_SIZE).color(icon_color);
                    let response = Self::centered_icon_button(ui, icon_label, selected, FAVORITE_ITEM_HEIGHT.min(FAVORITE_ICON_SIZE));

                    if response.clicked() {
                        state.select_favorite_modpack(index);
                    }

                    if response.hovered() {
                        hovered_favorite_index = Some(index);

                        if let Some(elapsed) = state.favorite_hover_elapsed_seconds(index, now_seconds) {
                            if elapsed >= FAVORITE_LABEL_HOVER_DELAY_SECONDS {
                                response.on_hover_text(*modpack_name);
                            } else {
                                let remaining = FAVORITE_LABEL_HOVER_DELAY_SECONDS - elapsed;
                                ui.ctx()
                                    .request_repaint_after(Duration::from_secs_f64(remaining.clamp(0.01, FAVORITE_LABEL_HOVER_DELAY_SECONDS)));
                            }
                        } else {
                            ui.ctx().request_repaint_after(Duration::from_secs_f64(FAVORITE_LABEL_HOVER_DELAY_SECONDS));
                        }
                    }
                }
            });

        state.update_favorite_hover(hovered_favorite_index, now_seconds);
    }

    fn draw_bottom_section(ui: &mut egui::Ui, state: &mut LauncherState) {
        // In bottom-up layout this first space is the bottom margin.
        ui.add_space(BOTTOM_SECTION_OUTER_MARGIN);

        ui.scope(|ui| {
            let mut style = ui.style().as_ref().clone();
            style.visuals.widgets.inactive.expansion = 0.0;
            style.visuals.widgets.hovered.expansion = 0.0;
            style.visuals.widgets.active.expansion = 0.0;
            style.visuals.widgets.open.expansion = 0.0;
            ui.set_style(style);

            let settings_selected = state.page() == LauncherPage::Settings;
            let settings_response = Self::centered_icon_button(ui, Self::nav_icon_label(SETTINGS_ICON, settings_selected), settings_selected, NAV_ICON_SIZE)
                .on_hover_text("Settings");
            if settings_response.clicked() {
                state.select_settings();
            }

            ui.add_space(BOTTOM_SECTION_ITEM_GAP);

            let downloads_selected = state.page() == LauncherPage::DownloadManager;
            let downloads_response = Self::centered_icon_button(
                ui,
                Self::nav_icon_label(DOWNLOAD_MANAGER_ICON, downloads_selected),
                downloads_selected,
                NAV_ICON_SIZE,
            )
            .on_hover_text("Download Manager");
            if downloads_response.clicked() {
                state.select_download_manager();
            }

            ui.add_space(BOTTOM_SECTION_ITEM_GAP);

            let configurator_selected = state.page() == LauncherPage::ModpackConfigurator;
            let configurator_response = Self::centered_icon_button(
                ui,
                Self::nav_icon_label(MODPACK_CONFIGURATOR_ICON, configurator_selected),
                configurator_selected,
                NAV_ICON_SIZE,
            )
            .on_hover_text("Modpack Configurator");
            if configurator_response.clicked() {
                state.select_modpack_configurator();
            }
        });

        // In bottom-up layout this trailing space becomes top margin above the group.
        ui.add_space(BOTTOM_SECTION_OUTER_MARGIN);
    }

    fn draw_blank_page(_ui: &mut egui::Ui, _state: &LauncherState) {
        // Intentional: all pages are blank in this prototype pass.
    }

    fn favorite_icon(index: usize) -> &'static str {
        const ICONS: [&str; 8] = ["●", "■", "◆", "▲", "▼", "◉", "◈", "⬢"];
        ICONS[index % ICONS.len()]
    }

    fn favorite_icon_color(index: usize, selected: bool) -> egui::Color32 {
        if selected {
            return egui::Color32::from_rgb(240, 240, 240);
        }

        const PALETTE: [egui::Color32; 8] = [
            egui::Color32::from_rgb(92, 186, 255),
            egui::Color32::from_rgb(124, 220, 149),
            egui::Color32::from_rgb(255, 186, 92),
            egui::Color32::from_rgb(196, 149, 255),
            egui::Color32::from_rgb(255, 126, 152),
            egui::Color32::from_rgb(93, 214, 199),
            egui::Color32::from_rgb(255, 219, 111),
            egui::Color32::from_rgb(149, 187, 255),
        ];
        PALETTE[index % PALETTE.len()]
    }

    fn centered_icon_button(ui: &mut egui::Ui, icon_text: egui::RichText, selected: bool, button_size: f32) -> egui::Response {
        let mut response: Option<egui::Response> = None;
        ui.horizontal(|ui| {
            let left_offset = ((ui.available_width() - button_size) * 0.5).max(0.0);
            ui.add_space(left_offset);
            response = Some(ui.add_sized([button_size, button_size], egui::Button::selectable(selected, icon_text)));
        });
        response.expect("centered icon button must create a response")
    }

    fn nav_icon_label(icon: &'static str, selected: bool) -> egui::RichText {
        let color = if selected {
            egui::Color32::from_rgb(240, 240, 240)
        } else {
            egui::Color32::from_rgb(168, 168, 168)
        };
        egui::RichText::new(icon).size(NAV_ICON_FONT_SIZE).color(color)
    }
}
