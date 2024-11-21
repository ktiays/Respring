//
//  Created by ktiays on 2024/11/21.
//  Copyright (c) 2024 ktiays. All rights reserved.
// 

import Foundation

/// A type that can serve as the animatable data of an animatable type.
///
/// VectorArithmetic extends the AdditiveArithmetic protocol with scalar multiplication
/// and a way to query the vector magnitude of the value.
public protocol VectorArithmetic: AdditiveArithmetic {
    
    /// Returns the dot-product of this vector arithmetic instance with itself.
    var magnitudeSquared: Double { get }
    
    /// Multiplies each component of this value by the given value.
    mutating func scale(by rhs: Double)
}

extension VectorArithmetic {
    
    /// Returns a value with each component of this value multiplied by the given value.
    public func scaled(by rhs: Double) -> Self {
        var result = self
        result.scale(by: rhs)
        return result
    }
    
    /// Interpolates this value with `other` by the specified amount.
    ///
    /// This is equivalent to `self = self + (other - self) * amount`.
    @inlinable
    public mutating func interpolate(towards other: Self, amount: Double) {
        self = self + (other - self).scaled(by: amount)
    }
    
    /// Returns this value interpolated with `other` by the specified amount.
    ///
    /// This result is equivalent to `self + (other - self) * amount`.
    @inlinable
    public func interpolated(towards other: Self, amount: Double) -> Self {
        self + (other - self).scaled(by: amount)
    }
}
