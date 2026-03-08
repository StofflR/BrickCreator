use crate::brick::base::*;
use crate::common::*;
use crate::types::BrickType;

// constants
const WIDTH: f32 = 348.181;
const HEIGHT: f32 = 16.0;
pub const DEFAULT_Y_OFFSET: f32 = 0.0;
const TYPE: BrickType = BrickType::H0Collapsed;

#[derive(Clone, PartialEq)]
pub struct BrickH0 {
    pub base: BaseBrick,
}

impl std::ops::Deref for BrickH0 {
    type Target = BaseBrick;
    fn deref(&self) -> &BaseBrick {
        &self.base
    }
}

impl std::ops::DerefMut for BrickH0 {
    fn deref_mut(&mut self) -> &mut BaseBrick {
        &mut self.base
    }
}

impl Default for BrickH0 {
    fn default() -> Self {
        Self {
            base: BaseBrick {
                offset: (DEFAULT_X_OFFSET, DEFAULT_Y_OFFSET),
                ..BaseBrick::default()
            },
        }
    }
}

impl Brick for BrickH0 {
    fn get_type(&self) -> crate::types::BrickType {
        TYPE
    }
    fn get_dimensions(&self) -> (u32, u32) {
        (WIDTH as u32, HEIGHT as u32)
    }
}

impl SVGRenderable for BrickH0 {
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

            <g>
                <path class=\"background\" d=\"M0,5h348.181v5h-348.181z\" />
            </g>

             <g>
                <path class=\"background\" d=\"M 0 2 l 11 0 l 1.75 4.13 l 19.5 0 l 1.75 -4.13 l 314.18 0 l 0 4.13 l -348.18 0 z\" />

                <path class=\"border\"
                    d=\"m 347.74 0 l -315.06 0 l 0 0.01 l -1.75 4.12 l -16.86 0 l -1.75 -4.13 l -0.02 0.01 l 0 -0.01 l -12.3 0 l 0 2 l 11 0 l 1.75 4.12 l 0.01 0 l 0 0.01 l 19.5 0 l 0 0 l 0 0 l 1.75 -4.12 L 348.18 2.01 l 0 -2 z\" />
                <path fill-opacity=\"0.4\" fill=\"#fff\"
                    d=\"M347.736,2.063l-313.753,-0.063l0,0.003l-0.008,-0.003l-1.751,4.125l-19.484,0l-1.74,-4.098l0,-0.027l-11,0l0,2l9.665,0l1.753,4.127l0.005,-0.002l22.115,0l0,-0.001l0.008,0.003l1.753,-4.127l312.88,0.063l0,-2z\"
                    stroke-opacity=\"0.4\" />
            </g>

            <g>
                <path class=\"background\"
                    d=\"M0,9.667l12.318,-0.004l1.75,4.125l16.857,0.014l1.75,-4.125l12.367,-0.004l-45.042,-0.006z\" />
                <path class=\"border\"
                    d=\"M347.736,9.657l-315.056,0.001l0,0.008l-1.749,4.117l-16.858,0l-1.752,-4.126l-0.019,0.008l0,-0.007l-12.302,0l0,2l10.998,0l1.748,4.115l0.005,-0.002l0,0.012l19.502,0l0,-0.003l0.002,0.001l1.752,-4.123l314.17,-0.001l0,-2z\" />
            </g>
            {}
        </svg>",
        self.parse_content())
    }
}
