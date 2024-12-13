use std::error::Error;

pub(crate) mod boxdraw;
pub(crate) mod direction;
pub(crate) mod iter;
pub(crate) mod pos;

#[allow(dead_code)]
pub(crate) fn not_yet_implemented() -> Result<String, Box<dyn Error>> {
    Ok("Not yet implemented!".to_string())
}
