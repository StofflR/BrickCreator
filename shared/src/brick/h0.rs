use crate::brick::base::BrickSVG;
use crate::color::ColorScheme;
use yew::{Html, html};

// constants
const WIDTH: u32 = 350;
const LENGTH: u32 = 16;

pub struct BrickH0 {
    content: String,
}

impl BrickH0 {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
}

impl BrickSVG for BrickH0 {
    fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    fn to_svg(&self, color_scheme: &ColorScheme) -> Html {
        html! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox={format!("0 0 {} {}", WIDTH, LENGTH)}>
             <style>
                {format!(".background {{ fill: {}; }}", color_scheme.color)}
                {format!(".border {{ fill: {}; }}", color_scheme.border)}
                {format!(".shade {{ stop-color: {}; }}", color_scheme.shade)}
            </style>

            <g viewBox="0 10 350 16">
                <path class="background" d="M0,5h355v5h-355z" />
            </g>

            <g viewbox="0 0 350 16">
                <path class="background" d="M0,2.000l11,0l1.75,4.125l19.5,0l1.75,-4.125l320,0l0,4.125l-355,0l0,-4.125z" />

                <path class="border"
                    d="M347.736,0l-315.056,0.001l0,0.008l-1.749,4.117l-16.858,0l-1.752,-4.126l-0.019,0.008l0,-0.007l-12.302,0l0,2l10.998,0l1.748,4.115l0.005,-0.002l0,0.012l19.502,0l0,-0.003l0.002,0.001l1.752,-4.123l320,-0.001l0,-2z" />
                <path fill-opacity="0.4" fill="#fff"
                    d="M347.736,2.063l-313.753,-0.063l0,0.003l-0.008,-0.003l-1.751,4.125l-19.484,0l-1.74,-4.098l0,-0.027l-11,0l0,2l9.665,0l1.753,4.127l0.005,-0.002l22.115,0l0,-0.001l0.008,0.003l1.753,-4.127l320,0.063l0,-2z"
                    stroke-opacity="0.4" />
            </g>

            <g viewbox="0 0 350 16">
                <path class="background"
                    d="M0,9.667l12.318,-0.004l1.75,4.125l16.857,0.014l1.75,-4.125l12.367,-0.004l-45.042,-0.006z" />
                <path class="border"
                    d="M347.736,9.657l-315.056,0.001l0,0.008l-1.749,4.117l-16.858,0l-1.752,-4.126l-0.019,0.008l0,-0.007l-12.302,0l0,2l10.998,0l1.748,4.115l0.005,-0.002l0,0.012l19.502,0l0,-0.003l0.002,0.001l1.752,-4.123l317.729,-0.001l0,-2z" />
            </g>
            {self.content_to_svg()}
        </svg>
        }
    }
}
