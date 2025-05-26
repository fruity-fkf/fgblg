# Markdown Capabilities Test

A file to demonstrate all useful Markdown syntax features.

---

## Headings

# H1 Heading
## H2 Heading
### H3 Heading
#### H4 Heading
##### H5 Heading
###### H6 Heading

---

## Emphasis

*Italic text with asterisks*

_Italic text with underscores_

**Bold text with asterisks**

__Bold text with underscores__

***Bold and italic***

~~Strikethrough~~

---

## Lists

### Unordered list

- Item 1
- Item 2
  - Nested Item
    - Deeper level

### Ordered list

1. First item
2. Second item
   1. Subitem
   2. Subitem

---

## Code

### Inline code

Use `printf()` to print text.

### Code block

````rust
fn main() {
    println!("Hello, world!");
}
````

### Syntax-highlighted block

```python
def greet(name):
    print(f"Hello, {name}!")
```

---

## Blockquotes

> This is a blockquote.
>
> > Nested blockquote.

---

## Links

[Rust](https://rust-lang.org)

<https://github.com>

---

## Images

![Rust logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

---

## Tables

| Feature    | Supported | Notes                    |
|------------|-----------|--------------------------|
| Tables     | ✅        | GFM only                 |
| Emojis     | ✅        | GitHub / some renderers  |
| Syntax     | ✅        | Language-specific blocks |

---

## Horizontal Rule

---

## Task Lists

- [x] Item 1
- [ ] Item 2
- [ ] Item 3

---

## Emoji

👍 🎉 🚀 🤖

---

## HTML in Markdown

<div style="color: red; font-weight: bold;">
This text is styled with raw HTML.
</div>

---

## Footnotes

This is a sentence with a footnote.[^1]

[^1]: This is the footnote content.

---

## Definition List (supported in some engines)

Term 1
: Definition of term 1

Term 2
: Definition of term 2

---

## Math (only works in renderers that support KaTeX/LaTeX)

Inline math: $E = mc^2$

Block math:

$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$
