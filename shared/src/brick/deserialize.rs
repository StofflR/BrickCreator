use rusttype::Scale;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::brick::base::BaseBrick;
use crate::brick::h0::BrickH0;
use crate::brick::h1_base::BrickH1Base;
use crate::brick::h1_control::BrickH1Control;
use crate::brick::h2_base::BrickH2Base;
use crate::brick::h2_control::BrickH2Control;
use crate::brick::h3_base::BrickH3Base;
use crate::color::{ALL_COLOR_SCHEMES, ColorScheme};
use crate::common::BrickRenderable;
use crate::types::BrickType;

#[derive(Deserialize)]
struct BrickData {
    brick_type: BrickType,
    content: String,
    color_scheme: (String, String, String, String), // (color, shade, border, text)
    offset: (f32, f32),
    scale: (f32, f32),
}

pub fn deserialize_brick_type<'de, D>(deserializer: D) -> Result<BrickType, D::Error>
where
    D: Deserializer<'de>,
{
    let data = BrickData::deserialize(deserializer)?;
    Ok(data.brick_type)
}

fn deserialize_base<'de, D>(
    deserializer: D,
    expected_type: BrickType,
) -> Result<BaseBrick, D::Error>
where
    D: Deserializer<'de>,
{
    let data = BrickData::deserialize(deserializer)?;

    if data.brick_type != expected_type {
        return Err(D::Error::custom(format!(
            "Invalid brick_type '{:?}', expected '{:?}'",
            data.brick_type, expected_type
        )));
    }

    Ok(BaseBrick {
        content: data.content,
        color_scheme: ColorScheme {
            name: get_name_from_color_scheme(&data.color_scheme)
                .unwrap_or_else(|| "Custom".to_string()),
            color: data.color_scheme.0,
            shade: data.color_scheme.1,
            border: data.color_scheme.2,
            text: data.color_scheme.3,
        }, // Convert tuple to ColorScheme
        offset: data.offset,
        scale: Scale {
            x: data.scale.0,
            y: data.scale.1,
        },
    })
}

impl<'de> Deserialize<'de> for BrickH0 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            base: deserialize_base(deserializer, BrickType::H0Collapsed)?,
        })
    }
}

impl<'de> Deserialize<'de> for BrickH1Base {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            base: deserialize_base(deserializer, BrickType::H1Base)?,
        })
    }
}

impl<'de> Deserialize<'de> for BrickH1Control {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            base: deserialize_base(deserializer, BrickType::H1Control)?,
        })
    }
}

impl<'de> Deserialize<'de> for BrickH2Base {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            base: deserialize_base(deserializer, BrickType::H2Base)?,
        })
    }
}

impl<'de> Deserialize<'de> for BrickH2Control {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            base: deserialize_base(deserializer, BrickType::H2Control)?,
        })
    }
}

impl<'de> Deserialize<'de> for BrickH3Base {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            base: deserialize_base(deserializer, BrickType::H3Base)?,
        })
    }
}

pub fn get_name_from_color_scheme(
    color_scheme: &(String, String, String, String),
) -> Option<String> {
    ALL_COLOR_SCHEMES.iter().find_map(|scheme| {
        if scheme.color == color_scheme.0
            && scheme.shade == color_scheme.1
            && scheme.border == color_scheme.2
            && scheme.text == color_scheme.3
        {
            Some(scheme.name.clone())
        } else {
            None
        }
    })
}

impl<'de> Deserialize<'de> for Box<dyn BrickRenderable> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = BrickData::deserialize(deserializer)?;

        let color_scheme =
            get_name_from_color_scheme(&data.color_scheme).unwrap_or("Custom".to_string());
        let base = BaseBrick {
            content: data.content,
            color_scheme: ColorScheme {
                name: color_scheme,
                color: data.color_scheme.0,
                shade: data.color_scheme.1,
                border: data.color_scheme.2,
                text: data.color_scheme.3,
            },
            offset: data.offset,
            scale: Scale {
                x: data.scale.0,
                y: data.scale.1,
            },
        };

        match data.brick_type {
            BrickType::H0Collapsed => Ok(Box::new(BrickH0 { base })),
            BrickType::H1Base => Ok(Box::new(BrickH1Base { base })),
            BrickType::H2Base => Ok(Box::new(BrickH2Base { base })),
            BrickType::H3Base => Ok(Box::new(BrickH3Base { base })),
            BrickType::H1Control => Ok(Box::new(BrickH1Control { base })),
            BrickType::H2Control => Ok(Box::new(BrickH2Control { base })),
        }
    }
}
