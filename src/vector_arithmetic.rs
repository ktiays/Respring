use crate::additive_arithmetic::AdditiveArithmetic;

/// A type that can serve as the animatable data of an animatable type.
pub trait VectorArithmetic: AdditiveArithmetic + Clone {
    /// Returns the dot-product of this vector arithmetic instance with itself.
    fn magnitude_squared(&self) -> f64;

    /// Multiplies each component of this value by the given value.
    fn scale_by(&mut self, scalar: f64);

    /// Returns a value with each component of this value multiplied by the given value.
    #[inline]
    fn scaled_by(mut self, scalar: f64) -> Self {
        self.scale_by(scalar);
        self
    }
}

macro_rules! vector_arithmetic_impl {
    ($($t:ty)*) => ($(
        impl VectorArithmetic for $t {
            fn magnitude_squared(&self) -> f64 {
                (*self as f64) * (*self as f64)
            }

            fn scale_by(&mut self, scalar: f64) {
                *self = (*self as f64 * scalar) as Self;
            }
        }
    )*)
}

vector_arithmetic_impl! { f32 f64 }
