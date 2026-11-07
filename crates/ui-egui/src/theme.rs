//! Design tokens (light and dark), interface fonts and the egui style.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, Visuals};

/// PowerGIS brand colour (the Project tab and the logo) and its lighter tint.
pub const BRAND: Color32 = Color32::from_rgb(0x0F, 0x62, 0xFE);
pub const BRAND_DARK: Color32 = Color32::from_rgb(0x00, 0x43, 0xCE);
/// Contextual tab group colour (Feature Layer, Labeling, Data).
pub const CONTEXT: Color32 = Color32::from_rgb(0x0E, 0x93, 0x84);

/// Every colour the interface uses; widgets never hard-code colours.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub dark: bool,
    pub header: Color32,
    pub ribbon: Color32,
    pub tab_strip: Color32,
    pub canvas: Color32,
    pub pane: Color32,
    pub pane_header: Color32,
    pub border: Color32,
    pub border_strong: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub icon: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub checked: Color32,
    pub accent: Color32,
    pub accent_text: Color32,
    pub on_accent: Color32,
    pub input: Color32,
    pub input_border: Color32,
    pub group_label: Color32,
    pub status_bar: Color32,
    pub selection: Color32,
    pub menu: Color32,
    pub shadow: Color32,
    pub green: Color32,
    pub red: Color32,
    pub amber: Color32,
    pub context: Color32,
    pub context_tint: Color32,
}

impl Tokens {
    pub fn light() -> Self {
        Tokens {
            dark: false,
            header: Color32::from_rgb(0xF8, 0xFA, 0xFC),
            ribbon: Color32::from_rgb(0xFF, 0xFF, 0xFF),
            tab_strip: Color32::from_rgb(0xF1, 0xF5, 0xF9),
            canvas: Color32::from_rgb(0xEA, 0xEE, 0xF4),
            pane: Color32::from_rgb(0xFF, 0xFF, 0xFF),
            pane_header: Color32::from_rgb(0xF8, 0xFA, 0xFC),
            border: Color32::from_rgb(0xE2, 0xE8, 0xF0),
            border_strong: Color32::from_rgb(0xCB, 0xD5, 0xE1),
            text: Color32::from_rgb(0x0F, 0x17, 0x2A),
            text_dim: Color32::from_rgb(0x64, 0x74, 0x8B),
            text_disabled: Color32::from_rgb(0x94, 0xA3, 0xB8),
            icon: Color32::from_rgb(0x33, 0x41, 0x55),
            hover: Color32::from_rgb(0xF1, 0xF5, 0xF9),
            pressed: Color32::from_rgb(0xE2, 0xE8, 0xF0),
            checked: Color32::from_rgb(0xEE, 0xF2, 0xFF),
            accent: Color32::from_rgb(0x25, 0x63, 0xEB),
            accent_text: Color32::from_rgb(0x1D, 0x4E, 0xD8),
            on_accent: Color32::WHITE,
            input: Color32::WHITE,
            input_border: Color32::from_rgb(0xCB, 0xD5, 0xE1),
            group_label: Color32::from_rgb(0x64, 0x74, 0x8B),
            status_bar: Color32::from_rgb(0xF8, 0xFA, 0xFC),
            selection: Color32::from_rgb(0xDB, 0xEA, 0xFE),
            menu: Color32::WHITE,
            shadow: Color32::from_rgba_unmultiplied(15, 23, 42, 35),
            green: Color32::from_rgb(0x10, 0xB9, 0x81),
            red: Color32::from_rgb(0xEF, 0x44, 0x44),
            amber: Color32::from_rgb(0xF5, 0x9E, 0x0B),
            context: CONTEXT,
            context_tint: Color32::from_rgb(0xCC, 0xFB, 0xF1),
        }
    }

