use shared::types::BrickType;
use yew::{Callback, Html, Properties, function_component, html};

const ALL_BRICK_TYPES: &[BrickType] = &[
    BrickType::H0Collapsed,
    BrickType::H1Base,
    BrickType::H1Control,
    BrickType::H2Base,
    BrickType::H2Control,
    BrickType::H3Base,
];

fn brick_type_name(brick_type: &BrickType) -> &str {
    match brick_type {
        BrickType::H0Collapsed => "H0 Collapsed",
        BrickType::H1Base => "H1 Base",
        BrickType::H1Control => "H1 Control",
        BrickType::H2Base => "H2 Base",
        BrickType::H2Control => "H2 Control",
        BrickType::H3Base => "H3 Base",
    }
}

fn brick_type_description(brick_type: &BrickType) -> &str {
    match brick_type {
        BrickType::H0Collapsed => "Thin horizontal brick",
        BrickType::H1Base => "Standard base brick - small",
        BrickType::H1Control => "Control brick - small",
        BrickType::H2Base => "Standard base brick - medium",
        BrickType::H2Control => "Control brick - medium",
        BrickType::H3Base => "Standard base brick - large",
    }
}

#[derive(Properties, PartialEq)]
pub struct BrickTypeViewProps {
    pub on_select: Callback<BrickType>,
    #[prop_or(BrickType::H0Collapsed)]
    pub selected: BrickType,
}

#[function_component(BrickTypeView)]
pub fn brick_type_view(props: &BrickTypeViewProps) -> Html {
    html! {
        <div class="brick-type-list">
            <h2>{ "Brick Types" }</h2>
            <div class="brick-type-grid">
                {ALL_BRICK_TYPES.iter().map(|brick_type| {
                    let brick_type_clone = *brick_type;
                    let on_click = props.on_select.clone();
                    let is_selected = props.selected == *brick_type;
                    let item_class = if is_selected {
                        "brick-type-item selected"
                    } else {
                        "brick-type-item"
                    };

                    html! {
                        <div
                            class={item_class}
                            onclick={move |_| on_click.emit(brick_type_clone)}
                        >
                            <div class="brick-type-name">{ brick_type_name(brick_type) }</div>
                            <div class="brick-type-description">{ brick_type_description(brick_type) }</div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
