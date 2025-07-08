#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum NumberScript {
    SimplifiedChinese(ScriptStyle),
    TraditionalChinese(ScriptStyle),
    Japanese(ScriptStyle),
    Korean(ScriptStyle),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ScriptStyle {
    Upper,
    Lower { circle_as_zero: bool },
}
