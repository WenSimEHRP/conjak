use anyhow::{Context, Result, bail};
use ciborium::from_reader;
use serde::Deserialize;
use typst_wasm_protocol::wasm_export;

#[derive(Deserialize)]
struct ThousandSepInput {
    value: u64,
    lang: Option<String>,
    region: Option<String>,
    separators: Option<Vec<String>>,
}

const THOUSAND_SEP_JA: &[&str] = &[
    "万", "億", "兆", "京", "垓", "秭", "穰", "溝", "澗", "正", "載", "極",
];
const THOUSAND_SEP_KO: &[&str] = &[
    "만", "억", "조", "경", "해", "자", "양", "구", "간", "정", "재", "극",
];
const THOUSAND_SEP_ZH_SIM: &[&str] = &[
    "万", "亿", "兆", "京", "垓", "秭", "穰", "沟", "涧", "正", "载", "极",
];
const THOUSAND_SEP_ZH_TRA: &[&str] = &[
    "萬", "億", "兆", "京", "垓", "秭", "穰", "溝", "澗", "正", "載", "極",
];

fn get_thousand_separators(lang: &str, region: &str) -> Result<&'static [&'static str]> {
    match (lang, region) {
        ("ja", _) => Ok(THOUSAND_SEP_JA),
        ("ko", _) => Ok(THOUSAND_SEP_KO),
        ("zh", "CN") => Ok(THOUSAND_SEP_ZH_SIM),
        ("zh", "HK") => Ok(THOUSAND_SEP_ZH_TRA),
        ("zh", "MO") => Ok(THOUSAND_SEP_ZH_TRA),
        ("zh", "SG") => Ok(THOUSAND_SEP_ZH_SIM),
        ("zh", "TW") => Ok(THOUSAND_SEP_ZH_TRA),
        ("zh", _) => Ok(THOUSAND_SEP_ZH_SIM),
        _ => bail!("Unsupported language/region: {}/{}", lang, region),
    }
}

fn separate(lang: &str, region: &str, value: u64) -> Result<String> {
    let separators =
        get_thousand_separators(lang, region).context("Failed to get thousand separators")?;

    let mut result = String::new();
    let mut step = 0;
    let mut value = value;

    while value > 0 {
        let chunk = value % 10000;
        value /= 10000;
        if chunk == 0 {
            step += 1;
            continue; // Skip leading zeros in higher chunks
        }
        if step > 0 {
            let separator = if step <= separators.len() {
                separators[step - 1]
            } else {
                ""
            };
            result = format!("{}{}{}", chunk, separator, result);
        } else {
            result = format!("{}", chunk);
        }
        step += 1;
    }

    Ok(result)
}

fn separate_with_custom(value: u64, custom_separators: &[String]) -> Result<String> {
    if value == 0 {
        return Ok("0".to_string());
    }

    let mut result = String::new();
    let mut step = 0;
    let mut value = value;

    while value > 0 {
        let chunk = value % 10000;
        value /= 10000;

        if step > 0 {
            let separator = if step <= custom_separators.len() {
                &custom_separators[step - 1]
            } else {
                ""
            };
            result = format!("{}{}{}", chunk, separator, result);
        } else {
            result = format!("{}", chunk);
        }
        step += 1;
    }

    Ok(result)
}

#[wasm_export]
pub fn sep_by_thousands(input: &[u8]) -> std::result::Result<Vec<u8>, String> {
    inner_sep_by_thousands(input).map_err(|e| e.to_string())
}

fn inner_sep_by_thousands(input: &[u8]) -> Result<Vec<u8>> {
    let input: ThousandSepInput = from_reader(input).context("Failed to deserialize input")?;

    let separated: String = match input.separators {
        Some(custom_separators) => separate_with_custom(input.value, &custom_separators)
            .context("Failed to format number with custom separators")?,
        None => match (input.lang, input.region) {
            (Some(lang), Some(region)) => separate(&lang, &region, input.value)
                .context("Failed to format number with default separators")?,
            (Some(lang), None) => separate(&lang, "", input.value)
                .context("Failed to format number with default separators")?,
            (None, Some(_)) => {
                bail!("You must provide a language if you provide a region");
            }
            (None, None) => {
                bail!("You must provide either lang/region pair or custom separators");
            }
        },
    };

    Ok(separated.into_bytes())
}
