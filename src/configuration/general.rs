use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct General {
    pub template_string: String,
}
