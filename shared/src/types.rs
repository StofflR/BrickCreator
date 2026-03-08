use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BrickType {
    #[serde(rename = "H0Collapsed")]
    H0Collapsed,
    #[serde(rename = "H1Base")]
    H1Base,
    #[serde(rename = "H2Base")]
    H2Base,
    #[serde(rename = "H3Base")]
    H3Base,
    #[serde(rename = "H1Control")]
    H1Control,
    #[serde(rename = "H2Control")]
    H2Control,
}
