use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use num_bigint::BigUint;
#[allow(unused_imports)]
use num_traits::float::FloatCore;
use num_traits::{FromPrimitive, ToPrimitive, Zero};

use crate::{
    chinese_characters::{ChineseNumber, ChinesePoint, ChineseSign},
    NumberScript,
};

fn unsigned_integer_to_chinese(
    number_script: NumberScript,
    mut value: u128,
) -> String {
    if value == 0 {
        return ChineseNumber::零.to_str(number_script).to_string();
    }

    let mut numbers: Vec<ChineseNumber> = Vec::with_capacity(1);

    while value > 0 {
        let n = (value % 10) as u8;
        value /= 10;

        numbers.push(unsafe { ChineseNumber::from_ordinal_unsafe(n) });
    }

    numbers.into_iter().rev().map(|cn| cn.to_str(number_script)).collect()
}

fn big_unsigned_integer_to_chinese(
    number_script: NumberScript,
    mut value: BigUint,
) -> String {
    let big_0 = BigUint::zero();
    let big_10 = BigUint::from(10u8);

    if value == big_0 {
        return ChineseNumber::零.to_str(number_script).to_string();
    }

    let mut numbers: Vec<ChineseNumber> = Vec::with_capacity(1);

    while value > big_0 {
        let n = (value.clone() % &big_10).to_u8().unwrap();
        value /= &big_10;

        numbers.push(unsafe { ChineseNumber::from_ordinal_unsafe(n) });
    }

    numbers.into_iter().rev().map(|cn| cn.to_str(number_script)).collect()
}

fn positive_float_to_chinese(
    number_script: NumberScript,
    value: f64,
) -> String {
    let (integer, fraction) = {
        let integer = BigUint::from_f64(value.trunc()).unwrap();
        let fraction = ((value.fract() * 100.0).round() % 100f64) as u8;

        (integer, fraction)
    };

    let mut s = big_unsigned_integer_to_chinese(number_script, integer);

    if fraction > 0 {
        s.push_str(ChinesePoint::to_str(number_script));

        s.push_str(
            unsafe { ChineseNumber::from_ordinal_unsafe(fraction / 10) }
                .to_str(number_script),
        );

        let d = fraction % 10;

        if d > 0 {
            s.push_str(
                unsafe { ChineseNumber::from_ordinal_unsafe(d) }
                    .to_str(number_script),
            );
        }
    }

    s
}

/// 將 `u8` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u8_to_chinese_naive(
    number_script: NumberScript,
    value: u8,
) -> String {
    from_u128_to_chinese_naive(number_script, value as u128)
}

/// 將 `u16` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u16_to_chinese_naive(
    number_script: NumberScript,
    value: u16,
) -> String {
    from_u128_to_chinese_naive(number_script, value as u128)
}

/// 將 `u32` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u32_to_chinese_naive(
    number_script: NumberScript,
    value: u32,
) -> String {
    from_u128_to_chinese_naive(number_script, value as u128)
}

/// 將 `u32` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u64_to_chinese_naive(
    number_script: NumberScript,
    value: u64,
) -> String {
    from_u128_to_chinese_naive(number_script, value as u128)
}

/// 將 `u128` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_u128_to_chinese_naive(
    number_script: NumberScript,
    value: u128,
) -> String {
    unsigned_integer_to_chinese(number_script, value)
}

/// 將 `usize` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_usize_to_chinese_naive(
    number_script: NumberScript,
    value: usize,
) -> String {
    unsigned_integer_to_chinese(number_script, value as u128)
}

/// 將 `i8` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i8_to_chinese_naive(
    number_script: NumberScript,
    value: i8,
) -> String {
    from_i128_to_chinese_naive(number_script, value as i128)
}

/// 將 `i16` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i16_to_chinese_naive(
    number_script: NumberScript,
    value: i16,
) -> String {
    from_i128_to_chinese_naive(number_script, value as i128)
}

/// 將 `i32` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i32_to_chinese_naive(
    number_script: NumberScript,
    value: i32,
) -> String {
    from_i128_to_chinese_naive(number_script, value as i128)
}

/// 將 `i64` 整數轉成中文數字。不進行單位計算。
#[inline]
pub fn from_i64_to_chinese_naive(
    number_script: NumberScript,
    value: i64,
) -> String {
    from_i128_to_chinese_naive(number_script, value as i128)
}

/// 將 `i128` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_i128_to_chinese_naive(
    number_script: NumberScript,
    value: i128,
) -> String {
    if value < 0 {
        let mut s =
            from_u128_to_chinese_naive(number_script, -(value + 1) as u128 + 1);

        s.insert_str(0, ChineseSign::負.to_str(number_script));

        s
    } else {
        from_u128_to_chinese_naive(number_script, value as u128)
    }
}

/// 將 `isize` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_isize_to_chinese_naive(
    number_script: NumberScript,
    value: isize,
) -> String {
    from_i128_to_chinese_naive(number_script, value as i128)
}

/// 將 `f32` 整數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_f32_to_chinese_naive(
    number_script: NumberScript,
    value: f32,
) -> String {
    from_f64_to_chinese_naive(number_script, value as f64)
}

#[inline]
fn from_f64_to_chinese(
    number_script: NumberScript,
    value: f64,
) -> String {
    if value < 0.0 {
        let mut s = positive_float_to_chinese(number_script, -value);

        s.insert_str(0, ChineseSign::負.to_str(number_script));

        s
    } else {
        positive_float_to_chinese(number_script, value)
    }
}

/// 將 `f64` 浮點數轉成中文數字，不進行單位計算。
#[inline]
pub fn from_f64_to_chinese_naive(
    number_script: NumberScript,
    value: f64,
) -> String {
    from_f64_to_chinese(number_script, value)
}
