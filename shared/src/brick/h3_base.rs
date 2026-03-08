use crate::brick::base::{BaseBrick, DEFAULT_X_OFFSET};
use crate::{common::*, types};

// constants
const WIDTH: f32 = 348.181;
const HEIGHT: f32 = 94.748;
pub const DEFAULT_Y_OFFSET: f32 = 0.03;
const TYPE: crate::types::BrickType = types::BrickType::H3Base;

#[derive(Clone, PartialEq)]
pub struct BrickH3Base {
    pub base: BaseBrick,
}

impl std::ops::Deref for BrickH3Base {
    type Target = BaseBrick;
    fn deref(&self) -> &BaseBrick {
        &self.base
    }
}

impl std::ops::DerefMut for BrickH3Base {
    fn deref_mut(&mut self) -> &mut BaseBrick {
        &mut self.base
    }
}

impl Default for BrickH3Base {
    fn default() -> Self {
        Self {
            base: BaseBrick {
                offset: (DEFAULT_X_OFFSET, DEFAULT_Y_OFFSET),
                ..BaseBrick::default()
            },
        }
    }
}

impl Brick for BrickH3Base {
    fn get_type(&self) -> crate::types::BrickType {
        TYPE
    }
    fn get_dimensions(&self) -> (u32, u32) {
        (WIDTH as u32, HEIGHT as u32)
    }
}

impl SVGRenderable for BrickH3Base {
    fn to_svg(&self) -> String {
        let background_color = &self.color_scheme.color;
        let border_color = &self.color_scheme.border;
        let shade_color = &self.color_scheme.shade;
        format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {WIDTH} {HEIGHT}\">
             <style>
                .background {{ fill: {background_color}; }}
                .border {{ fill: {border_color}; }}
                .shade {{ stop-color: {shade_color}; }}
                text {{ xml:space=\"preserve\" }}
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
                        <rect fill-opacity=\"0.025\" width=\"25.458\" height=\"4.836\" x=\"10.271\" y=\"22.5\" fill=\"#fff\" />
                        <rect width=\"25.458\" height=\"4.836\" x=\"10.271\" y=\"22.5\" fill=\"url(#grad1)\" />
                        <rect width=\"25.458\" height=\"4.836\" x=\"10.271\" y=\"22.5\" fill=\"url(#grad2)\" />
                        <rect width=\"25.458\" height=\"4.836\" x=\"10.271\" y=\"22.5\" fill=\"url(#grad3)\" />
                    </g>
                </g>
            </defs>
            // middle scalable background
            <g>
                <path class=\"background\" d=\"M0,5h348.181v85h-348.181z\" />
            </g>
            // top
            <g>
                <path class=\"background\" d=\"M 0 2 l 11 0 l 1.75 4.13 l 19.5 0 l 1.75 -4.13 l 314.18 0 l 0 4.13 l -348.18 0 z\" />

                <path class=\"border\"
                    d=\"m 347.74 0 l -315.06 0 l 0 0.01 l -1.75 4.12 l -16.86 0 l -1.75 -4.13 l -0.02 0.01 l 0 -0.01 l -12.3 0 l 0 2 l 11 0 l 1.75 4.12 l 0.01 0 l 0 0.01 l 19.5 0 l 0 0 l 0 0 l 1.75 -4.12 L 348.18 2.01 l 0 -2 z\" />
                <path fill-opacity=\"0.4\" fill=\"#fff\"
                    d=\"M347.736,2.063l-313.753,-0.063l0,0.003l-0.008,-0.003l-1.751,4.125l-19.484,0l-1.74,-4.098l0,-0.027l-11,0l0,2l9.665,0l1.753,4.127l0.005,-0.002l22.115,0l0,-0.001l0.008,0.003l1.753,-4.127l312.88,0.063l0,-2z\"
                    stroke-opacity=\"0.4\" />
            </g>
            // middle path for bars on the left
            <g>
                <use y=\"-5\" href=\"#tabblock\" />
                <use y=\"1.05\" href=\"#tabblock\" />
                <use y=\"7.1\" href=\"#tabblock\" />
                <use y=\"13.15\" href=\"#tabblock\" />
                <use y=\"19.2\" href=\"#tabblock\" />
                <use y=\"25.25\" href=\"#tabblock\" />
                <use y=\"31.3\" href=\"#tabblock\" />
                <use y=\"37.35\" href=\"#tabblock\" />
                <use y=\"43.4\" href=\"#tabblock\" />
                <use y=\"49.45\" href=\"#tabblock\" />
                <use y=\"55.5\" href=\"#tabblock\" />
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
