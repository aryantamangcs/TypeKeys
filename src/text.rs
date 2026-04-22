use crate::config::Config;
use color_eyre::Result;

const TEXT: &str = "Hello How are you?
Where do you leave?
How old are you?
";

pub fn get_text(_config: &Config) -> Result<String> {
    Ok(TEXT.to_string())
}
