use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use egui::{Color32, Stroke};
use image::DynamicImage;
use anyhow::Result;

use crate::kreide::types::RPG_GameCore_AvatarPropertyType;

/// Global cache: avatar_id -> PNG buffer
/// Populated in on_set_lineup, cleared on every call
static AVATAR_BUFFER_CACHE: LazyLock<Mutex<HashMap<u32, Vec<u8>>>> = 
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Global cache: monster_id -> PNG buffer
/// Cleared on every lineup set and repopulated as enemies initialize
static MONSTER_BUFFER_CACHE: LazyLock<Mutex<HashMap<u32, Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Global cache: property_name -> PNG buffer
/// Populated on demand as properties are encountered
static PROPERTY_BUFFER_CACHE: LazyLock<Mutex<HashMap<String, Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn clear_avatar_buffers() {
    let mut cache = AVATAR_BUFFER_CACHE.lock().unwrap();
    cache.clear();
}

pub fn clear_property_buffers() {
    let mut cache = PROPERTY_BUFFER_CACHE.lock().unwrap();
    cache.clear();
}

pub fn format_damage(value: f64) -> String {
    if value >= 1_000_000.0 {
        let m = value / 1_000_000.0;
        format!("{:.1}M", m)
    } else if value >= 1_000.0 {
        format!("{}K", (value / 1_000.0).floor())
    } else {
        format!("{}", value.floor())
    }
}

pub fn get_character_color(index: usize) -> egui::Color32 {
    const COLORS: &[egui::Color32] = &[
        egui::Color32::from_rgb(255, 99, 132),
        egui::Color32::from_rgb(54, 162, 235),
        egui::Color32::from_rgb(255, 206, 86),
        egui::Color32::from_rgb(75, 192, 192),
        egui::Color32::from_rgb(153, 102, 255),
        egui::Color32::from_rgb(255, 159, 64),
        egui::Color32::from_rgb(231, 233, 237),
        egui::Color32::from_rgb(102, 255, 102),
    ];

    COLORS[index % COLORS.len()]
}

