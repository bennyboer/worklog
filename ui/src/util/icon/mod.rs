use druid::widget::SvgData;

pub(crate) const ARROW_LEFT: &'static str = include_str!("./assets/arrow_left/arrow_left.svg");
pub(crate) const ARROW_RIGHT: &'static str = include_str!("./assets/arrow_right/arrow_right.svg");
pub(crate) const SLOTH: &'static str = include_str!("./assets/sloth/sloth.svg");

/// Get an SVG icon.
pub(crate) fn get_icon(svg_src: &'static str) -> SvgData {
    match svg_src.parse::<SvgData>() {
        Ok(svg) => svg,
        Err(_) => SvgData::default(),
    }
}
