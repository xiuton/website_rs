//! 圆形生成器页面配置与持久化。

use crate::utils::storage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct PageConfig {
    pub(super) config_width: f64,
    pub(super) config_height: f64,
    pub(super) gap: f64,
    pub(super) min_radius: f64,
    pub(super) max_radius: f64,
    pub(super) picker_speed: f64,
    pub(super) anim_duration: f64,
    pub(super) dwell_time: f64,
}

impl Default for PageConfig {
    fn default() -> Self {
        Self {
            config_width: 800.0,
            config_height: 600.0,
            gap: 10.0,
            min_radius: 10.0,
            max_radius: 60.0,
            picker_speed: 200.0,
            anim_duration: 60.0,
            dwell_time: 100.0,
        }
    }
}

pub(super) fn save_config(config: &PageConfig) {
    if let Ok(json) = serde_json::to_string(config) {
        storage::set_circle_generator_config(&json);
    }
}

pub(super) fn load_config() -> PageConfig {
    if let Some(json) = storage::get_circle_generator_config() {
        if let Ok(config) = serde_json::from_str(&json) {
            return config;
        }
    }
    PageConfig::default()
}

pub(super) fn save_all_config(config: &PageConfig) {
    save_config(config);
}

pub(super) fn hsl_color(index: usize, total: usize, dark_mode: bool) -> String {
    let hue = (index as f64 / total.max(1) as f64) * 360.0;
    if dark_mode {
        format!("hsl({}, 70%, 70%)", hue as u32)
    } else {
        format!("hsl({}, 65%, 60%)", hue as u32)
    }
}
