use crate::color::ColorScheme;
use yew::{Html, html};

pub trait BrickSVG {
    fn to_svg(&self, color_scheme: &ColorScheme) -> Html;
    fn set_content(&mut self, content: &str);
    fn get_content(&self) -> &str;

    fn content_to_svg(&self) -> Html {
        html! {
            <text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" fill="currentColor" font-size="4">
                { self.get_content() }
            </text>
        }
    }
}
