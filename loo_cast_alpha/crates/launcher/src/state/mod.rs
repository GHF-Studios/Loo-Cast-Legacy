//! Launcher state and routing.

pub const FAVORITE_MODPACKS: [&str; 16] = [
    "Skyforge Prime",
    "Factory Delta",
    "Aetherline",
    "Nomad Survival",
    "Crimson Vault",
    "Rustwood Ultra",
    "Obsidian Fields",
    "Echo Sector",
    "Arctic Circuit",
    "Stormfront Plus",
    "Astral Bloom",
    "Glass Canyon",
    "Nightfall Remix",
    "Dustline Hardcore",
    "Titan Relay",
    "Aurora Fleet",
];

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LauncherPage {
    Home,
    FavoriteModpack(usize),
    ModpackConfigurator,
    DownloadManager,
    Settings,
}

pub struct LauncherState {
    page: LauncherPage,
    hovered_favorite_index: Option<usize>,
    hovered_favorite_started_at: f64,
}

impl Default for LauncherState {
    fn default() -> Self {
        Self {
            page: LauncherPage::Home,
            hovered_favorite_index: None,
            hovered_favorite_started_at: 0.0,
        }
    }
}

impl LauncherState {
    pub fn page(&self) -> LauncherPage {
        self.page
    }

    pub fn favorite_modpacks(&self) -> &'static [&'static str] {
        &FAVORITE_MODPACKS
    }

    pub fn select_home(&mut self) {
        self.page = LauncherPage::Home;
    }

    pub fn select_favorite_modpack(&mut self, index: usize) {
        if index < FAVORITE_MODPACKS.len() {
            self.page = LauncherPage::FavoriteModpack(index);
        }
    }

    pub fn select_modpack_configurator(&mut self) {
        self.page = LauncherPage::ModpackConfigurator;
    }

    pub fn select_download_manager(&mut self) {
        self.page = LauncherPage::DownloadManager;
    }

    pub fn select_settings(&mut self) {
        self.page = LauncherPage::Settings;
    }

    pub fn update_favorite_hover(&mut self, hovered_index: Option<usize>, now_seconds: f64) {
        match (self.hovered_favorite_index, hovered_index) {
            (Some(previous), Some(current)) if previous == current => {}
            (_, Some(current)) => {
                self.hovered_favorite_index = Some(current);
                self.hovered_favorite_started_at = now_seconds;
            }
            (_, None) => {
                self.hovered_favorite_index = None;
            }
        }
    }

    pub fn favorite_hover_elapsed_seconds(&self, index: usize, now_seconds: f64) -> Option<f64> {
        if self.hovered_favorite_index == Some(index) {
            Some((now_seconds - self.hovered_favorite_started_at).max(0.0))
        } else {
            None
        }
    }
}
