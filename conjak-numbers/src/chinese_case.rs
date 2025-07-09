use serde::Deserialize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NumberScript {
    SimplifiedChinese(ScriptStyle),
    TraditionalChinese(ScriptStyle),
    Japanese(ScriptStyle),
    Korean(ScriptStyle),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptStyle {
    Upper,
    Lower { circle_as_zero: bool },
}
