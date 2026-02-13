use shared::brick::base::BrickSVG;
use shared::brick::{h0, h1_base, h1_control, h2_base, h2_control, h3_base};
use shared::color::{BLUE_SCHEME, ColorScheme};
use shared::types::BrickType;
use yew::{Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct BrickProps {
    #[prop_or(BLUE_SCHEME)]
    pub color_scheme: ColorScheme,

    #[prop_or(BrickType::H1Base)]
    pub brick_type: BrickType,
}

#[function_component(Brick)]
pub fn brick(props: &BrickProps) -> Html {
    let brick: Box<dyn BrickSVG> = match props.brick_type {
        BrickType::H0Collapsed => Box::new(h0::BrickH0::new()),
        BrickType::H1Base => Box::new(h1_base::BrickH1Base::new()),
        BrickType::H1Control => Box::new(h1_control::BrickH1Control::new()),
        BrickType::H2Base => Box::new(h2_base::BrickH2Base::new()),
        BrickType::H2Control => Box::new(h2_control::BrickH2Control::new()),
        BrickType::H3Base => Box::new(h3_base::BrickH3Base::new()),
    };

    html! {
        <div class="brick-container">
            {brick.to_svg(&props.color_scheme)}
        </div>
    }
}
