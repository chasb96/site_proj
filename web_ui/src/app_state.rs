use std::sync::Arc;

use handlebars::Handlebars;
use crate::handlebars::get_handlebars;

#[derive(Clone)]
pub struct AppState {
    pub handlebars: Arc<Handlebars<'static>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { 
            handlebars: Arc::new(get_handlebars()), 
        }
    }
}