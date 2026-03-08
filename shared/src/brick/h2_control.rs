use crate::brick::base::{BaseBrick, DEFAULT_X_OFFSET};
use crate::{common::*, types};

// constants
const WIDTH: f32 = 348.181;
const HEIGHT: f32 = 94.748;
pub const DEFAULT_Y_OFFSET: f32 = 0.17;
const TYPE: crate::types::BrickType = types::BrickType::H2Control;

#[derive(Clone, PartialEq)]
pub struct BrickH2Control {
    pub base: BaseBrick,
}

impl std::ops::Deref for BrickH2Control {
    type Target = BaseBrick;
    fn deref(&self) -> &BaseBrick {
        &self.base
    }
}

impl std::ops::DerefMut for BrickH2Control {
    fn deref_mut(&mut self) -> &mut BaseBrick {
        &mut self.base
    }
}

impl Default for BrickH2Control {
    fn default() -> Self {
        Self {
            base: BaseBrick {
                offset: (DEFAULT_X_OFFSET, DEFAULT_Y_OFFSET),
                ..BaseBrick::default()
            },
        }
    }
}

impl Brick for BrickH2Control {
    fn get_type(&self) -> crate::types::BrickType {
        TYPE
    }
    fn get_dimensions(&self) -> (u32, u32) {
        (WIDTH as u32, HEIGHT as u32)
    }
}

impl SVGRenderable for BrickH2Control {
    fn to_svg(&self) -> String {
        let background_color = &self.color_scheme.color;
        let border_color = &self.color_scheme.border;
        let shade_color = &self.color_scheme.shade;
        format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {WIDTH} {HEIGHT}\">
             <style>
                .background {{ fill: {background_color}; }}
                .border {{ fill: {border_color}; }}
                .shade {{ stop-color: {shade_color}; }}
                text {{ xml:space: \"preserve\" }}
            </style>
            <defs>
                <linearGradient id=\"grad1\" x1=\"0%\" x2=\"0%\" y1=\"0%\" y2=\"100%\">
                    <stop offset=\"0%\" class=\"shade\" stop-opacity=\"0.3\" />
                    <stop offset=\"35%\" class=\"shade\" stop-opacity=\"0\" />
                </linearGradient>
                <linearGradient id=\"grad2\" x1=\"0%\" x2=\"100%\" y1=\"0%\" y2=\"0%\">
                    <stop offset=\"0%\" class=\"shade\" stop-opacity=\"0.125\" />
                    <stop offset=\"2.5%\" class=\"shade\" stop-opacity=\"0\" />
                </linearGradient>
                <linearGradient id=\"grad3\" x1=\"100%\" x2=\"0%\" y1=\"0%\" y2=\"0%\">
                    <stop offset=\"0%\" class=\"shade\" stop-opacity=\"0.125\" />
                    <stop offset=\"2.5%\" class=\"shade\" stop-opacity=\"0\" />
                </linearGradient>

                <g id=\"tabblock\">
                    <g>
                        <rect fill-opacity=\"0.025\" width=\"25.458\" height=\"6.836\" x=\"10.271\" y=\"22.5\" fill=\"#fff\" />
                        <rect width=\"25.458\" height=\"6.836\" x=\"10.271\" y=\"22.5\" fill=\"url(#grad1)\" />
                        <rect width=\"25.458\" height=\"6.836\" x=\"10.271\" y=\"22.5\" fill=\"url(#grad2)\" />
                        <rect width=\"25.458\" height=\"6.836\" x=\"10.271\" y=\"22.5\" fill=\"url(#grad3)\" />
                    </g>
                </g>
            </defs>
            // middle scalable background
            <g>
                <path class=\"background\" d=\"M0,20h348.181v69h-348.181z\" />
            </g>
            // top
            <g>
                <path class=\"border\"
                    d=\"m 214 19 c -12.279 -0.277 -17.75 1 -35.25 -3.25 c -20.8 -5 -66 -15.5 -110.784 -15.748 S 0 9.35 0 9.35 v 14.73 L 348.181 24.222 l 0 -5.222 l -130 0 Z\" />
                <path class=\"background\"
                    d=\"m 212.75 20 c -12.75 0 -17.75 1 -35.25 -3.25 c -20.8 -5.05 -66 -15.5 -109.75 -15.5 s -67.75 9.31 -67.75 9.75 l 0 13.78 l 348.18 0 l 0 -4.78 s -137.18 0 -128.18 0 z\" />
                <path fill-opacity=\"0.4\" fill=\"#fff\"
                    d=\"M 212.75 20 C 200 20 195 21 177.5 16.75 C 156.7 11.7 111.5 1.25 67.06 1.22 S 0 11 0 11 L 0 11.72 V 14.01 S 22.6 4.01 67.04 4.01 S 156.7 14.45 177.5 19.5 C 195 23.75 200 22.75 217.75 22.75 C 220.96 22.75 348.18 22.75 348.18 22.75 V 20 Z\"
                    stroke-opacity=\"0.4\" />
            </g>
            // middle path for bars on the left
            <g>
                <use y=\"1\" href=\"#tabblock\" />
                <use y=\"9\" href=\"#tabblock\" />
                <use y=\"17.2\" href=\"#tabblock\" />
                <use y=\"25.2\" href=\"#tabblock\" />
                <use y=\"33.2\" href=\"#tabblock\" />
                <use y=\"41.2\" href=\"#tabblock\" />
                <use y=\"49.7\" href=\"#tabblock\" />
            </g>

            // bottom
            <g>
                <path class=\"background\"
                    d=\"M0,88.661l12.318,-0.004l1.75,4.125l16.857,0.014l1.75,-4.125l12.367,-0.004l-45.042,-0.006z\" />
                <path class=\"border\"
                    d=\"M347.736,88.657l-315.056,0.001l0,0.008l-1.749,4.117l-16.858,0l-1.752,-4.126l-0.019,0.008l0,-0.007l-12.302,0l0,2l10.998,0l1.748,4.115l0.005,-0.002l0,0.012l19.502,0l0,-0.003l0.002,0.001l1.752,-4.123l314.17-0.001l0,-2z\" />
            </g>
            {}
        </svg>",
        self.parse_content())
    }
}
