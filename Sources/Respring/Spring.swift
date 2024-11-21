//
//  Created by ktiays on 2024/11/20.
//  Copyright (c) 2024 ktiays. All rights reserved.
//

import Foundation

extension Double {
    static let tau: Self = 2 * .pi
}

/// A representation of a spring's motion.
public struct Spring: Hashable, Sendable {

    var angularFrequency: Double
    var decayConstant: Double
    var _mass: Double
    
    init(angularFrequency: Double, decayConstant: Double, mass: Double) {
        self.angularFrequency = angularFrequency
        self.decayConstant = decayConstant
        self._mass = mass
    }
}
