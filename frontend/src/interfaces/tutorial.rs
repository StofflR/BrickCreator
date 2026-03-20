use std::rc::Rc;

use shared::brick::h0::BrickH0;
use shared::brick::h1_base::BrickH1Base;
use shared::brick::h1_control::BrickH1Control;
use shared::brick::h2_base::BrickH2Base;
use shared::brick::h2_control::BrickH2Control;
use shared::brick::h3_base::BrickH3Base;
use shared::common::{BrickRenderable, Pixmap};
use shared::tutorial::Tutorial;
use yew::Reducible;

use crate::interfaces::brick::BrickState;

#[derive(Clone, Default, PartialEq)]
pub struct TutorialViewState {
    pub tutorial: Tutorial,
    pub selected_index: Option<usize>,
}
pub enum TutorialAction {
    AddBrick(BrickState),
    RemoveSelected,
    ApplyChanges(BrickState),
    Select(usize),
    Deselect,
    MoveEntry(usize, usize),
    LoadJson(String),
}

impl Reducible for TutorialViewState {
    type Action = TutorialAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TutorialAction::AddBrick(brick) => {
                let mut new_state = (*self).clone();
                new_state.add_brick(&brick);
                let len = new_state.tutorial.content.len();
                new_state.selected_index = Some(len - 1);
                Rc::new(new_state)
            }
            TutorialAction::RemoveSelected => {
                let mut new_state = (*self).clone();
                let removed_index = new_state.selected_index;
                let old_len = new_state.tutorial.content.len();
                new_state.remove_selected();
                let len = new_state.tutorial.content.len();
                new_state.selected_index = match (removed_index, len) {
                    (Some(_index), 0) => None,
                    (Some(index), len) => {
                        if index + 1 == old_len {
                            Some(len - 1)
                        } else {
                            Some(index)
                        }
                    }
                    _ => None,
                };
                Rc::new(new_state)
            }
            TutorialAction::ApplyChanges(brick) => {
                let mut new_state = (*self).clone();
                let _ = new_state.apply_changes(&brick);
                Rc::new(new_state)
            }
            TutorialAction::Select(index) => {
                let mut new_state = (*self).clone();
                new_state.selected_index = Some(index);
                Rc::new(new_state)
            }
            TutorialAction::Deselect => {
                let mut new_state = (*self).clone();
                new_state.selected_index = None;
                Rc::new(new_state)
            }
            TutorialAction::MoveEntry(from, to) => {
                let mut new_state = (*self).clone();
                let _ = new_state.move_entry(from, to);
                new_state.selected_index = Some(to);
                Rc::new(new_state)
            }
            TutorialAction::LoadJson(text) => {
                let mut new_state = (*self).clone();
                match new_state.load_json(&text) {
                    Ok(()) => {
                        new_state.selected_index = None;
                        Rc::new(new_state)
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Import error: {e}").into());
                        self
                    }
                }
            }
        }
    }
}

fn box_from_state(state: &BrickState) -> Box<dyn BrickRenderable> {
    let base = state.as_brick().deref().clone();
    match state.get_type() {
        shared::types::BrickType::H0Collapsed => Box::new(BrickH0 { base }),
        shared::types::BrickType::H1Base => Box::new(BrickH1Base { base }),
        shared::types::BrickType::H2Base => Box::new(BrickH2Base { base }),
        shared::types::BrickType::H3Base => Box::new(BrickH3Base { base }),
        shared::types::BrickType::H1Control => Box::new(BrickH1Control { base }),
        shared::types::BrickType::H2Control => Box::new(BrickH2Control { base }),
    }
}

fn state_from_box(brick: &dyn BrickRenderable) -> BrickState {
    let base = brick.deref().clone();
    match brick.get_type() {
        shared::types::BrickType::H0Collapsed => BrickState::H0Collapsed(BrickH0 { base }),
        shared::types::BrickType::H1Base => BrickState::H1Base(BrickH1Base { base }),
        shared::types::BrickType::H2Base => BrickState::H2Base(BrickH2Base { base }),
        shared::types::BrickType::H3Base => BrickState::H3Base(BrickH3Base { base }),
        shared::types::BrickType::H1Control => BrickState::H1Control(BrickH1Control { base }),
        shared::types::BrickType::H2Control => BrickState::H2Control(BrickH2Control { base }),
    }
}

impl TutorialViewState {
    fn add_brick(&mut self, state: &BrickState) {
        self.tutorial.content.push(box_from_state(state));
    }

    pub fn get_brick_state_list(&self) -> Vec<BrickState> {
        self.tutorial
            .content
            .iter()
            .map(|brick| state_from_box(brick.as_ref()))
            .collect()
    }

    fn apply_changes(&mut self, state: &BrickState) -> Result<(), String> {
        let index = self
            .selected_index
            .ok_or_else(|| "No brick selected".to_string())?;
        self.tutorial.content[index] = box_from_state(state);
        Ok(())
    }

    fn remove_selected(&mut self) {
        if let Some(index) = self.selected_index {
            self.tutorial.delete(index);
        }
    }

    fn move_entry(&mut self, from: usize, to: usize) -> Result<(), String> {
        self.tutorial.move_brick(from, to)
    }

    pub fn to_json(&self) -> String {
        self.tutorial.to_json()
    }

    fn load_json(&mut self, json: &str) -> Result<(), String> {
        self.tutorial = Tutorial::from_json(json)?;
        Ok(())
    }

    pub fn get_png_bytes(&self, target_width: u32) -> Result<Vec<u8>, String> {
        let pixmap = self.tutorial.to_pixmap(target_width)?;
        pixmap.encode_png().map_err(|e| e.to_string())
    }

    pub fn get_png(&self, target_width: u32) -> Result<String, String> {
        use base64::Engine;
        let png_bytes = self.get_png_bytes(target_width)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&png_bytes))
    }
}
