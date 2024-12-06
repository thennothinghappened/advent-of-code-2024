use std::error::Error;

#[allow(dead_code)]
pub(crate) fn not_yet_implemented() -> Result<String, Box<dyn Error>> {
    Ok("Not yet implemented!".to_string())
}
