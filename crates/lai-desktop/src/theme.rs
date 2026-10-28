//! One deliberate palette for the desktop app: graphite surfaces, warm text,
//! a restrained amber accent, and chart colors chosen to sit beside it.

use gpui_kit::component::Theme;
use gpui_kit::{App, px, rgb};

pub fn install(cx: &mut App, dark: bool) {
    Theme::change(
        if dark {
            gpui_kit::component::ThemeMode::Dark
        } else {
            gpui_kit::component::ThemeMode::Light
        },
        None,
        cx,
    );
    apply(cx, dark);
}

pub fn apply(cx: &mut App, dark: bool) {
    Theme::update(cx, |theme| {
        theme.font_family = "Red Hat Display".into();
        theme.mono_font_family = "Adwaita Mono".into();
        theme.font_size = px(14.);
        theme.mono_font_size = px(12.5);
        theme.radius = px(7.);
        theme.radius_lg = px(12.);
        theme.shadow = dark;

        let c = &mut theme.colors;
        if dark {
            c.background = rgb(0x1E1A1B).into();
            c.foreground = rgb(0xFAF9F5).into();
            c.border = rgb(0x3B3334).into();
            c.muted = rgb(0x2E2829).into();
            c.muted_foreground = rgb(0xB0A8A1).into();
            c.accent = rgb(0x3A3233).into();
            c.accent_foreground = rgb(0xFAF9F5).into();
            c.primary = rgb(0xEBA31A).into();
            c.primary_foreground = rgb(0x231F20).into();
            c.primary_hover = rgb(0xF5B93F).into();
            c.primary_active = rgb(0xC98A12).into();
            c.secondary = rgb(0x2A2425).into();
            c.secondary_foreground = rgb(0xFAF9F5).into();
            c.secondary_hover = rgb(0x342D2E).into();
            c.secondary_active = rgb(0x3B3334).into();
            c.danger = rgb(0xE0776B).into();
            c.danger_foreground = rgb(0x231F20).into();
            c.danger_hover = rgb(0xE98A7E).into();
            c.danger_active = rgb(0xC4695E).into();
            c.success = rgb(0x7CC08F).into();
            c.success_foreground = rgb(0x0E1A12).into();
            c.success_hover = rgb(0x8CCB9E).into();
            c.success_active = rgb(0x68A97B).into();
            c.warning = rgb(0xD9A441).into();
            c.info = rgb(0xA9B29F).into();
            c.popover = rgb(0x262122).into();
            c.popover_foreground = rgb(0xFAF9F5).into();
            c.list = rgb(0x1E1A1B).into();
            c.list_hover = rgb(0x2F292A).into();
            c.list_active = rgb(0x3A3233).into();
            c.list_active_border = rgb(0x6B552A).into();
            c.list_even = rgb(0x211D1E).into();
            c.list_head = rgb(0x231F20).into();
            c.table = rgb(0x1E1A1B).into();
            c.table_hover = rgb(0x2F292A).into();
            c.table_active = rgb(0x3A3233).into();
            c.table_active_border = rgb(0x6B552A).into();
            c.table_even = rgb(0x211D1E).into();
            c.table_head = rgb(0x231F20).into();
            c.table_head_foreground = rgb(0x9C978E).into();
            c.table_foot = rgb(0x231F20).into();
            c.table_foot_foreground = rgb(0x9C978E).into();
            c.table_row_border = rgb(0x1E2126).into();
            c.selection = rgb(0x3C4756).into();
            c.ring = rgb(0xB47400).into();
            c.title_bar = rgb(0x231F20).into();
            c.status_bar = rgb(0x231F20).into();
            c.sidebar = rgb(0x231F20).into();
            c.sidebar_foreground = rgb(0xE7E1D6).into();
            c.sidebar_accent = rgb(0x332C2D).into();
            c.sidebar_accent_foreground = rgb(0xF4EFE5).into();
            c.sidebar_border = rgb(0x3A3233).into();
            c.sidebar_primary = rgb(0xEBA31A).into();
            c.sidebar_primary_foreground = rgb(0x231F20).into();
            c.caret = rgb(0xEBA31A).into();
            c.drop_target = rgb(0xEBA31A).into();
            c.chart_1 = rgb(0xEBA31A).into();
            c.chart_2 = rgb(0xC2A46B).into();
            c.chart_3 = rgb(0xA9B29F).into();
            c.chart_4 = rgb(0xC97E5A).into();
            c.chart_5 = rgb(0x8E7C6A).into();
            c.button = rgb(0x2A2425).into();
            c.button_foreground = rgb(0xFAF9F5).into();
            c.button_hover = rgb(0x342D2E).into();
            c.button_active = rgb(0x3B3334).into();
            c.button_primary = rgb(0xEBA31A).into();
            c.button_primary_foreground = rgb(0x231F20).into();
            c.button_primary_hover = rgb(0xF5B93F).into();
            c.button_primary_active = rgb(0xC98A12).into();
            c.button_secondary = rgb(0x2A2425).into();
            c.button_secondary_foreground = rgb(0xFAF9F5).into();
            c.button_secondary_hover = rgb(0x342D2E).into();
            c.button_secondary_active = rgb(0x3B3334).into();
            c.button_danger = rgb(0xE0776B).into();
            c.button_danger_foreground = rgb(0x231F20).into();
            c.button_danger_hover = rgb(0xE98A7E).into();
            c.button_danger_active = rgb(0xC4695E).into();
        } else {
            c.background = rgb(0xFAF9F5).into();
            c.foreground = rgb(0x231F20).into();
            c.border = rgb(0xE5DFD4).into();
            c.muted = rgb(0xF1EDE4).into();
            c.muted_foreground = rgb(0x6B635C).into();
            c.accent = rgb(0xF0E9DC).into();
            c.accent_foreground = rgb(0x231F20).into();
            c.primary = rgb(0xB47400).into();
            c.primary_foreground = rgb(0xFFF9EF).into();
            c.primary_hover = rgb(0xC98A12).into();
            c.primary_active = rgb(0x9C6400).into();
            c.secondary = rgb(0xEFE9DE).into();
            c.secondary_foreground = rgb(0x231F20).into();
            c.secondary_hover = rgb(0xF5EFE4).into();
            c.secondary_active = rgb(0xEDE5D6).into();
            c.danger = rgb(0xB4544A).into();
            c.danger_foreground = rgb(0xFFF8F6).into();
            c.danger_hover = rgb(0xC05F54).into();
            c.danger_active = rgb(0x9C463D).into();
            c.success = rgb(0x557A62).into();
            c.success_foreground = rgb(0xF7FBF7).into();
            c.success_hover = rgb(0x61886E).into();
            c.success_active = rgb(0x476A54).into();
            c.warning = rgb(0x9A6B14).into();
            c.info = rgb(0x4A6B8A).into();
            c.popover = rgb(0xFFFFFF).into();
            c.popover_foreground = rgb(0x231F20).into();
            c.list = rgb(0xFFFFFF).into();
            c.list_hover = rgb(0xF4F0E9).into();
            c.list_active = rgb(0xF0E5D2).into();
            c.list_active_border = rgb(0xD8B87A).into();
            c.list_even = rgb(0xFCFBF8).into();
            c.list_head = rgb(0xF5F1E9).into();
            c.table = rgb(0xFFFFFF).into();
            c.table_hover = rgb(0xF4F0E9).into();
            c.table_active = rgb(0xF0E5D2).into();
            c.table_active_border = rgb(0xD8B87A).into();
            c.table_even = rgb(0xFCFBF8).into();
            c.table_head = rgb(0xF5F1E9).into();
            c.table_head_foreground = rgb(0x6B635C).into();
            c.table_foot = rgb(0xF5F1E9).into();
            c.table_foot_foreground = rgb(0x6B635C).into();
            c.table_row_border = rgb(0xEFE9DE).into();
            c.selection = rgb(0xF0E2C4).into();
            c.ring = rgb(0xB47400).into();
            c.title_bar = rgb(0xF3F0E9).into();
            c.status_bar = rgb(0xF3F0E9).into();
            c.sidebar = rgb(0xF3F0E9).into();
            c.sidebar_foreground = rgb(0x231F20).into();
            c.sidebar_accent = rgb(0xE7DFD2).into();
            c.sidebar_accent_foreground = rgb(0x231F20).into();
            c.sidebar_border = rgb(0xE5DFD4).into();
            c.sidebar_primary = rgb(0xB47400).into();
            c.sidebar_primary_foreground = rgb(0xFFF9EF).into();
            c.caret = rgb(0xB47400).into();
            c.drop_target = rgb(0xB47400).into();
            c.chart_1 = rgb(0xC98A12).into();
            c.chart_2 = rgb(0x557A62).into();
            c.chart_3 = rgb(0x4A6B8A).into();
            c.chart_4 = rgb(0xA5644F).into();
            c.chart_5 = rgb(0x6E5D87).into();
            c.button = rgb(0xFFFFFF).into();
            c.button_foreground = rgb(0x231F20).into();
            c.button_hover = rgb(0xF7F3EC).into();
            c.button_active = rgb(0xEFE9DF).into();
            // Brand amber on the primary action, graphite text for contrast.
            c.button_primary = rgb(0xEBA31A).into();
            c.button_primary_foreground = rgb(0x231F20).into();
            c.button_primary_hover = rgb(0xF5B93F).into();
            c.button_primary_active = rgb(0xC98A12).into();
            c.button_secondary = rgb(0xEFE9DE).into();
            c.button_secondary_foreground = rgb(0x231F20).into();
            c.button_secondary_hover = rgb(0xF5EFE4).into();
            c.button_secondary_active = rgb(0xEDE5D6).into();
            c.button_danger = rgb(0xB4544A).into();
            c.button_danger_foreground = rgb(0xFFF8F6).into();
            c.button_danger_hover = rgb(0xC05F54).into();
            c.button_danger_active = rgb(0x9C463D).into();
        }
    });
}
