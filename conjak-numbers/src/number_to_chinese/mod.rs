mod functions;
mod naive;
mod number_to_chinese_error;
mod traits;

mod functions_test;

use alloc::string::String;

use functions::*;
pub use naive::*;
pub use number_to_chinese_error::*;
pub use traits::*;

use crate::{ChineseCountMethod, ChineseSign, NumberScript};

// TODO unsigned integer

/// 將 `u8` 整數轉成中文數字。
#[inline]
pub fn from_u8_to_chinese(
    number_script: NumberScript,
    value: u8,
) -> String {
    from_u128_to_chinese_low(number_script, value as u128).unwrap()
}

/// 將 `u16` 整數轉成中文數字。
#[inline]
pub fn from_u16_to_chinese(
    number_script: NumberScript,
    value: u16,
) -> String {
    from_u128_to_chinese_low(number_script, value as u128).unwrap()
}

/// 將 `u32` 整數轉成中文數字，使用 **「下數」**。
#[inline]
pub fn from_u32_to_chinese_low(
    number_script: NumberScript,
    value: u32,
) -> String {
    from_u128_to_chinese_low(number_script, value as u128).unwrap()
}

/// 將 `u32` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_u32_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: u32,
) -> String {
    from_u128_to_chinese_ten_thousand(number_script, value as u128)
}

/// 將 `u32` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_u32_to_chinese_middle(
    number_script: NumberScript,
    value: u32,
) -> String {
    from_u128_to_chinese_middle(number_script, value as u128)
}

/// 將 `u32` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_u32_to_chinese_high(
    number_script: NumberScript,
    value: u32,
) -> String {
    from_u128_to_chinese_high(number_script, value as u128)
}

/// 將 `u64` 整數轉成中文數字，使用 **「下數」**。數值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_u64_to_chinese_low(
    number_script: NumberScript,
    value: u64,
) -> Result<String, NumberToChineseError> {
    from_u128_to_chinese_low(number_script, value as u128)
}

/// 將 `u64` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_u64_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: u64,
) -> String {
    from_u128_to_chinese_ten_thousand(number_script, value as u128)
}

/// 將 `u64` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_u64_to_chinese_middle(
    number_script: NumberScript,
    value: u64,
) -> String {
    from_u128_to_chinese_middle(number_script, value as u128)
}

/// 將 `u64` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_u64_to_chinese_high(
    number_script: NumberScript,
    value: u64,
) -> String {
    from_u128_to_chinese_high(number_script, value as u128)
}

/// 將 `u128` 整數轉成中文數字，使用 **「下數」**。數值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_u128_to_chinese_low(
    number_script: NumberScript,
    value: u128,
) -> Result<String, NumberToChineseError> {
    if value >= 1_0000_0000_0000_0000 {
        return Err(NumberToChineseError::Overflow);
    }

    Ok(unsigned_integer_to_chinese_low(number_script, false, value))
}

/// 將 `u128` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_u128_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: u128,
) -> String {
    unsigned_integer_to_chinese_ten_thousand(number_script, false, value)
}

/// 將 `u128` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_u128_to_chinese_middle(
    number_script: NumberScript,
    value: u128,
) -> String {
    unsigned_integer_to_chinese_middle(number_script, false, value)
}

/// 將 `u128` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_u128_to_chinese_high(
    number_script: NumberScript,
    value: u128,
) -> String {
    unsigned_integer_to_chinese_high(number_script, false, value)
}

/// 將 `usize` 整數轉成中文數字，使用 **「下數」**。數值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_usize_to_chinese_low(
    number_script: NumberScript,
    value: usize,
) -> Result<String, NumberToChineseError> {
    from_u128_to_chinese_low(number_script, value as u128)
}

/// 將 `usize` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_usize_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: usize,
) -> String {
    from_u128_to_chinese_ten_thousand(number_script, value as u128)
}

/// 將 `usize` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_usize_to_chinese_middle(
    number_script: NumberScript,
    value: usize,
) -> String {
    from_u128_to_chinese_middle(number_script, value as u128)
}

