#let plg = plugin("cjk_num_format.wasm")

/// Generate a string with the given value formatted with thousands separators.
///
/// - value (int): The number to format.
/// - separators (array): An array of strings to use as thousands separators.
/// - lang (str): The language code to use for formatting (e.g., "en", "zh").
/// - region (str): The region code to use for formatting (e.g., "US", "CN").
/// -> str
#let sep-by-ten-thousands(
  value,
  separators: none,
  lang: none,
  region: none,
) = context {
  str(
    plg.sep_by_ten_thousands(
      cbor.encode((
        lang: if lang == none { text.lang } else { lang },
        region: if region == none { text.region } else { region },
        value: value,
        separators: separators,
      )),
    ),
  )
}
