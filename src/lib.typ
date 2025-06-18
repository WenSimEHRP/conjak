#let plg = plugin("cjk_num_format.wasm")

/// Generate a string with the given value formatted with thousands separators.
/// ```example
/// #set text(lang: "ja", region: "jp")
/// #let c = cjk-num-format
/// #c.sep-by-ten-thousands(12345670000000)\
/// #c.sep-by-ten-thousands(1145141919810)\
/// #c.sep-by-ten-thousands(1145141919810000)\
/// #c.sep-by-ten-thousands(135700012255)\
/// ```
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
  // we're not using `numbering` here because `numbering('壹', xxx) doesn't support korean`
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

/// Generate a smart CJK content based on the language and region.
///
/// - zh (dict, str): String(s) for Chinese. You can optionally provide a dictionary with keys `cn`, `tw`, `hk`, etc. to specify different strings for different regions. `HK`, `MO`, and `TW` will default to the `tw` string, while other regions (e.g., Singapore) will default to the `cn` string.
/// - ja (str): String for Japanese.
/// - ko (str): String for Korean.
/// -> content
#let cjk-content(
  zh: none,
  ja: none,
  ko: none,
) = context {
  if text.lang == "zh" {
    if type(zh) == str {
      return zh
    }
    zh.at(
      lower(text.region),
      default: {
        if lower(text.region) in ("hk", "mo", "tw") {
          zh.tw
        } else {
          zh.cn
        }
      },
    )
  } else if text.lang == "ja" {
    ja
  } else if text.lang == "ko" {
    ko
  } else {
    if type(zh) == str {
      zh
    } else {
      zh.cn
    }
  }
}

#let _year-with-beginning(pfx, negative-pfx: none, year, arabic) = {
  let year-str = cjk-content(
    zh: "年",
    ja: "年",
    ko: "년",
  )
  let first-year = cjk-content(
    zh: "元年",
    ja: "元年",
    ko: "원년",
  )
  if arabic {
    // If arabic is true, return the year in Arabic numerals
    if year < 0 {
      negative-pfx + str(calc.abs(year)) + year-str
    } else if year == 0 {
      pfx + first-year
    } else {
      pfx + str(year + 1) + year-str
    }
  } else {
    if year < 0 {
      negative-pfx + numbering("一", calc.abs(year)) + year-str
    } else if year == 0 {
      pfx + first-year
    } else if year <= 100 {
      pfx + numbering("一", year + 1) + year-str
    } else {
      let year-char-map = "〇一二三四五六七八九十".clusters()
      let y = year + 1
      let ret = ""
      while y != 0 {
        ret = year-char-map.at(calc.rem(y, 10)) + ret
        y = calc.floor(y / 10)
      }
      pfx + ret + year-str
    }
  }
}

/// Format a date in CJK style, including the year, month, and day.
/// ```example
/// #cjk-num-format.cjk-date-format(
///   datetime(year: 2023, month: 10, day: 1),
/// )
/// ```
///
/// - pfx (str, content): Prefix for the date string.
/// ```example
/// #cjk-num-format.cjk-date-format(
///   pfx: "西元",
///   datetime(year: 2023, month: 10, day: 1),
/// )
/// ```
/// - negative-pfx (str, content): Prefix for negative years.
/// ```example
/// #cjk-num-format.cjk-date-format(
///   negative-pfx: "西元前",
///   datetime(year: -2023, month: 10, day: 1),
/// )
/// ```
/// - date (datetime): The date to format.
/// - established (int): The year when the era was established.
/// - arabic (auto, bool): Whether to use Arabic numerals for the year.
/// ```example
/// #cjk-num-format.cjk-date-format(
///   arabic: true,
///   datetime(year: 2023, month: 10, day: 1),
/// )
/// ```
/// -> content
#let cjk-date-format(
  pfx: none,
  negative-pfx: none,
  date,
  established: 1,
  arabic: auto,
) = context {
  let arabic = arabic
  if arabic == auto {
    if text.lang == "ko" {
      arabic = true
    } else {
      arabic = false
    }
  }
  let month-str = cjk-content(
    zh: "月",
    ja: "月",
    ko: "월",
  )
  let day-str = cjk-content(
    zh: "日",
    ja: "日",
    ko: "일",
  )
  let ret = ""
  ret += _year-with-beginning(pfx, negative-pfx: negative-pfx, date.year() - established, arabic)
  ret += if arabic { str(date.month()) } else { numbering("一", date.month()) } + month-str
  ret += if arabic { str(date.day()) } else { numbering("一", date.day()) } + day-str
  ret
}

/// Format a date in the Republic of China (ROC) calendar style.
/// ```example
/// #cjk-num-format.roc-date-format(
///   datetime(year: 1949, month: 9, day: 30),
/// )\
/// #cjk-num-format.roc-date-format(
///   datetime(year: 1912, month: 10, day: 1),
/// )\
/// #cjk-num-format.roc-date-format(
///   datetime(year: 1910, month: 10, day: 1),
/// )\
/// #cjk-num-format.roc-date-format(
///   datetime(year: 2025, month: 10, day: 1),
/// )\
/// #cjk-num-format.roc-date-format(
///   datetime(year: 2025, month: 10, day: 1),
///   arabic: true,
/// )
/// ```
///
/// - pfx (str, content): Prefix for the date string.
/// - negative-pfx (str, content): Prefix for negative years.
/// - date (datetime): The date to format.
/// - arabic (auto, bool): Whether to use Arabic numerals for the year.
/// -> content
#let roc-date-format(date, pfx: auto, negative-pfx: auto, arabic: auto) = {
  let default-pfx = cjk-content(
    zh: (cn: "民国", tw: "民國"),
    ja: "民国",
    ko: "민국",
  )
  let default-negative-pfx = cjk-content(
    zh: "民前",
    ja: "民国前",
    ko: "민국전",
  )
  cjk-date-format(
    pfx: if pfx == auto { default-pfx } else { pfx },
    negative-pfx: if negative-pfx == auto { default-negative-pfx } else { negative-pfx },
    date,
    established: 1912,
    arabic: arabic,
  )
}

