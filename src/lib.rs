use typst_wasm_protocol::wasm_export;

mod number_to_text;
mod lunar;

#[wasm_export]
pub fn number_to_text(input: &[u8]) -> Result<Vec<u8>, String> {
    number_to_text::number_to_text(input).map_err(|e| e.to_string())
}

#[wasm_export]
pub fn solar_to_lunisolar(input: &[u8]) -> Result<Vec<u8>, String> {
    lunar::solar_to_lunisolar(input).map_err(|e| e.to_string())
}
