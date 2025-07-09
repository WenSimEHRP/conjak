#import "src/lib.typ": *
#set page(width: 20cm, height: auto, margin: 1cm)
#set text(font: "Noto Serif CJK SC")

#grid(
  columns: (1fr,) * 3,
  rows: auto,
  inset: .5em,
  ..(("zh", "cn"), ("zh", "sg"), ("zh", "mo"), ("zh", "hk"), ("zh", "tw"), ("ja", none), ("ko", none)).map(it => [
    #it\
    // varies based on locale settings
    #set text(lang: it.at(0), region: it.at(1))
    #sep-by-ten-thousands(12315649494)\
    #sep-by-ten-thousands(1145141919810)\
    #sep-by-ten-thousands(10086)\
    #cjk-date-format(datetime(year: 1949, month: 7, day: 1))\
    #cjk-date-format(datetime(year: 2000, month: 12, day: 31))\
    #cjk-date-format(datetime(year: 1949, month: 7, day: 30), arabic: true)\
    #roc-date-format(datetime(year: 1949, month: 9, day: 30), arabic: true)\
    #roc-date-format(datetime(year: 1912, month: 7, day: 30), arabic: false)\
    #roc-date-format(datetime(year: 1911, month: 7, day: 31), alternative-30: true)\
    #japan-date-format(datetime(year: 1912, month: 7, day: 29), alternative-20: true)\
    #japan-date-format(datetime(year: 1912, month: 7, day: 30))\
    #japan-date-format(datetime(year: 1989, month: 1, day: 1), alternative-january: true)\
    #japan-date-format(datetime(year: 1989, month: 1, day: 6))\
    #japan-date-format(datetime(year: 1989, month: 1, day: 6), arabic: true)\
    #japan-date-format(datetime(year: 1989, month: 1, day: 7))\
    #japan-date-format(datetime(year: 2025, month: 6, day: 17))\
    #japan-date-format(datetime(year: 9999, month: 6, day: 17))\
    #juche-date-format(datetime(year: 2025, month: 6, day: 17))\
    #juche-date-format(datetime(year: 1912, month: 6, day: 17))\
  ]),
  [
    Lunar calendar:\
    #for it in (
      datetime(year: 2025, month: 6, day: 17),
      datetime(year: 2023, month: 10, day: 18),
      datetime(year: 1912, month: 6, day: 19),
      datetime(year: 1901, month: 12, day: 20),
    ) {
      to_lunar(it) + linebreak()
    }
  ],
  ..range(4).map(i => grid.hline(y: i))
)

Source code for this example:

#import "@preview/zebraw:0.5.5": *
#show: zebraw.with(..zebraw-themes.zebra)
#raw(read("example.typ"), lang: "typ", block: true)
