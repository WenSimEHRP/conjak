#import "src/lib.typ": *
#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "Sarasa Fixed SC")

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
