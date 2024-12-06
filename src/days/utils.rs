use std::error::Error;

#[allow(dead_code)]
pub(super) fn not_yet_implemented() -> Result<String, Box<dyn Error>> {
    Ok("Not yet implemented!".to_string())
}
