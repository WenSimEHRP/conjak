#import "@preview/tidy:0.4.3"
#import "@preview/cmarker:0.1.6"
#set text(font: "Sarasa UI SC", size: 11pt)
#show raw: set text(font: "Sarasa Fixed SC")
#let current-chapter = state("chapter", none)
#import "@preview/zebraw:0.5.5": *
#show: zebraw.with(..zebraw-themes.zebra)
#let version = sys.inputs.at("version", default: "INDEV")
#set page(
  paper: "a4",
  header: context {
    let a = counter(page).get()
    if a.at(0) != 1 {
      [
        _#current-chapter.get()_ #h(1fr) CJK Num Format #h(1em) *#version*
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
  = CJK Num Format
  Format dates and numbers.

  #v(1fr)

  *#version* #h(1em) #datetime.today().display()

  #link("https://github.com/wensimehrp/cjk-num-format")

  Created by Jeremy Gao

  #v(1fr)
  #outline(target: heading.where(level: 2), indent: 0em)
  #pagebreak()
]

// don't show the title

#{
  show <markdown-ignore>: none
  show heading.where(level: 1): none
  cmarker.render(read("readme.md"))
}
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
      cjk-num-format: lib,
    ),
  )
  tidy.show-module(
    docs,
    // style: docs-style,
    style: tidy.styles.default,
    show-outline: false,
  )
}
