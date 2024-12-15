use std::{error::Error, io};

pub(crate) mod boxdraw;
pub(crate) mod direction;
pub(crate) mod iter;
pub(crate) mod pos;

#[allow(dead_code)]
pub(crate) fn not_yet_implemented() -> Result<String, Box<dyn Error>> {
    Ok("Not yet implemented!".to_string())
}

#[allow(dead_code)]
pub(crate) fn wait_for_user() {
    let _ = io::stdin().read_line(&mut String::new());
}
