use cliclack::{ Theme, ThemeState };
use console::{ Style };

pub struct CustomTheme;

impl Theme for CustomTheme {
    fn bar_color(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Active => Style::new().yellow(),
            ThemeState::Cancel => Style::new().red(),
            ThemeState::Submit => Style::new().green(),
            ThemeState::Error(_) => Style::new().yellow(),
        }
    }
}
