use shared::color::{BLUE_SCHEME, ColorScheme};
use shared::types::BrickType;
use yew::html::Scope;
use yew::{Component, Context, Html, html};

mod components;
use components::{Brick, BrickTypeView, ColorView};

pub enum Message {
    Reset,
    ToggleSidebar,
    UpdateColorScheme(ColorScheme),
    UpdateContent(String),
    UpdateType(BrickType),
}

pub struct App {
    sidebar_collapsed: bool,
    color_scheme: ColorScheme,
    brick_type: BrickType,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            sidebar_collapsed: false,
            color_scheme: BLUE_SCHEME,
            brick_type: BrickType::H0Collapsed,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Reset => true,
            Message::ToggleSidebar => {
                self.sidebar_collapsed = !self.sidebar_collapsed;
                true
            }
            Message::UpdateColorScheme(scheme) => {
                self.color_scheme = scheme;
                true
            }
            Message::UpdateContent(_content) => {
                // Handle content update
                true
            }
            Message::UpdateType(brick_type) => {
                self.brick_type = brick_type;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let sidebar_class = if self.sidebar_collapsed {
            "sidebar collapsed"
        } else {
            "sidebar"
        };

        html! {
            <div class="app-container">
                <div class={sidebar_class}>
                    <button
                        class="toggle-btn"
                        onclick={ctx.link().callback(|_| Message::ToggleSidebar)}
                    >
                        { if self.sidebar_collapsed { "☰" } else { "✕" } }
                    </button>
                    <h1>{ "Brick Creator" }</h1>
                    {self.view_panel(ctx.link())}
                </div>
                <div class="main-content">
                    <Brick color_scheme={self.color_scheme} brick_type={self.brick_type} />
                    <BrickTypeView
                        selected={self.brick_type}
                        on_select={ctx.link().callback(|brick_type| Message::UpdateType(brick_type))}
                    />
                    <ColorView
                        on_select={ctx.link().callback(|scheme| Message::UpdateColorScheme(scheme))}
                    />
                </div>
            </div>
        }
    }
}

impl App {
    fn view_panel(&self, _link: &Scope<Self>) -> Html {
        html! {
            <div>
                <button onclick={_link.callback(|_| Message::Reset)}>{ "Reset" }</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
