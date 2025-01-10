# relative-luminance

[![CI](https://github.com/spenserblack/relative-luminance-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/spenserblack/relative-luminance-rs/actions/workflows/ci.yml)

This is a small crate that provides utilities to handle
[relative luminance][relative-luminance]. In short, among RGB color channels, green
represents the majority of light perceived by the typical human eye, then red, then
blue. This can explain why white text may be hard to read on a green (`#00FF00`)
background, yet black text can be hard to read on a blue (`#0000FF`) background.

| background | foreground | readability |
| :--------: | :--------: | :---------: |
| `#00FF00` | `#FFFFFF` | hard to read |
| `#00FF00` | `#000000` | readable |
| `#0000FF` | `#FFFFFF` | readable |
| `#0000FF` | `#000000` | hard to read |

Run `cargo run --example contrast` to see an example of how this looks.

## Example usage

```rust
use relative_luminance::*;

// Choose a foreground color that contrasts with the background color
let fg_color = if bg_color.relative_luminance() > 0.5 {
    dark_color
} else {
    light_color
};

println!("{}", "Hello, World!".color(fg_color).on_color(bg_color));
```

[relative-luminance]: https://en.wikipedia.org/wiki/Relative_luminance
