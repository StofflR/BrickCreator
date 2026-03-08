use shared::types::BrickType;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BrickTypeEntry {
    pub kind: BrickType,
    pub label: &'static str,
}

pub const ALL_BRICK_TYPES: &[BrickTypeEntry] = &[
    BrickTypeEntry {
        kind: BrickType::H0Collapsed,
        label: "H0Collapsed",
    },
    BrickTypeEntry {
        kind: BrickType::H1Base,
        label: "H1Base",
    },
    BrickTypeEntry {
        kind: BrickType::H2Base,
        label: "H2Base",
    },
    BrickTypeEntry {
        kind: BrickType::H3Base,
        label: "H3Base",
    },
    BrickTypeEntry {
        kind: BrickType::H1Control,
        label: "H1Control",
    },
    BrickTypeEntry {
        kind: BrickType::H2Control,
        label: "H2Control",
    },
];

pub trait BrickTypeModel {
    fn all_types(&self) -> &'static [BrickTypeEntry];
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StaticBrickTypeModel;

impl BrickTypeModel for StaticBrickTypeModel {
    fn all_types(&self) -> &'static [BrickTypeEntry] {
        ALL_BRICK_TYPES
    }
}
