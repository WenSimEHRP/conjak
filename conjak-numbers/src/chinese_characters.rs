use enum_ordinalize::Ordinalize;

use crate::{NumberScript, ScriptStyle};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[ordinalize(impl_trait = false)]
#[ordinalize(ordinal(pub(crate) fn ordinal))]
#[ordinalize(from_ordinal_unsafe(pub(crate) fn from_ordinal_unsafe))]
#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum ChineseNumber {
    零,
    一,
    二,
    三,
    四,
    五,
    六,
    七,
    八,
    九,
    十,
}

impl ChineseNumber {
    #[inline]
    pub(crate) const fn to_str(self, number_script: NumberScript) -> &'static str {
        match self {
            Self::零 => match number_script {
                NumberScript::SimplifiedChinese(ss)
                | NumberScript::TraditionalChinese(ss)
                | NumberScript::Japanese(ss) => match ss {
                    ScriptStyle::Lower {
                        circle_as_zero: true,
                    } => "〇",
                    _ => "零",
                },
                NumberScript::Korean(_) => "영",
            },
            Self::一 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper)
                | NumberScript::TraditionalChinese(ScriptStyle::Upper) => "壹",
                NumberScript::Japanese(ScriptStyle::Upper) => "壱",
                NumberScript::Korean(_) => "일",
                _ => "一",
            },
            Self::二 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper)  => "贰",
                NumberScript::TraditionalChinese(ScriptStyle::Upper) => "貳",
                NumberScript::Japanese(ScriptStyle::Upper) => "弐",
                NumberScript::Korean(_) => "이",
                _ => "二",
            },
            Self::三 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper) => "叁",
                NumberScript::TraditionalChinese(ScriptStyle::Upper) => "參",
                NumberScript::Japanese(ScriptStyle::Upper) => "参",
                NumberScript::Korean(_) => "삼",
                _ => "三",
            },
            Self::四 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper) => "肆",
                NumberScript::TraditionalChinese(ScriptStyle::Upper) => "肆",
                NumberScript::Korean(_) => "사",
                _ => "四",
            },
            Self::五 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper) => "伍",
                NumberScript::TraditionalChinese(ScriptStyle::Upper) => "伍",
                NumberScript::Korean(_) => "오",
                _ => "五",
            },
            Self::六 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper) => "陆",
                NumberScript::TraditionalChinese(ScriptStyle::Upper) => "陸",
                NumberScript::Korean(_) => "육",
                _ => "六",
            },
            Self::七 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper)
                | NumberScript::TraditionalChinese(ScriptStyle::Upper) => "柒",
                NumberScript::Korean(_) => "칠",
                _ => "七",
            },
            Self::八 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper)
                | NumberScript::TraditionalChinese(ScriptStyle::Upper) => "捌",
                NumberScript::Korean(_) => "팔",
                _ => "八",
            },
            Self::九 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper)
                | NumberScript::TraditionalChinese(ScriptStyle::Upper) => "玖",
                NumberScript::Korean(_) => "구",
                _ => "九",
            },
            Self::十 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper)
                | NumberScript::TraditionalChinese(ScriptStyle::Upper)
                | NumberScript::Japanese(ScriptStyle::Upper) => "拾",
                NumberScript::Korean(_) => "십",
                _ => "十",
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Ordinalize)]
#[ordinalize(impl_trait = false)]
#[ordinalize(ordinal(pub(crate) fn ordinal))]
#[ordinalize(from_ordinal_unsafe(pub(crate) fn from_ordinal_unsafe))]
#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum ChineseExponent {
    分,
    角,
    個,
    十,
    百,
    千,
    萬,
    億,
    兆,
    京,
    垓,
    秭,
    穰,
    溝,
    澗,
    正,
    載,
    極,
}

impl ChineseExponent {
    #[inline]
    pub(crate) const fn to_str(self, number_script: NumberScript) -> &'static str {
        match self {
            Self::分 => match number_script {
                NumberScript::SimplifiedChinese(_) | NumberScript::TraditionalChinese(_) => "分",
                NumberScript::Japanese(_) => "厘",
                NumberScript::Korean(_) => "분",
            },
            Self::角 => match number_script {
                NumberScript::SimplifiedChinese(_) | NumberScript::TraditionalChinese(_) => "角",
                NumberScript::Japanese(_) => "钱",
                NumberScript::Korean(_) => "각",
            },
            Self::個 => match number_script {
                NumberScript::SimplifiedChinese(_) => "个",
                NumberScript::TraditionalChinese(_) | NumberScript::Japanese(_) => "個",
                NumberScript::Korean(_) => "개",
            },
            Self::十 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper)
                | NumberScript::TraditionalChinese(ScriptStyle::Upper) => "拾",
                NumberScript::Korean(_) => "십",
                _ => "十",
            },
            Self::百 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper)
                | NumberScript::TraditionalChinese(ScriptStyle::Upper) => "佰",
                NumberScript::Korean(_) => "백",
                _ => "百",
            },
            Self::千 => match number_script {
                NumberScript::SimplifiedChinese(ScriptStyle::Upper)
                | NumberScript::TraditionalChinese(ScriptStyle::Upper) => "仟",
                NumberScript::Korean(_) => "천",
                _ => "千",
            },
            Self::萬 => match number_script {
                NumberScript::TraditionalChinese(_) => "萬",
                NumberScript::Korean(_) => "만",
                _ => "万",
            },
            Self::億 => match number_script {
                NumberScript::SimplifiedChinese(_) => "亿",
                NumberScript::Korean(_) => "억",
                _ => "億",
            },
            Self::兆 => match number_script {
                NumberScript::Korean(_) => "조",
                _ => "兆",
            },
            Self::京 => match number_script {
                NumberScript::Korean(_) => "경",
                _ => "京",
            },
            Self::垓 => match number_script {
                NumberScript::Korean(_) => "해",
                _ => "垓",
            },
            Self::秭 => match number_script {
                NumberScript::Korean(_) => "자",
                _ => "秭",
            },
            Self::穰 => match number_script {
                NumberScript::Korean(_) => "양",
                _ => "穰",
            },
            Self::溝 => match number_script {
                NumberScript::SimplifiedChinese(_) => "沟",
                NumberScript::Korean(_) => "구",
                _ => "溝",
            },
            Self::澗 => match number_script {
                NumberScript::SimplifiedChinese(_) => "涧",
                NumberScript::Korean(_) => "간",
                _ => "澗",
            },
            Self::正 => match number_script {
                NumberScript::Korean(_) => "정",
                _ => "正",
            },
            Self::載 => match number_script {
                NumberScript::SimplifiedChinese(_) => "载",
                NumberScript::Korean(_) => "재",
                _ => "載",
            },
            Self::極 => match number_script {
                NumberScript::SimplifiedChinese(_) => "极",
                NumberScript::Korean(_) => "극",
                _ => "極",
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum ChineseSign {
    正,
    負,
}

impl ChineseSign {
    #[inline]
    pub(crate) const fn to_str(self, number_script: NumberScript) -> &'static str {
        match self {
            Self::正 => match number_script {
                NumberScript::Korean(_) => "정",
                _ => "正",
            },
            Self::負 => match number_script {
                NumberScript::SimplifiedChinese(_) => "负",
                NumberScript::Korean(_) => "부",
                _ => "負",
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct ChinesePoint;

impl ChinesePoint {
    #[inline]
    pub(crate) const fn to_str(number_script: NumberScript) -> &'static str {
        match number_script {
            NumberScript::TraditionalChinese(_) => "點",
            NumberScript::Korean(_) => "점",
            _ => "点",
        }
    }
}
