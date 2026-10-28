//! OriginLM brand artwork supplied with the product asset pack.

use gpui_kit::base::StyledExt as _;
use gpui_kit::*;

const MARK_WIDTH: f32 = 545.0;
const MARK_HEIGHT: f32 = 336.0;

/// The app icon: graphite symbol on the amber tile, exactly as shipped.
/// Amber and graphite are fixed brand values, so this reads the same in both
/// themes and matches the real window icon.
pub fn app_tile(size: f32) -> AnyElement {
    div()
        .size(px(size))
        .flex_shrink_0()
        .rounded(px(size * 0.24))
        .bg(rgb(0xEBA31A))
        .h_flex()
        .items_center()
        .justify_center()
        .child(
            gpui_kit::component::Icon::default()
                .data(include_bytes!(
                    "../assets/brand/originlm-mark-small-graphite.svg"
                ))
                .w(px(size * 0.68))
                .h(px(size * 0.68 * MARK_HEIGHT / MARK_WIDTH)),
        )
        .into_any_element()
}
