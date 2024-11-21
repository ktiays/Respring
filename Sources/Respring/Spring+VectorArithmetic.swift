//
//  Created by ktiays on 2024/11/21.
//  Copyright (c) 2024 ktiays. All rights reserved.
//

import Foundation

extension Spring {

    /// Calculates the value of the spring at a given time given a target amount of change.
    public func value<V>(target: V, initialVelocity: V = .zero, time: TimeInterval) -> V where V: VectorArithmetic {
        fatalError()
    }

    /// Calculates the velocity of the spring at a given time given a target amount of change.
    public func velocity<V>(target: V, initialVelocity: V = .zero, time: TimeInterval) -> V where V: VectorArithmetic {
        fatalError()
    }

    /// Updates the current  value and velocity of a spring.
    ///
    /// - Parameters:
    ///   - value: The current value of the spring.
    ///   - velocity: The current velocity of the spring.
    ///   - target: The target that `value` is moving towards.
    ///   - deltaTime: The amount of time that has passed since the spring was
    ///     at the position specified by `value`.
    public func update<V>(value: inout V, velocity: inout V, target: V, deltaTime: TimeInterval) where V: VectorArithmetic {
        fatalError()
    }

    /// Calculates the force upon the spring given a current position, target, and velocity amount of change.
    ///
    /// This value is in units of the vector type per second squared.
    public func force<V>(target: V, position: V, velocity: V) -> V where V: VectorArithmetic {
        fatalError()
    }
}
