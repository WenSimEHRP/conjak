# Changelog

## INDEV

- feat: `format-lunar-date`. Thanks to `chinese-lunisolar-calendar`, we can now format a western date to the traditional lunisolar date that is widely used in CJK areas.
- chore!: all functions are renamed to `verb-noun` form (to match English grammar). This means that all your favourite functions are now obsolete.
- feat: number format now supports larger numbers. The built-in numbering schemes `numbering("一", value)` and `numbering("壹", value)` only accept integers. By directly using [the crate behind it](https://github.com/magiclen/chinese-number), the function now supports a much wider number range (f64::MAX, `1.7976931348623157E+308f64`). However, due to the floating number nature, very large numbers are almost guaranteed to have rounding errors.

## 0.2.3

Initial public release.
