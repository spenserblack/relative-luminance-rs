//! This crate helps calculate [*relative* luminance][relative-luminance] values for
//! colors. This can help determine if a color *looks* light or dark, which may be
//! different from if a color is mathematically light or dark. For example, even bright
//! blue (`#0000FF`) can appear to be dark to many people.
//!
//! # Examples
//!
//! ```
//! use relative_luminance::{Luminance, Rgb};
//!
//! let black: Rgb<f32> = Rgb { r: 0.0, g: 0.0, b: 0.0 };
//! let white: Rgb<f32> = Rgb { r: 1.0, g: 1.0, b: 1.0 };
//! let green: Rgb<f32> = Rgb { r: 0.0, g: 1.0, b: 0.0 };
//! let blue: Rgb<f32> = Rgb { r: 0.0, g: 0.0, b: 1.0 };
//!
//! // If luminance is above 0.5, it can be considered bright.
//! assert_eq!(black.relative_luminance(), 0.0);
//! assert_eq!(white.relative_luminance(), 1.0);
//! assert!(green.relative_luminance() > 0.5);
//! assert!(blue.relative_luminance() < 0.5);
//! ```
//!
//! ```
//! use relative_luminance::{Luminance, Rgb};
//!
//! /// RGB channels in the range [0, 255].
//! struct MyRgb {
//!     r: u8,
//!     g: u8,
//!     b: u8,
//! }
//!
//! impl Luminance<f32> for MyRgb {
//!     fn luminance_rgb(&self) -> Rgb<f32> {
//!         Rgb {
//!             // Normalizing color channels to the range [0.0, 1.0]
//!             r: f32::from(self.r) / 255.0,
//!             g: f32::from(self.g) / 255.0,
//!             b: f32::from(self.b) / 255.0,
//!         }
//!     }
//! }
//!
//! let black = MyRgb { r: 0, g: 0, b: 0 };
//! let white = MyRgb { r: 255, g: 255, b: 255 };
//!
//! assert_eq!(black.relative_luminance(), 0.0);
//! assert_eq!(white.relative_luminance(), 1.0);
//! ```
//!
//! [relative-luminance]: https://en.wikipedia.org/wiki/Relative_luminance
use core::ops::{Add, Mul};
/// This trait is used to define numerical types that can be used to calculate relative
/// luminance values.
///
/// ```ignore
/// impl LuminanceValue for MyFloatLikeType {
///     type Channel = MyFloatLikeType;
///     type Weight = MyFloatLikeType;
///     type Weighted = MyFloatLikeType;
///
///     const RED_WEIGHT: MyFloatLikeType = 0.2126;
///     const GREEN_WEIGHT: MyFloatLikeType = 0.7152;
///     const BLUE_WEIGHT: MyFloatLikeType = 0.0722;
/// }
/// ```
///
/// The associated types can help with custom implementations where you need to mix
/// different types for precision.
pub trait LuminanceValue: Copy + Clone {
    /// The type used for RGB channels.
    ///
    /// ```ignore
    /// let r: Self::Channel = 1.0;
    ///
    /// let weighted_r = r * RED_WEIGHT;
    /// ```
    type Channel: Copy + Mul<Self::Weight, Output = Self::Weighted>;
    /// The type used for modifying the channel's value.
    ///
    /// ```ignore
    /// const RED_WEIGHT: Weight = 0.2126;
    ///
    /// let weighted_r = r * RED_WEIGHT;
    /// ```
    type Weight;
    /// The numerical type of the weighted channel.
    ///
    /// ```ignore
    /// let weighted: Weighted = r * RED_WEIGHT;
    /// ```
    type Weighted: Add<Self::Weighted, Output = Self::Weighted>;
    /// The modifier for the red channel. If the channel is within [0.0, 1.0], this
    /// value should be 0.2126.
    const RED_WEIGHT: Self::Weight;
    /// The modifier for the green channel. If the channel is within [0.0, 1.0], this
    /// value should be 0.7152.
    const GREEN_WEIGHT: Self::Weight;
    /// The modifier for the blue channel. If the channel is within [0.0, 1.0], this
    /// value should be 0.0722.
    const BLUE_WEIGHT: Self::Weight;
}

/// Gets the relative luminance of RGB channels.
fn relative_luminance<T: LuminanceValue>(
    r: T::Channel,
    g: T::Channel,
    b: T::Channel,
) -> T::Weighted {
    (r * T::RED_WEIGHT) + (g * T::GREEN_WEIGHT) + (b * T::BLUE_WEIGHT)
}

impl LuminanceValue for f32 {
    type Channel = f32;
    type Weight = f32;
    type Weighted = f32;
    const RED_WEIGHT: f32 = 0.2126;
    const GREEN_WEIGHT: f32 = 0.7152;
    const BLUE_WEIGHT: f32 = 0.0722;
}

impl LuminanceValue for f64 {
    type Channel = f64;
    type Weight = f64;
    type Weighted = f64;
    const RED_WEIGHT: f64 = 0.2126;
    const GREEN_WEIGHT: f64 = 0.7152;
    const BLUE_WEIGHT: f64 = 0.0722;
}

/// Struct for containing RGB channels that can be used for calculating luminance.
///
/// ```
/// # use relative_luminance::Rgb;
/// let green: Rgb<f32> = Rgb {
///     r: 0.0,
///     g: 1.0,
///     b: 0.0,
/// };
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Rgb<T: LuminanceValue> {
    pub r: T::Channel,
    pub g: T::Channel,
    pub b: T::Channel,
}

impl<T: LuminanceValue> Rgb<T> {
    /// Creates a new `Rgb<T>`
    pub fn new(r: T::Channel, g: T::Channel, b: T::Channel) -> Self {
        Rgb { r, g, b }
    }
    /// Gets the relative luminance of the color.
    fn relative_luminance(&self) -> T::Weighted {
        relative_luminance::<T>(self.r, self.g, self.b)
    }
}

/// Implement this trait on your color type to provide relative luminance calculations.
///
/// ```
/// # use relative_luminance::Luminance;
///
/// /// RGB channels in the range [0, 255].
/// struct MyRgb {
///     r: u8,
///     g: u8,
///     b: u8,
/// }
///
/// impl Luminance<f32> for MyRgb {
///     fn luminance_rgb(&self) -> relative_luminance::Rgb<f32> {
///         relative_luminance::Rgb {
///             r: f32::from(self.r) / 255.0,
///             g: f32::from(self.g) / 255.0,
///             b: f32::from(self.b) / 255.0,
///         }
///     }
/// }
///
/// let black = MyRgb { r: 0, g: 0, b: 0 };
/// let white = MyRgb { r: 255, g: 255, b: 255 };
///
/// assert_eq!(black.relative_luminance(), 0.0);
/// assert_eq!(white.relative_luminance(), 1.0);
/// ```
pub trait Luminance<T: LuminanceValue> {
    fn luminance_rgb(&self) -> Rgb<T>;

    fn relative_luminance(&self) -> T::Weighted {
        self.luminance_rgb().relative_luminance()
    }
}

impl<T: LuminanceValue> Luminance<T> for Rgb<T> {
    fn luminance_rgb(&self) -> Rgb<T> {
        *self
    }

    fn relative_luminance(&self) -> T::Weighted {
        // NOTE Small optimization to avoid cloning
        Rgb::relative_luminance(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_trail_impl_equals_struct_impl() {
        let rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
        assert_eq!(
            Rgb::relative_luminance(&rgb),
            Luminance::relative_luminance(&rgb)
        );
    }
}
