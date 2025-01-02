extern crate napi_derive;
mod platform;

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn start_capture() -> Result<String> {
  let message = platform::implementation::start();
  Ok(message)
}