pub fn wrap_character_name(name: &str, max_line_length: usize) -> String {
    if name.len() <= max_line_length {
        return name.to_string();
    }

    let words: Vec<&str> = name.split_whitespace().collect();
    if words.is_empty() {
        return name.to_string();
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if !current_line.is_empty() && current_line.len() + 1 + word.len() > max_line_length {
            lines.push(current_line.clone());
            current_line.clear();
        }

        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines.join("\n")
}

pub fn get_window_frame(ctx: &egui::Context, opacity: f32) -> egui::Frame {
    let opacity = opacity.clamp(0.0, 1.0);
    let color = ctx.style().visuals.extreme_bg_color.gamma_multiply(opacity);
    egui::Frame::new()
        .fill(color)
        .stroke(Stroke::new(0.5, Color32::WHITE))
        .inner_margin(8.0)
        .corner_radius(10.0)
}

pub fn get_transparent_window_frame(ctx: &egui::Context, opacity: f32) -> egui::Frame {
    egui::Frame::new().inner_margin(8.0).corner_radius(10.0)
}

/// Clear and populate the avatar buffer cache with the given avatar IDs
pub fn populate_avatar_buffers(avatar_ids: &Vec<u32>) {
    let mut cache = AVATAR_BUFFER_CACHE.lock().unwrap();
    cache.clear();
    
    for &avatar_id in avatar_ids {
        match crate::kreide::helpers::get_avatar_png_bytes(avatar_id) {
            Ok(buffer) => {
                cache.insert(avatar_id, buffer);
            }
            Err(e) => {
                log::warn!("Failed to cache avatar icon {}: {}", avatar_id, e);
            }
        }
    }
}

/// Clear all cached monster icon PNG buffers
pub fn clear_monster_buffers() {
    let mut cache = MONSTER_BUFFER_CACHE.lock().unwrap();
    cache.clear();
}

/// Cache a monster icon PNG buffer if it is not already cached
pub fn cache_monster_buffer(monster_id: u32) {
    let mut cache = MONSTER_BUFFER_CACHE.lock().unwrap();
    if cache.contains_key(&monster_id) {
        return;
    }

    match crate::kreide::helpers::get_monster_png_bytes(monster_id) {
        Ok(buffer) => {
            cache.insert(monster_id, buffer);
        }
        Err(e) => {
            log::warn!("Failed to cache monster icon {}: {}", monster_id, e);
        }
    }
}

/// Cache a property icon PNG buffer if it is not already cached
pub fn cache_property_buffer(property_name: RPG_GameCore_AvatarPropertyType) {
    let mut cache = PROPERTY_BUFFER_CACHE.lock().unwrap();
    if cache.contains_key(&property_name.to_string()) {
        return;
    }

    match crate::kreide::helpers::get_property_icon_png_bytes(&property_name.to_string()) {
        Ok(buffer) => {
            cache.insert(property_name.to_string(), buffer);
        }
        Err(e) => {
            log::warn!("Failed to cache property icon {}: {}", property_name, e);
        }
    }
}

pub fn load_avatar_image(ctx: &egui::Context, avatar_id: u32, options: egui::TextureOptions) -> Option<egui::TextureHandle> {
    const IMAGE_CACHE_ID: &str = "ui.helpers.avatar_image_cache";
    type TextureCache = HashMap<u32, egui::TextureHandle>;

    let cache_id = egui::Id::new(IMAGE_CACHE_ID);

    // Check if already cached as texture
    if let Some(cached_handle) = ctx.data(|data| {
        data.get_temp::<TextureCache>(cache_id)
            .and_then(|cache| cache.get(&avatar_id).cloned())
    }) {
        return Some(cached_handle);
    }

    // Get buffer from global cache (populated in on_set_lineup)
    let buffer = {
        let cache = AVATAR_BUFFER_CACHE.lock().unwrap();
        cache.get(&avatar_id).cloned()?
    };

    use image::EncodableLayout;
    let image = image::load_from_memory_with_format(&buffer, image::ImageFormat::Png).ok()?;
    let color_image = match &image {
        DynamicImage::ImageRgb8(image) => {
            egui::ColorImage::from_rgb(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            )
        }
        other => {
            let image = other.to_rgba8();
            egui::ColorImage::from_rgba_unmultiplied(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            )
        }
    };
    
    // Cache the texture handle by avatar_id
    let name = format!("avatar_{}", avatar_id);
    let handle = ctx.load_texture(&name, color_image, options);
    ctx.data_mut(|data| {
        let mut cache = data.get_temp::<TextureCache>(cache_id).unwrap_or_default();
        cache.insert(avatar_id, handle.clone());
        data.insert_temp(cache_id, cache);
    });
    Some(handle)
}

pub fn load_monster_image(
    ctx: &egui::Context,
    monster_id: u32,
    options: egui::TextureOptions,
) -> Option<egui::TextureHandle> {
    const IMAGE_CACHE_ID: &str = "ui.helpers.monster_image_cache";
    type TextureCache = HashMap<u32, egui::TextureHandle>;

    let cache_id = egui::Id::new(IMAGE_CACHE_ID);

    // Check if already cached as texture
    if let Some(cached_handle) = ctx.data(|data| {
        data.get_temp::<TextureCache>(cache_id)
            .and_then(|cache| cache.get(&monster_id).cloned())
    }) {
        return Some(cached_handle);
    }

    // Get buffer from global cache (populated during enemy initialization)
    let buffer = {
        let cache = MONSTER_BUFFER_CACHE.lock().unwrap();
        cache.get(&monster_id).cloned()?
    };

    use image::EncodableLayout;
    let image = image::load_from_memory_with_format(&buffer, image::ImageFormat::Png).ok()?;
    let color_image = match &image {
        DynamicImage::ImageRgb8(image) => {
            egui::ColorImage::from_rgb(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            )
        }
        other => {
            let image = other.to_rgba8();
            egui::ColorImage::from_rgba_unmultiplied(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            )
        }
    };

    // Cache the texture handle by monster_id
    let name = format!("monster_{}", monster_id);
    let handle = ctx.load_texture(&name, color_image, options);
    ctx.data_mut(|data| {
        let mut cache = data.get_temp::<TextureCache>(cache_id).unwrap_or_default();
        cache.insert(monster_id, handle.clone());
        data.insert_temp(cache_id, cache);
    });
    Some(handle)
}

pub fn load_property_icon_image(
    ctx: &egui::Context,
    property_name: &str,
    options: egui::TextureOptions,
) -> Option<egui::TextureHandle> {
    const IMAGE_CACHE_ID: &str = "ui.helpers.property_icon_cache";
    type TextureCache = HashMap<String, egui::TextureHandle>;

    let cache_id = egui::Id::new(IMAGE_CACHE_ID);

    // Check if already cached as texture
    if let Some(cached_handle) = ctx.data(|data| {
        data.get_temp::<TextureCache>(cache_id)
            .and_then(|cache| cache.get(property_name).cloned())
    }) {
        return Some(cached_handle);
    }

    // Get buffer from global cache (populated on demand)
    let buffer = {
        let cache = PROPERTY_BUFFER_CACHE.lock().unwrap();
        cache.get(property_name).cloned()?
    };

    use image::EncodableLayout;
    let image = image::load_from_memory_with_format(&buffer, image::ImageFormat::Png).ok()?;
    let color_image = match &image {
        DynamicImage::ImageRgb8(image) => {
            egui::ColorImage::from_rgb(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            )
        }
        other => {
            let image = other.to_rgba8();
            egui::ColorImage::from_rgba_unmultiplied(
                [image.width() as usize, image.height() as usize],
                image.as_bytes(),
            )
        }
    };

    // Cache the texture handle by property_name
    let name = format!("property_{}", property_name);
    let handle = ctx.load_texture(&name, color_image, options);
    ctx.data_mut(|data| {
        let mut cache = data.get_temp::<TextureCache>(cache_id).unwrap_or_default();
        cache.insert(property_name.to_string(), handle.clone());
        data.insert_temp(cache_id, cache);
    });
    Some(handle)
}