/// Format a date in the Juche calendar style (North Korea's calendar).
///
/// - pfx (str, content): Prefix for the date string.
/// - negative-pfx (str, content): Prefix for negative years.
/// - date (datetime): The date to format.
/// - arabic (auto, bool): Whether to use Arabic numerals for the year.
/// -> content
#let juche-date-format(date, pfx: auto, negative-pfx: auto, arabic: auto) = {
  let default-pfx = cjk-content(
    zh: (cn: "主体", tw: "主體"),
    ja: "主体",
    ko: "주체",
  )
  let default-negative-pfx = cjk-content(
    zh: "主体前",
    ja: "主体前",
    ko: "주체전",
  )
  cjk-date-format(
    pfx: if pfx == auto { default-pfx } else { pfx },
    negative-pfx: if negative-pfx == auto { default-negative-pfx } else { negative-pfx },
    date,
    established: 1912,
    arabic: arabic,
  )
}

/// Format a date in the Japanese era style (e.g., Meiji, Taisho, Showa, Heisei, Reiwa).
/// ```example
/// #cjk-num-format.japan-date-format(
///   datetime(year: 2023, month: 10, day: 1),
/// )\
/// #cjk-num-format.japan-date-format(
///   datetime(year: 1989, month: 1, day: 6),
/// )\
/// #cjk-num-format.japan-date-format(
///   datetime(year: 1989, month: 1, day: 7),
/// )
/// ```
/// - date (datetime): The date to format.
/// - arabic (bool, auto): Whether to use Arabic numerals for the year.
/// -> content
#let japan-date-format(date, arabic: auto) = {
  let meiji = datetime(year: 1868, month: 9, day: 1)
  let taisho = datetime(year: 1912, month: 7, day: 30)
  let showa = datetime(year: 1926, month: 12, day: 25)
  let heisei = datetime(year: 1989, month: 1, day: 7)
  let reiwa = datetime(year: 2019, month: 4, day: 30)
  // match the datetime
  let meiji-pfx = cjk-content(
    zh: "明治",
    ja: "明治",
    ko: "명치",
  )
  let taisho-pfx = cjk-content(
    zh: "大正",
    ja: "大正",
    ko: "대정",
  )
  let showa-pfx = cjk-content(
    zh: "昭和",
    ja: "昭和",
    ko: "쇼와",
  )
  let heisei-pfx = cjk-content(
    zh: "平成",
    ja: "平成",
    ko: "헤이세이",
  )
  let reiwa-pfx = cjk-content(
    zh: "令和",
    ja: "令和",
    ko: "레와",
  )
  if date < meiji {
    cjk-date-format(date, arabic: arabic)
  } else if date < taisho {
    cjk-date-format(pfx: meiji-pfx, date, established: meiji.year(), arabic: arabic)
  } else if date < showa {
    cjk-date-format(pfx: taisho-pfx, date, established: taisho.year(), arabic: arabic)
  } else if date < heisei {
    cjk-date-format(pfx: showa-pfx, date, established: showa.year(), arabic: arabic)
  } else if date < reiwa {
    cjk-date-format(pfx: heisei-pfx, date, established: heisei.year(), arabic: arabic)
  } else {
    cjk-date-format(pfx: reiwa-pfx, date, established: reiwa.year(), arabic: arabic)
  }
}

/// Format a number in Chinese currency style. This function is based on ```typ numbering("壹", v)``` but adds units for the whole number and the first two decimal places.
/// ```example
/// #cjk-num-format.daxie(123456)\
/// #cjk-num-format.daxie(8642.99)\
/// #cjk-num-format.daxie(2356, u1: "圆", whole: "正")
/// ```
///
/// - v (int, float): The value to format.
/// - u1 (str): The unit for the whole number (e.g., "元").
/// - u2 (str): The unit for the first decimal place (e.g., "角").
/// - u3 (str): The unit for the second decimal place (e.g., "分").
/// - whole (str): The string to append if the value is a whole number (e.g., "整").
/// -> str
#let daxie(v, u1: "元", u2: "角", u3: "分", whole: "整") = {
  // this only works for Chinese
  // check if float. If so, check the last two digits
  let ret = numbering("壹", calc.floor(v)) + u1
  if type(v) == float {
    let last-two-digits = calc.round(calc.rem(v * 100, 100))
    let first-digit = calc.floor(last-two-digits / 10)
    let second-digit = calc.floor(calc.rem(last-two-digits, 10))
    if first-digit != 0 {
      ret += numbering("壹", first-digit) + u2
    }
    if second-digit != 0 {
      ret += numbering("壹", second-digit) + u3
    }
    if last-two-digits == 0 {
      ret += whole
    }
  } else {
    ret += whole
  }
  ret
}
