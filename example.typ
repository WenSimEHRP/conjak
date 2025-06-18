#import "src/lib.typ": *
#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "Sarasa Fixed SC")
#show raw: set text(font: "Sarasa Fixed SC")

#grid(
  columns: 2,
  gutter: 1em,
  [#set text(lang: "zh")
    #sep-by-ten-thousands(12345670000000)\
    #sep-by-ten-thousands(1145141919810)\
    #sep-by-ten-thousands(1145141919810000)\
    #sep-by-ten-thousands(135700012255)\

    #set text(lang: "zh", region: "tw")
    #sep-by-ten-thousands(12345670000000)\
    #sep-by-ten-thousands(1145141919810)\
    #sep-by-ten-thousands(1145141919810000)\
    #sep-by-ten-thousands(135700012255)\
  ],
  [#set text(lang: "ja")
    #sep-by-ten-thousands(12345670000000)\
    #sep-by-ten-thousands(1145141919810)\
    #sep-by-ten-thousands(1145141919810000)\
    #sep-by-ten-thousands(135700012255)\

    #set text(lang: "ko")
    #sep-by-ten-thousands(12345670000000)\
    #sep-by-ten-thousands(1145141919810)\
    #sep-by-ten-thousands(1145141919810000)\
    #sep-by-ten-thousands(135700012255)\
  ]
)

#pagebreak()
#set page(paper: "a4", margin: auto)

#grid(
  columns: (1fr,) * 3,
  rows: (1fr,) * 3,
  inset: .5em,
  ..(("zh", "cn"), ("zh", "sg"), ("zh", "mo"), ("zh", "hk"), ("zh", "tw"), ("ja", none), ("ko", none)).map(it => [
    #it\
    #set text(lang: it.at(0), region: it.at(1))
    #cjk-date-format(datetime(year: 1949, month: 7, day: 1))\
    #cjk-date-format(datetime(year: 1949, month: 7, day: 1), arabic: true)\
    #roc-date-format(datetime(year: 1949, month: 9, day: 30), arabic: true)\
    #roc-date-format(datetime(year: 1912, month: 7, day: 1), arabic: false)\
    #roc-date-format(datetime(year: 1911, month: 7, day: 1))\
    #japan-date-format(datetime(year: 1912, month: 10, day: 1))\
    #japan-date-format(datetime(year: 1989, month: 1, day: 1))\
    #japan-date-format(datetime(year: 1989, month: 1, day: 6))\
    #japan-date-format(datetime(year: 1989, month: 1, day: 6), arabic: true)\
    #japan-date-format(datetime(year: 1989, month: 1, day: 7))\
    #japan-date-format(datetime(year: 2025, month: 6, day: 17))\
    #japan-date-format(datetime(year: 9999, month: 6, day: 17))\
    #juche-date-format(datetime(year: 2025, month: 6, day: 17))\
    #juche-date-format(datetime(year: 1912, month: 6, day: 17))\
  ]),
  ..range(4).map(i => grid.hline(y: i))
)