/// 將 `usize` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_usize_to_chinese_high(
    number_script: NumberScript,
    value: usize,
) -> String {
    from_u128_to_chinese_high(number_script, value as u128)
}

// TODO signed integer

/// 將 `i8` 整數轉成中文數字。
#[inline]
pub fn from_i8_to_chinese(
    number_script: NumberScript,
    value: i8,
) -> String {
    from_i128_to_chinese_low(number_script, value as i128).unwrap()
}

/// 將 `i16` 整數轉成中文數字。
#[inline]
pub fn from_i16_to_chinese(
    number_script: NumberScript,
    value: i16,
) -> String {
    from_i128_to_chinese_low(number_script, value as i128).unwrap()
}

/// 將 `i32` 整數轉成中文數字，使用 **「下數」**。
#[inline]
pub fn from_i32_to_chinese_low(
    number_script: NumberScript,
    value: i32,
) -> String {
    from_i128_to_chinese_low(number_script, value as i128).unwrap()
}

/// 將 `i32` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_i32_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: i32,
) -> String {
    from_i128_to_chinese_ten_thousand(number_script, value as i128)
}

/// 將 `i32` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_i32_to_chinese_middle(
    number_script: NumberScript,
    value: i32,
) -> String {
    from_i128_to_chinese_middle(number_script, value as i128)
}

/// 將 `i32` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_i32_to_chinese_high(
    number_script: NumberScript,
    value: i32,
) -> String {
    from_i128_to_chinese_high(number_script, value as i128)
}

/// 將 `i64` 整數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_i64_to_chinese_low(
    number_script: NumberScript,
    value: i64,
) -> Result<String, NumberToChineseError> {
    from_i128_to_chinese_low(number_script, value as i128)
}

/// 將 `i64` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_i64_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: i64,
) -> String {
    from_i128_to_chinese_ten_thousand(number_script, value as i128)
}

/// 將 `i64` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_i64_to_chinese_middle(
    number_script: NumberScript,
    value: i64,
) -> String {
    from_i128_to_chinese_middle(number_script, value as i128)
}

/// 將 `i64` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_i64_to_chinese_high(
    number_script: NumberScript,
    value: i64,
) -> String {
    from_i128_to_chinese_high(number_script, value as i128)
}

/// 將 `i128` 整數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_i128_to_chinese_low(
    number_script: NumberScript,
    value: i128,
) -> Result<String, NumberToChineseError> {
    if value < 0 {
        let mut s =
            from_u128_to_chinese_low(number_script, -(value + 1) as u128 + 1)
                .map_err(|err| match err {
                    NumberToChineseError::Overflow => NumberToChineseError::Underflow,
                    _ => err,
                })?;

        s.insert_str(0, ChineseSign::負.to_str(number_script));

        Ok(s)
    } else {
        from_u128_to_chinese_low(number_script, value as u128)
    }
}

/// 將 `i128` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_i128_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: i128,
) -> String {
    if value < 0 {
        let mut s = from_u128_to_chinese_ten_thousand(
            number_script,
            -(value + 1) as u128 + 1,
        );

        s.insert_str(0, ChineseSign::負.to_str(number_script));

        s
    } else {
        from_u128_to_chinese_ten_thousand(number_script, value as u128)
    }
}

/// 將 `i128` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_i128_to_chinese_middle(
    number_script: NumberScript,
    value: i128,
) -> String {
    if value < 0 {
        let mut s =
            from_u128_to_chinese_middle(number_script, -(value + 1) as u128 + 1);

        s.insert_str(0, ChineseSign::負.to_str(number_script));

        s
    } else {
        from_u128_to_chinese_middle(number_script, value as u128)
    }
}

/// 將 `i128` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_i128_to_chinese_high(
    number_script: NumberScript,
    value: i128,
) -> String {
    if value < 0 {
        let mut s =
            from_u128_to_chinese_high(number_script, -(value + 1) as u128 + 1);

        s.insert_str(0, ChineseSign::負.to_str(number_script));

        s
    } else {
        from_u128_to_chinese_high(number_script, value as u128)
    }
}

/// 將 `isize` 整數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_isize_to_chinese_low(
    number_script: NumberScript,
    value: isize,
) -> Result<String, NumberToChineseError> {
    from_i128_to_chinese_low(number_script, value as i128)
}

