//
//  Created by ktiays on 2024/11/21.
//  Copyright (c) 2024 ktiays. All rights reserved.
//

import Foundation

extension CGFloat: VectorArithmetic {

    public var magnitudeSquared: Double {
        self * self
    }

    public mutating func scale(by rhs: Double) {
        self *= CGFloat(rhs)
    }
}

extension Double: VectorArithmetic {

    public var magnitudeSquared: Double {
        self * self
    }

    public mutating func scale(by rhs: Double) {
        self *= rhs
    }
}

extension Float: VectorArithmetic {

    public var magnitudeSquared: Double {
        Double(self) * Double(self)
    }

    public mutating func scale(by rhs: Double) {
        self *= Float(rhs)
    }
}
