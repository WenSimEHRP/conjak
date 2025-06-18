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

/// Generate a smart CJK string based on the language and region.
///
/// - zh (dict, str): String(s) for Chinese. You can optionally provide a dictionary with keys `cn`, `tw`, `hk`, etc. to specify different strings for different regions.
/// - ja (str): String for Japanese.
/// - ko (str): String for Korean.
/// -> content
#let cjk-string(
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
    zh.cn // default to zh-cn
  }
}

#let _year-with-beginning(pfx, negative-pfx: none, year, arabic) = {
  let year-str = cjk-string(
    zh: "年",
    ja: "年",
    ko: "년",
  )
  let first-year = cjk-string(
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
///
/// - pfx (str, content): Prefix for the date string.
/// - negative-pfx (str, content): Prefix for negative years.
/// - date (datetime): The date to format.
/// - established (int): The year when the era was established.
/// - arabic (auto, bool): Whether to use Arabic numerals for the year.
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
  let month-str = cjk-string(
    zh: "月",
    ja: "月",
    ko: "월",
  )
  let day-str = cjk-string(
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
///
/// - pfx (str, content): Prefix for the date string.
/// - negative-pfx (str, content): Prefix for negative years.
/// - date (datetime): The date to format.
/// - arabic (auto, bool): Whether to use Arabic numerals for the year.
/// -> content
#let roc-date-format(date, pfx: auto, negative-pfx: auto, arabic: auto) = {
  let default-pfx = cjk-string(
    zh: (cn: "民国", tw: "民國"),
    ja: "民国",
    ko: "민국",
  )
  let default-negative-pfx = cjk-string(
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
  let default-pfx = cjk-string(
    zh: (cn: "主体", tw: "主體"),
    ja: "主体",
    ko: "주체",
  )
  let default-negative-pfx = cjk-string(
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
///
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
  let meiji-pfx = cjk-string(
    zh: "明治",
    ja: "明治",
    ko: "명치",
  )
  let taisho-pfx = cjk-string(
    zh: "大正",
    ja: "大正",
    ko: "대정",
  )
  let showa-pfx = cjk-string(
    zh: "昭和",
    ja: "昭和",
    ko: "쇼와",
  )
  let heisei-pfx = cjk-string(
    zh: "平成",
    ja: "平成",
    ko: "헤이세이",
  )
  let reiwa-pfx = cjk-string(
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