    pub fn dark() -> Self {
        Tokens {
            dark: true,
            header: Color32::from_rgb(0x0A, 0x0E, 0x17),
            ribbon: Color32::from_rgb(0x12, 0x18, 0x26),
            tab_strip: Color32::from_rgb(0x0A, 0x0E, 0x17),
            canvas: Color32::from_rgb(0x08, 0x0B, 0x12),
            pane: Color32::from_rgb(0x0F, 0x14, 0x20),
            pane_header: Color32::from_rgb(0x14, 0x1A, 0x29),
            border: Color32::from_rgb(0x1E, 0x27, 0x38),
            border_strong: Color32::from_rgb(0x2D, 0x3B, 0x54),
            text: Color32::from_rgb(0xF1, 0xF5, 0xF9),
            text_dim: Color32::from_rgb(0x94, 0xA3, 0xB8),
            text_disabled: Color32::from_rgb(0x47, 0x55, 0x69),
            icon: Color32::from_rgb(0xCB, 0xD5, 0xE1),
            hover: Color32::from_rgb(0x1A, 0x23, 0x34),
            pressed: Color32::from_rgb(0x23, 0x30, 0x48),
            checked: Color32::from_rgb(0x1E, 0x2E, 0x4A),
            accent: Color32::from_rgb(0x38, 0xBD, 0xF8),
            accent_text: Color32::from_rgb(0x7D, 0xD3, 0xFC),
            on_accent: Color32::from_rgb(0x03, 0x07, 0x12),
            input: Color32::from_rgb(0x0C, 0x10, 0x1A),
            input_border: Color32::from_rgb(0x28, 0x35, 0x4C),
            group_label: Color32::from_rgb(0x94, 0xA3, 0xB8),
            status_bar: Color32::from_rgb(0x0A, 0x0E, 0x17),
            selection: Color32::from_rgb(0x1E, 0x3A, 0x5F),
            menu: Color32::from_rgb(0x13, 0x19, 0x27),
            shadow: Color32::from_black_alpha(150),
            green: Color32::from_rgb(0x34, 0xD3, 0x99),
            red: Color32::from_rgb(0xF8, 0x71, 0x71),
            amber: Color32::from_rgb(0xFB, 0xBF, 0x24),
            context: Color32::from_rgb(0x2D, 0xD4, 0xBF),
            context_tint: Color32::from_rgb(0x13, 0x36, 0x34),
        }
    }

    pub fn get(ctx: &egui::Context) -> Tokens {
        if ctx.global_style().visuals.dark_mode { Tokens::dark() } else { Tokens::light() }
    }
}

pub fn regular(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("semibold".into()))
}

/// Installs the interface fonts: an installed system sans-serif when available, then `extra`
/// faces (name, bytes, collection index) as fallbacks for scripts it lacks, such as CJK.
pub fn install_fonts(
    ctx: &egui::Context,
    extra: &[(String, Arc<FontData>, Option<powergis_engine::render::fonts::ScriptFont>)],
    current: Option<powergis_engine::render::fonts::ScriptFont>,
) {
    let f = powergis_engine::render::fonts::ui_fonts();
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("ui".into(), Arc::new(FontData::from_owned(f.regular.clone())));
    // Heavier where the face is variable (macOS's system font is one file for every weight, #591),
    // as exports draw it; a face with no weight axis ignores it.
    let weight = egui::epaint::text::VariationCoords::new([(b"wght", powergis_engine::render::fonts::SEMIBOLD_WEIGHT)]);
    let semibold = FontData::from_owned(f.semibold.clone()).tweak(egui::FontTweak { coords: weight, ..Default::default() });
    fonts.font_data.insert("ui-semibold".into(), Arc::new(semibold));
    if let Some(prop) = fonts.families.get_mut(&FontFamily::Proportional) {
        prop.insert(0, "ui".into());
        // egui's Hack face has the Georgian letters that Ubuntu Light and Noto Sans lack.
        prop.push("Hack".into());
    }
    let mut semi = vec!["ui-semibold".to_string(), "ui".to_string()];
    if let Some(prop) = fonts.families.get(&FontFamily::Proportional) {
        semi.extend(prop.iter().skip(1).cloned());
    }
    fonts.families.insert(FontFamily::Name("semibold".into()), semi);
    for (name, data, region) in extra {
        fonts.font_data.insert(name.clone(), Arc::clone(data));
        let primary = region.is_some() && *region == current;
        // Arabic script goes ahead of the interface face: egui shapes each run of one face on its
        // own, so a Persian line reads in order only when this face draws its Latin words too.
        let first = primary && *region == Some(powergis_engine::render::fonts::ScriptFont::Arabic);
        for fam in [FontFamily::Proportional, FontFamily::Name("semibold".into())] {
            if let Some(v) = fonts.families.get_mut(&fam) {
                if first {
                    v.insert(0, name.clone());
                } else if primary {
                    // Right after the interface face, so its regional glyph shapes win.
                    v.insert(1.min(v.len()), name.clone());
                } else {
                    v.push(name.clone());
                }
            }
        }
        // Monospace text (the status bar's bearing and pitch, code boxes) falls back to them too,
        // after Hack, so translated or CJK text in it doesn't draw as boxes.
        if let Some(v) = fonts.families.get_mut(&FontFamily::Monospace) {
            v.push(name.clone());
        }
    }
    ctx.set_fonts(fonts);
}

