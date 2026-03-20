use std::ops::Deref;

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::brick::h0::BrickH0;
use crate::brick::h1_base::BrickH1Base;
use crate::brick::h1_control::BrickH1Control;
use crate::brick::h2_base::BrickH2Base;
use crate::brick::h2_control::BrickH2Control;
use crate::brick::h3_base::BrickH3Base;
use crate::common::{Brick, BrickRenderable};
use crate::types::BrickType;

pub fn serialize_brick<S, B>(
    brick: &B,
    serializer: S,
    brick_type: BrickType,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    B: Brick + ?Sized,
{
    let mut serialized = serializer.serialize_struct("Brick", 5)?;
    serialized.serialize_field("brick_type", &brick_type)?;
    serialized.serialize_field("content", &brick.content)?;
    let color_scheme = (
        brick.color_scheme.color.clone(),
        brick.color_scheme.shade.clone(),
        brick.color_scheme.border.clone(),
        brick.color_scheme.text.clone(),
    );
    serialized.serialize_field("color_scheme", &color_scheme)?;
    serialized.serialize_field("offset", &brick.offset)?;
    serialized.serialize_field("scale", &(brick.scale.x, brick.scale.y))?;
    serialized.end()
}

impl Serialize for BrickH0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_brick(self, serializer, BrickType::H0Collapsed)
    }
}

impl Serialize for BrickH1Base {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_brick(self, serializer, BrickType::H1Base)
    }
}

impl Serialize for BrickH1Control {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_brick(self, serializer, BrickType::H1Control)
    }
}

impl Serialize for BrickH2Base {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_brick(self, serializer, BrickType::H2Base)
    }
}

impl Serialize for BrickH2Control {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_brick(self, serializer, BrickType::H2Control)
    }
}

impl Serialize for BrickH3Base {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_brick(self, serializer, BrickType::H3Base)
    }
}

impl Serialize for Box<&dyn BrickRenderable> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let brick = Box::deref(self);
        serialize_brick(*brick, serializer, self.get_type())
    }
}

impl Serialize for Box<dyn BrickRenderable> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let brick = Box::deref(self);
        serialize_brick(brick, serializer, self.get_type())
    }
}
