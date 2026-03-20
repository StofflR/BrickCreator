use std::rc::Rc;

use shared::brick::h0::BrickH0;
use shared::brick::h1_base::BrickH1Base;
use shared::brick::h1_control::BrickH1Control;
use shared::brick::h2_base::BrickH2Base;
use shared::brick::h2_control::BrickH2Control;
use shared::brick::h3_base::BrickH3Base;
use shared::color::ColorScheme;
use shared::common::BrickRenderable;
use shared::types::BrickType;

use yew::Reducible;

#[derive(Clone, PartialEq)]
pub enum BrickState {
    H0Collapsed(BrickH0),
    H1Base(BrickH1Base),
    H2Base(BrickH2Base),
    H3Base(BrickH3Base),
    H1Control(BrickH1Control),
    H2Control(BrickH2Control),
}

impl Default for BrickState {
    fn default() -> Self {
        BrickState::H1Base(BrickH1Base::default())
    }
}

pub enum StateAction {
    ChangeType(BrickType),
    ChangeColor(ColorScheme),
    ChangeOffset(f32, f32),
    ChangeContent(String),
    LoadJson(String),
    Set(BrickState),
}

impl Reducible for BrickState {
    /// Reducer Action Type
    type Action = StateAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            StateAction::ChangeType(new_type) => {
                let mut new_state = (*self).clone();
                new_state.change_type(new_type);
                Rc::new(new_state)
            }
            StateAction::ChangeColor(new_color) => {
                let mut new_state = (*self).clone();
                new_state.as_mut_brick().color_scheme = new_color;
                Rc::new(new_state)
            }
            StateAction::ChangeOffset(x, y) => {
                let mut new_state = (*self).clone();
                new_state.as_mut_brick().offset = (x, y);
                Rc::new(new_state)
            }
            StateAction::ChangeContent(new_content) => {
                let mut new_state = (*self).clone();
                new_state.as_mut_brick().content = new_content;
                Rc::new(new_state)
            }
            StateAction::LoadJson(text) => match BrickState::from_json(&text) {
                Ok(brick) => Rc::new(brick),
                Err(e) => {
                    web_sys::console::error_1(&format!("Import error: {e}").into());
                    self
                }
            },
            StateAction::Set(brick) => Rc::new(brick),
        }
    }
}

impl std::fmt::Display for BrickState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let brick: Box<&dyn BrickRenderable> = Box::new(self.as_brick());
        let result = serde_json::to_string_pretty(&brick).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", result)
    }
}

impl BrickState {
    pub fn from_brick(brick: &dyn BrickRenderable) -> Self {
        let base = brick.deref().clone();
        match brick.get_type() {
            BrickType::H0Collapsed => BrickState::H0Collapsed(BrickH0 { base }),
            BrickType::H1Base => BrickState::H1Base(BrickH1Base { base }),
            BrickType::H2Base => BrickState::H2Base(BrickH2Base { base }),
            BrickType::H3Base => BrickState::H3Base(BrickH3Base { base }),
            BrickType::H1Control => BrickState::H1Control(BrickH1Control { base }),
            BrickType::H2Control => BrickState::H2Control(BrickH2Control { base }),
        }
    }

    pub fn get_type(&self) -> BrickType {
        match self {
            BrickState::H0Collapsed(_) => BrickType::H0Collapsed,
            BrickState::H1Base(_) => BrickType::H1Base,
            BrickState::H2Base(_) => BrickType::H2Base,
            BrickState::H3Base(_) => BrickType::H3Base,
            BrickState::H1Control(_) => BrickType::H1Control,
            BrickState::H2Control(_) => BrickType::H2Control,
        }
    }

    pub fn change_type(&mut self, brick_type: BrickType) {
        let mut base = self.as_brick().deref().clone();
        let default_y_offset = match brick_type {
            BrickType::H0Collapsed => shared::brick::h0::DEFAULT_Y_OFFSET,
            BrickType::H1Base => shared::brick::h1_base::DEFAULT_Y_OFFSET,
            BrickType::H2Base => shared::brick::h2_base::DEFAULT_Y_OFFSET,
            BrickType::H3Base => shared::brick::h3_base::DEFAULT_Y_OFFSET,
            BrickType::H1Control => shared::brick::h1_control::DEFAULT_Y_OFFSET,
            BrickType::H2Control => shared::brick::h2_control::DEFAULT_Y_OFFSET,
        };
        base.offset.1 = default_y_offset;
        *self = match brick_type {
            BrickType::H0Collapsed => BrickState::H0Collapsed(BrickH0 { base }),
            BrickType::H1Base => BrickState::H1Base(BrickH1Base { base }),
            BrickType::H2Base => BrickState::H2Base(BrickH2Base { base }),
            BrickType::H3Base => BrickState::H3Base(BrickH3Base { base }),
            BrickType::H1Control => BrickState::H1Control(BrickH1Control { base }),
            BrickType::H2Control => BrickState::H2Control(BrickH2Control { base }),
        };
    }

    pub fn as_brick(&self) -> &dyn BrickRenderable {
        match self {
            BrickState::H0Collapsed(brick) => brick,
            BrickState::H1Base(brick) => brick,
            BrickState::H2Base(brick) => brick,
            BrickState::H3Base(brick) => brick,
            BrickState::H1Control(brick) => brick,
            BrickState::H2Control(brick) => brick,
        }
    }

    pub fn as_mut_brick(&mut self) -> &mut dyn BrickRenderable {
        match self {
            BrickState::H0Collapsed(brick) => brick,
            BrickState::H1Base(brick) => brick,
            BrickState::H2Base(brick) => brick,
            BrickState::H3Base(brick) => brick,
            BrickState::H1Control(brick) => brick,
            BrickState::H2Control(brick) => brick,
        }
    }

    pub fn get_svg(self) -> String {
        self.as_brick().to_svg()
    }
    pub fn get_png(self, target_width: u32) -> Result<Vec<u8>, String> {
        self.as_brick()
            .to_pixmap(target_width)?
            .encode_png()
            .map_err(|error| error.to_string())
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let brick =
            serde_json::from_str::<Box<dyn BrickRenderable>>(json).map_err(|e| e.to_string())?;
        Ok(BrickState::from_brick(&*brick))
    }
}