/// 將 `isize` 整數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_isize_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: isize,
) -> String {
    from_i128_to_chinese_ten_thousand(number_script, value as i128)
}

/// 將 `isize` 整數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_isize_to_chinese_middle(
    number_script: NumberScript,
    value: isize,
) -> String {
    from_i128_to_chinese_middle(number_script, value as i128)
}

/// 將 `isize` 整數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_isize_to_chinese_high(
    number_script: NumberScript,
    value: isize,
) -> String {
    from_i128_to_chinese_high(number_script, value as i128)
}

// TODO float

/// 將 `f32` 浮點數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_f32_to_chinese_low(
    number_script: NumberScript,
    value: f32,
) -> Result<String, NumberToChineseError> {
    from_f64_to_chinese_low(number_script, value as f64)
}

/// 將 `f32` 浮點數轉成中文數字，使用 **「萬進」**。
#[inline]
pub fn from_f32_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: f32,
) -> String {
    from_f64_to_chinese_ten_thousand(number_script, value as f64).unwrap()
}

/// 將 `f32` 浮點數轉成中文數字，使用 **「中數」**。
#[inline]
pub fn from_f32_to_chinese_middle(
    number_script: NumberScript,
    value: f32,
) -> String {
    from_f64_to_chinese_middle(number_script, value as f64).unwrap()
}

/// 將 `f32` 浮點數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_f32_to_chinese_high(
    number_script: NumberScript,
    value: f32,
) -> String {
    from_f64_to_chinese_high(number_script, value as f64)
}

#[inline]
fn from_f64_to_chinese(
    number_script: NumberScript,
    method: ChineseCountMethod,
    value: f64,
) -> String {
    if value < 0.0 {
        let mut s = positive_float_to_chinese(number_script, method, -value);

        s.insert_str(0, ChineseSign::負.to_str(number_script));

        s
    } else {
        positive_float_to_chinese(number_script, method, value)
    }
}

/// 將 `f64` 浮點數轉成中文數字，使用 **「下數」**。數值的絕對值不能大於或等於 `1_0000_0000_0000_0000`。
#[inline]
pub fn from_f64_to_chinese_low(
    number_script: NumberScript,
    value: f64,
) -> Result<String, NumberToChineseError> {
    if value >= 1_0000_0000_0000_0000f64 {
        return Err(NumberToChineseError::Overflow);
    } else if value <= -1_0000_0000_0000_0000f64 {
        return Err(NumberToChineseError::Underflow);
    }

    Ok(from_f64_to_chinese(number_script, ChineseCountMethod::Low, value))
}

/// 將 `f64` 浮點數轉成中文數字，使用 **「萬進」**。數值的絕對值不能大於或等於 `1e52`。
#[inline]
pub fn from_f64_to_chinese_ten_thousand(
    number_script: NumberScript,
    value: f64,
) -> Result<String, NumberToChineseError> {
    if value >= 1e52 {
        return Err(NumberToChineseError::Overflow);
    } else if value <= -1e52 {
        return Err(NumberToChineseError::Underflow);
    }

    Ok(from_f64_to_chinese(number_script, ChineseCountMethod::TenThousand, value))
}

/// 將 `f64` 浮點數轉成中文數字，使用 **「中數」**。數值的絕對值不能大於或等於 `1e96`。
#[inline]
pub fn from_f64_to_chinese_middle(
    number_script: NumberScript,
    value: f64,
) -> Result<String, NumberToChineseError> {
    if value >= 1e96 {
        return Err(NumberToChineseError::Overflow);
    } else if value <= -1e96 {
        return Err(NumberToChineseError::Underflow);
    }

    Ok(from_f64_to_chinese(number_script, ChineseCountMethod::Middle, value))
}

/// 將 `f64` 浮點數轉成中文數字，使用 **「上數」**。
#[inline]
pub fn from_f64_to_chinese_high(
    number_script: NumberScript,
    value: f64,
) -> String {
    from_f64_to_chinese(number_script, ChineseCountMethod::High, value)
}
