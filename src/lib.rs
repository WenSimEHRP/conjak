use typst_wasm_protocol::wasm_export;

mod models;

#[wasm_export]
pub fn sep_by_ten_thousands(input: &[u8]) -> Result<Vec<u8>, String> {
    models::sep_10000::inner_sep_by_ten_thousands(input).map_err(|e| e.to_string())
}

#[wasm_export]
pub fn solar_to_lunisolar(input: &[u8]) -> Result<Vec<u8>, String> {
    models::lunar::solar_to_lunisolar(input).map_err(|e| e.to_string())
}
