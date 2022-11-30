use anyhow::Result;
use handlebars::Handlebars;

/// Tide state.
#[derive(Clone)]
pub struct State {
    pub reg: Handlebars<'static>,
}

impl State {
    /// Create a new [`State`] with the default templates.
    pub fn with_default_templates() -> Result<State> {
        let mut reg = Handlebars::new();

        #[cfg(debug_assertions)]
        reg.set_dev_mode(true);

        reg.register_template_file("index", "src/www/index.hbs")?;
        reg.register_template_file("search", "src/www/search.hbs")?;

        Ok(State { reg })
    }
}
