use serde_json::Value;

use crate::{
    brick::h0::BrickH0,
    common::{BrickRenderable, Pixmap},
};

#[derive(Default)]
pub struct Tutorial {
    pub name: String,
    pub content: Vec<Box<dyn BrickRenderable>>,
}

impl PartialEq for Tutorial {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.content.len() == other.content.len()
            && self
                .content
                .iter()
                .zip(other.content.iter())
                .all(|(a, b)| a.get_type() == b.get_type() && a.deref() == b.deref())
    }
}

fn clone_brick(brick: &dyn BrickRenderable) -> Box<dyn BrickRenderable> {
    let base = brick.deref().clone();
    match brick.get_type() {
        crate::types::BrickType::H0Collapsed => Box::new(BrickH0 { base }),
        crate::types::BrickType::H1Base => Box::new(crate::brick::h1_base::BrickH1Base { base }),
        crate::types::BrickType::H2Base => Box::new(crate::brick::h2_base::BrickH2Base { base }),
        crate::types::BrickType::H3Base => Box::new(crate::brick::h3_base::BrickH3Base { base }),
        crate::types::BrickType::H1Control => {
            Box::new(crate::brick::h1_control::BrickH1Control { base })
        }
        crate::types::BrickType::H2Control => {
            Box::new(crate::brick::h2_control::BrickH2Control { base })
        }
    }
}

impl Clone for Tutorial {
    fn clone(&self) -> Self {
        Tutorial {
            name: self.name.clone(),
            content: self.content.iter().map(|brick| clone_brick(brick.as_ref())).collect(),
        }
    }
}

impl Tutorial {
    pub fn length(&self) -> usize {
        self.content.len()
    }

    pub fn move_brick(&mut self, from_index: usize, to_index: usize) -> Result<(), String> {
        if from_index >= self.content.len() || to_index >= self.content.len() {
            return Err("Index out of range".to_string());
        }
        let brick = self.content.remove(from_index);
        self.content.insert(to_index, brick);
        Ok(())
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.content.len() {
            self.content.remove(index);
        }
    }

    pub fn to_json(&self) -> String {
        let content = self
            .content
            .iter()
            .map(|brick| serde_json::to_string_pretty(brick).unwrap_or_else(|_| "{}".to_string()))
            .collect::<Vec<_>>();
        serde_json::to_string_pretty(&serde_json::json!({
            "name": self.name,
            "content": content,
        }))
        .unwrap_or_else(|_| "{}".to_string())
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let parsed: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {}", e))?;
        let name = parsed
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Missing 'name' field".to_string())?
            .to_string();
        let content_array = parsed
            .get("content")
            .and_then(|v| v.as_array())
            .ok_or_else(|| "Missing 'content' field".to_string())?;

        let content: Result<Vec<Box<dyn BrickRenderable>>, String> = content_array
            .iter()
            .map(Value::as_str)
            .map(|item| {
                let item = item.ok_or_else(|| "Content item is not a string".to_string())?;
                serde_json::from_str::<Box<dyn BrickRenderable>>(item)
                    .map_err(|error| error.to_string())
            })
            .collect();

        Ok(Tutorial {
            name,
            content: content?,
        })
    }
}

impl Pixmap for Tutorial {
    fn to_pixmap(&self, target_width: u32) -> Result<tiny_skia::Pixmap, String> {
        if self.content.is_empty() {
            return tiny_skia::Pixmap::new(target_width.max(1), 1)
                .ok_or_else(|| "Failed to create empty tutorial pixmap".to_string());
        }

        let pixmaps: Vec<tiny_skia::Pixmap> = self
            .content
            .iter()
            .map(|brick| brick.to_pixmap(target_width))
            .collect::<Result<Vec<_>, _>>()?;

        let total_height_sum: u32 = pixmaps.iter().map(|pm| pm.height()).sum();
        let overlap = (target_width as f32 * 0.02).ceil() as u32;
        let total_height =
            total_height_sum.saturating_sub(overlap * (pixmaps.len().saturating_sub(1) as u32));

        let mut canvas = tiny_skia::Pixmap::new(target_width, total_height.max(1))
            .ok_or_else(|| format!("Failed to create canvas {}x{}", target_width, total_height))?;

        let mut y_offset: u32 = 0;
        for pm in &pixmaps {
            let pm_data = pm.data();
            let pw = pm.width() as usize;
            let ph = pm.height() as usize;
            let canvas_w = canvas.width() as usize;

            for row in 0..ph {
                let dst_row = y_offset as usize + row;
                if dst_row >= canvas.height() as usize {
                    break;
                }

                let src_row_start = row * pw * 4;
                let dst_row_start = dst_row * canvas_w * 4;
                let canvas_data = canvas.data_mut();

                for col in 0..pw {
                    let source = src_row_start + col * 4;
                    let destination = dst_row_start + col * 4;
                    if source + 3 >= pm_data.len() || destination + 3 >= canvas_data.len() {
                        break;
                    }

                    let src_a = pm_data[source + 3] as u32;
                    let inv_a = 255 - src_a;
                    canvas_data[destination] = (pm_data[source] as u32
                        + canvas_data[destination] as u32 * inv_a / 255)
                        as u8;
                    canvas_data[destination + 1] = (pm_data[source + 1] as u32
                        + canvas_data[destination + 1] as u32 * inv_a / 255)
                        as u8;
                    canvas_data[destination + 2] = (pm_data[source + 2] as u32
                        + canvas_data[destination + 2] as u32 * inv_a / 255)
                        as u8;
                    canvas_data[destination + 3] =
                        (src_a + canvas_data[destination + 3] as u32 * inv_a / 255) as u8;
                }
            }

            y_offset += pm.height().saturating_sub(overlap);
        }

        Ok(canvas)
    }
}
