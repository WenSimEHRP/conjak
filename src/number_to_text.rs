use ciborium::from_reader;
use conjak_numbers::{ChineseCountMethod, NumberScript, NumberToChinese, ScriptStyle};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum NumberInput {
    Integer(i64),
    Float(f64),
}

#[derive(Deserialize)]
struct Input {
    value: NumberInput,
    number_script: Option<NumberScript>,
    count_method: Option<ChineseCountMethod>,
}

pub fn number_to_text(input: &[u8]) -> Result<Vec<u8>, String> {
    let input: Input = from_reader(input).map_err(|e| e.to_string())?;
    let number_script = input
        .number_script
        .unwrap_or(NumberScript::SimplifiedChinese(ScriptStyle::Lower {
            circle_as_zero: false,
        }));
    let count_method = input
        .count_method
        .unwrap_or(ChineseCountMethod::TenThousand);
    let s = match input.value {
        NumberInput::Integer(i) => i.to_chinese(number_script, count_method),
        NumberInput::Float(f) => f.to_chinese(number_script, count_method),
    }
    .map_err(|e| e.to_string())?;
    Ok(s.as_bytes().to_vec())
}
