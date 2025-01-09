use hsl::HSL;
use owo_colors::{OwoColorize, Rgb};
use relative_luminance::{Luminance, Rgb as LuminanceRgb};

struct RgbWrapper(Rgb);

impl Luminance<f32> for RgbWrapper {
    fn luminance_rgb(&self) -> LuminanceRgb<f32> {
        LuminanceRgb {
            r: f32::from(self.0 .0) / 255.0,
            g: f32::from(self.0 .1) / 255.0,
            b: f32::from(self.0 .2) / 255.0,
        }
    }
}

fn main() {
    println!("In this example we use black text at >0.5 brightness and white text at <=0.5");
    let colors = [
        ("black", Rgb(0, 0, 0)),
        ("red", Rgb(255, 0, 0)),
        ("green", Rgb(0, 255, 0)),
        ("blue", Rgb(0, 0, 255)),
        ("yellow", Rgb(255, 255, 0)),
        ("magenta", Rgb(255, 0, 255)),
        ("cyan", Rgb(0, 255, 255)),
        ("white", Rgb(255, 255, 255)),
    ];

    println!("Using the lightness from HSL:");
    colors.iter().for_each(|(label, bg)| {
        let bg = *bg;
        let hsl = HSL::from_rgb(&[bg.0, bg.1, bg.2]);
        let fg = if hsl.l > 0.5 {
            Rgb(0, 0, 0)
        } else {
            Rgb(255, 255, 255)
        };

        println!(
            "{: ^10} ({} lightness)",
            label.color(fg).on_color(bg),
            hsl.l
        );
    });

    println!("Using relative luminance:");
    colors.iter().for_each(|(label, bg)| {
        let bg = *bg;
        let luminance = RgbWrapper(bg).relative_luminance();
        let fg = if luminance > 0.5 {
            Rgb(0, 0, 0)
        } else {
            Rgb(255, 255, 255)
        };

        println!(
            "{: ^10} ({} relative luminance)",
            label.color(fg).on_color(bg),
            luminance
        );
    });
}