pub fn apply(ctx: &egui::Context, t: &Tokens) {
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.pane;
    v.window_fill = t.menu;
    v.extreme_bg_color = t.input;
    v.faint_bg_color = t.pane_header;
    v.window_stroke = Stroke::new(1.0, t.border);
    v.window_corner_radius = CornerRadius::same(12);
    v.menu_corner_radius = CornerRadius::same(10);
    v.selection.bg_fill = t.selection;
    v.selection.stroke = Stroke::new(1.0, t.accent);
    v.hyperlink_color = t.accent_text;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.border);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.text);
    // Slider rails and checkbox wells use this fill; keep it visible on white.
    v.widgets.inactive.bg_fill = if t.dark { Color32::from_rgb(0x20, 0x2B, 0x3D) } else { Color32::from_rgb(0xE2, 0xE8, 0xF0) };
    v.slider_trailing_fill = true;
    v.widgets.inactive.weak_bg_fill = t.input;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, t.input_border);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, t.text);
    v.widgets.inactive.corner_radius = CornerRadius::same(7);
    v.widgets.hovered.bg_fill = t.hover;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, t.border_strong);
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, t.text);
    v.widgets.hovered.corner_radius = CornerRadius::same(7);
    v.widgets.active.bg_fill = t.pressed;
    v.widgets.active.weak_bg_fill = t.pressed;
    v.widgets.active.fg_stroke = Stroke::new(1.0, t.text);
    v.widgets.active.corner_radius = CornerRadius::same(7);
    v.widgets.open.bg_fill = t.pressed;
    v.widgets.open.weak_bg_fill = t.pressed;
    v.popup_shadow = egui::Shadow { offset: [0, 8], blur: 24, spread: 0, color: t.shadow };
    v.window_shadow = egui::Shadow { offset: [0, 14], blur: 38, spread: 2, color: t.shadow };
    // Pin egui's theme to ours: the app decides when to follow the system (`dark_follows_system`).
    ctx.set_theme(if t.dark { egui::Theme::Dark } else { egui::Theme::Light });
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 5.0);
        s.spacing.button_padding = egui::vec2(8.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        s.text_styles.insert(egui::TextStyle::Body, regular(12.5));
        s.text_styles.insert(egui::TextStyle::Button, regular(12.5));
        s.text_styles.insert(egui::TextStyle::Small, regular(11.0));
        s.text_styles.insert(egui::TextStyle::Heading, semibold(16.0));
        s.text_styles.insert(egui::TextStyle::Monospace, FontId::monospace(12.0));
    });
}

/// egui colour from a map colour.
pub fn c32(c: powergis_engine::map::Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), c.a())
}

pub fn rgba(c: Color32) -> powergis_engine::map::Rgba {
    let [r, g, b, a] = c.to_srgba_unmultiplied();
    powergis_engine::map::Rgba::new(r, g, b, a)
}
