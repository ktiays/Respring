//
//  Created by ktiays on 2024/11/20.
//  Copyright (c) 2024 ktiays. All rights reserved.
//

import Foundation

extension Spring {

    /// Creates a spring with the specified duration and damping ratio.
    ///
    /// - Parameters:
    ///   - settlingDuration: The approximate time it will take for the spring to come to rest.
    ///   - dampingRatio: The amount of drag applied as a fraction of the amount needed to produce critical damping.
    ///   - epsilon: The threshhold for how small all subsequent values need to be before the spring is considered to have settled.
    public init(settlingDuration: TimeInterval, dampingRatio: Double, epsilon: Double = 0.001) {
        let duration = min(max(settlingDuration, .ulpOfOne), 10)
        let dampingRatio = min(max(dampingRatio, .ulpOfOne), 1)
        if dampingRatio >= 1 {

        } else {
            var d0 = dampingRatio * dampingRatio
            let d11 = sqrt(1 - d0)
            let d14 = duration * d0
            d0 = 0.64
        }
        fatalError()
    }
}
