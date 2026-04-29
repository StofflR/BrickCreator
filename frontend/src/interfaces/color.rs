use shared::color::{ALL_COLOR_SCHEMES, ColorScheme};

pub trait ColorModel {
    fn default_colors(&self) -> &[ColorScheme];
    fn custom_colors(&self) -> &[ColorScheme];
    fn all_colors(&self) -> Vec<ColorScheme>;
    fn add_custom_color(&mut self, color: ColorScheme) -> bool;
    fn replace_custom_color(&mut self, original_name: &str, color: ColorScheme) -> bool;
    fn remove_custom_color(&mut self, name: &str) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub struct BrickColorModel {
    default_colors: Vec<ColorScheme>,
    custom_colors: Vec<ColorScheme>,
}

impl BrickColorModel {
    pub fn new() -> Self {
        let default_colors = ALL_COLOR_SCHEMES.to_vec();

        Self {
            default_colors,
            custom_colors: Vec::new(),
        }
    }

    pub fn with_custom_colors(custom_colors: Vec<ColorScheme>) -> Self {
        let mut model = Self::new();
        for color in custom_colors {
            let _ = model.add_custom_color(color);
        }
        model
    }
}

impl Default for BrickColorModel {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorModel for BrickColorModel {
    fn default_colors(&self) -> &[ColorScheme] {
        &self.default_colors
    }

    fn custom_colors(&self) -> &[ColorScheme] {
        &self.custom_colors
    }

    fn all_colors(&self) -> Vec<ColorScheme> {
        self.default_colors
            .iter()
            .chain(self.custom_colors.iter())
            .cloned()
            .collect()
    }

    fn add_custom_color(&mut self, color: ColorScheme) -> bool {
        let exists_in_defaults = self
            .default_colors
            .iter()
            .any(|entry| entry.name == color.name);

        let exists_in_custom = self
            .custom_colors
            .iter()
            .any(|entry| entry.name == color.name);

        if exists_in_defaults || exists_in_custom {
            return false;
        }

        self.custom_colors.push(color);
        true
    }

    fn replace_custom_color(&mut self, original_name: &str, color: ColorScheme) -> bool {
        let Some(index) = self
            .custom_colors
            .iter()
            .position(|entry| entry.name == original_name)
        else {
            return false;
        };

        let exists_in_defaults = self
            .default_colors
            .iter()
            .any(|entry| entry.name == color.name);
        let exists_in_other_custom = self
            .custom_colors
            .iter()
            .enumerate()
            .any(|(entry_index, entry)| entry_index != index && entry.name == color.name);

        if exists_in_defaults || exists_in_other_custom {
            return false;
        }

        self.custom_colors[index] = color;
        true
    }

    fn remove_custom_color(&mut self, name: &str) -> bool {
        let before = self.custom_colors.len();
        self.custom_colors.retain(|entry| entry.name != name);
        self.custom_colors.len() != before
    }
}
