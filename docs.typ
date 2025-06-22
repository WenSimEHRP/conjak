#import "@preview/tidy:0.4.3"
#set text(font: "Sarasa UI SC", size: 11pt)
#show raw: set text(font: "Sarasa Fixed SC")
#let current-chapter = state("chapter", none)
#import "@preview/zebraw:0.5.5": *
#show: zebraw.with(..zebraw-themes.zebra)
#let version = sys.inputs.at("version", default: "INDEV")
// somehow this line was not working. Probably because of nixos?
#let date = sys.inputs.at("date", default: datetime.today().display())
#set page(
  paper: "a4",
  header: context {
    let a = counter(page).get()
    if a.at(0) != 1 {
      [
        _#current-chapter.get()_ #h(1fr) Conjak #h(1em) *#version*
      ]
    }
  },
  numbering: "1",
)
#set par(justify: true)
#show heading.where(level: 2): it => context {
  current-chapter.update(none)
  pagebreak(weak: true)
  current-chapter.update(it.body)
  it
}
#[
  #set align(center)
  = Conjak
  Format dates and numbers.

  #v(1fr)

  *#version* #h(1em) #date

  #link("https://github.com/wensimehrp/conjak")

  Created by Jeremy Gao

  #v(1fr)
  #outline(target: heading.where(level: 2), indent: 0em)
  #pagebreak()
]

== Goals

This library provides basic CJK (Chinese, Japanese, and Korean) number and date formatting utilities.
It is designed with simplicity in mind, and with maximum compatibility with different CJK languages and regions. Features include:

- Formatting numbers with thousands separators.
  - including _Daxie_ used by Chinese.
- Formatting dates in various CJK date formats.
- Automatic handling of locals, based on the current text language and region (i.e., the `lang` and `region` fields of `text`)

I do not aim to cover all possible utilities related to CJK number formatting, but rather to provide a solid foundation for common use cases. If you have specific needs or suggestions, feel free to open an issue on the GitHub repository.

=== Localization

*DISCLAIMER*: I am not a Japanese nor a Korean expert, so I have relied on existing resources
(i.e. Wikipedia, LLM) to implement features for these languages. If you find any mistakes or have suggestions for improvements, please let me know.

Functions provided by this library can adapt to the current text language and region
(hence their outputs are `content` but not `string` or `int`). See each function's documentation for details.

== Name

The name "Conjak" is derived from "Konjac", a type of plant that is commonly used in East Asian cuisine. The "K" and "C" are swapped to distinguish the package name from the plant.

Here's a picture of konjac:

#figure(
  image("konjac.jpg"),
  caption: [
    Processed konjac\
    #link("https://commons.wikimedia.org/w/index.php?curid=3106408")[By: chidorian - Flickr, CC BY-SA 2.0]
  ],
)

== Library Functions

#{
  import "src/lib.typ"
  show heading.where(level: 3): it => {
    colbreak(weak: true)
    it
  }
  let docs = tidy.parse-module(
    read("src/lib.typ"),
    old-syntax: true,
    scope: (
      conjak: lib,
    ),
  )
  tidy.show-module(
    docs,
    // style: docs-style,
    style: tidy.styles.default,
    show-outline: false,
  )
}
