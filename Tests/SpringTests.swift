//
//  Created by ktiays on 2024/11/20.
//  Copyright (c) 2024 ktiays. All rights reserved.
//

import SwiftUI
import Testing

@testable import Respring

@inlinable
func with(_ body: () -> Void) {
    body()
}

@available(iOS 17.0, macOS 14.0, *)
func == (_ lhs: Respring.Spring, _ rhs: SwiftUI.Spring) -> Bool {
    var angularFrequency: Double = 0
    var decayConstant: Double = 0
    var mass: Double = 0
    for child in Mirror(reflecting: rhs).children {
        if let label = child.label {
            switch label {
            case "angularFrequency":
                angularFrequency = child.value as! Double
            case "decayConstant":
                decayConstant = child.value as! Double
            case "_mass":
                mass = child.value as! Double
            default:
                break
            }
        }
    }
    return lhs == Respring.Spring(angularFrequency: angularFrequency, decayConstant: decayConstant, mass: mass)
}

@available(iOS 17.0, macOS 14.0, *)
@Test func durationBounce() {
    with {
        let spring = SwiftUI.Spring()
        let respring = Respring.Spring()
        #expect(respring == spring)
        #expect(respring.bounce == spring.bounce)
        #expect(respring.duration == spring.duration)
    }
    with {
        let spring = SwiftUI.Spring(duration: 0.7, bounce: 0.2)
        let respring = Respring.Spring(duration: 0.7, bounce: 0.2)
        #expect(respring == spring)
        #expect(respring.bounce == spring.bounce)
        #expect(respring.duration == spring.duration)
    }
    with {
        let spring = SwiftUI.Spring(duration: 0.3, bounce: 1.2)
        let respring = Respring.Spring(duration: 0.3, bounce: 1.2)
        #expect(respring == spring)
        #expect(respring.bounce == spring.bounce)
        #expect(respring.duration == spring.duration)
    }
    with {
        let spring = SwiftUI.Spring(duration: 0.7, bounce: -0.2)
        let respring = Respring.Spring(duration: 0.7, bounce: -0.2)
        #expect(respring == spring)
        #expect(respring.bounce == spring.bounce)
        #expect(respring.duration == spring.duration)
    }
    with {
        let spring = SwiftUI.Spring(duration: 0.7, bounce: -1.2)
        let respring = Respring.Spring(duration: 0.7, bounce: -1.2)
        #expect(respring == spring)
        #expect(respring.bounce.isNaN && spring.bounce.isNaN)
        #expect(respring.duration.isNaN && spring.duration.isNaN)
    }
}

@available(iOS 17.0, macOS 14.0, *)
@Test func responseDampingRatio() {
    with {
        let spring = SwiftUI.Spring(response: 0.4, dampingRatio: 0.8)
        let respring = Respring.Spring(response: 0.4, dampingRatio: 0.8)
        #expect(respring == spring)
        #expect(respring.response == spring.response)
        #expect(respring.dampingRatio == spring.dampingRatio)
    }
    with {
        let spring = SwiftUI.Spring(response: 0.2, dampingRatio: 1.8)
        let respring = Respring.Spring(response: 0.2, dampingRatio: 1.8)
        #expect(respring == spring)
        #expect(respring.response == spring.response)
        #expect(respring.dampingRatio == spring.dampingRatio)
    }
    with {
        let spring = SwiftUI.Spring(response: 1.3, dampingRatio: -0.1)
        let respring = Respring.Spring(response: 1.3, dampingRatio: -0.1)
        #expect(respring == spring)
        #expect(respring.response == spring.response)
        #expect(respring.dampingRatio == spring.dampingRatio)
    }
    with {
        let spring = SwiftUI.Spring(response: 1, dampingRatio: 1)
        let respring = Respring.Spring(response: 1, dampingRatio: 1)
        #expect(respring == spring)
        #expect(respring.response == spring.response)
        #expect(respring.dampingRatio == spring.dampingRatio)
    }
}

@available(iOS 17.0, macOS 14.0, *)
@Test func massStiffnessDamping() {
    with {
        let spring = SwiftUI.Spring(mass: 1, stiffness: 0.5, damping: 0.7)
        let respring = Respring.Spring(mass: 1, stiffness: 0.5, damping: 0.7)
        #expect(respring == spring)
        #expect(respring.stiffness == spring.stiffness)
        #expect(respring.damping == spring.damping)
    }
    with {
        let spring = SwiftUI.Spring(mass: 0.6, stiffness: 120, damping: 1.1)
        let respring = Respring.Spring(mass: 0.6, stiffness: 120, damping: 1.1)
        #expect(respring == spring)
        #expect(respring.stiffness == spring.stiffness)
        #expect(respring.damping == spring.damping)
    }
    with {
        let spring = SwiftUI.Spring(mass: 0.6, stiffness: 120, damping: 1.1, allowOverDamping: true)
        let respring = Respring.Spring(mass: 0.6, stiffness: 120, damping: 1.1, allowOverDamping: true)
        #expect(respring == spring)
        #expect(respring.stiffness == spring.stiffness)
        #expect(respring.damping == spring.damping)
    }
    with {
        let spring = SwiftUI.Spring(mass: 0.6, stiffness: 0.1, damping: 10, allowOverDamping: true)
        let respring = Respring.Spring(mass: 0.6, stiffness: 0.1, damping: 10, allowOverDamping: true)
        #expect(respring == spring)
        #expect(respring.stiffness == spring.stiffness)
        #expect(respring.damping == spring.damping)
    }
    with {
        let spring = SwiftUI.Spring(mass: 0, stiffness: 50, damping: 0.4)
        let respring = Respring.Spring(mass: 0, stiffness: 50, damping: 0.4)
        #expect(respring.stiffness.isNaN && spring.stiffness.isNaN)
        #expect(respring.damping.isNaN && spring.damping.isNaN)
    }
}

@available(iOS 17.0, macOS 14.0, *)
@Test func settlingDurationDampingRatio() {
    
}

@available(iOS 17.0, macOS 14.0, *)
@Test func vectorArithmetic() {
    with {
        let spring = SwiftUI.Spring()
        let respring = Respring.Spring()
        
        let springValue = spring.value(target: 1.2, time: 0.2)
        let respringValue = respring.value(target: 1.2, time: 0.2)
        #expect(springValue == respringValue)
        
        let springForce = spring.force(target: 0.2, position: 1, velocity: -0.2)
        let respringForce = respring.force(target: 0.2, position: 1, velocity: -0.2)
        #expect(springForce == respringForce)
        
        let springVelocity = spring.velocity(target: 1.2, initialVelocity: 0.1, time: 0.2)
        let respringVelocity = respring.velocity(target: 1.2, initialVelocity: 0.1, time: 0.2)
        #expect(springVelocity == respringVelocity)
        
        with {
            var springValue: Double = 0
            var springVelocity: Double = 0
            var respringValue: Double = 0
            var respringVelocity: Double = 0
            spring.update(value: &springValue, velocity: &springVelocity, target: 1.2, deltaTime: 0.16)
            respring.update(value: &respringValue, velocity: &respringVelocity, target: 1.2, deltaTime: 0.16)
            #expect(springValue == respringValue)
            
            spring.update(value: &springValue, velocity: &springVelocity, target: 1.2, deltaTime: 0.16)
            respring.update(value: &respringValue, velocity: &respringVelocity, target: 1.2, deltaTime: 0.16)
            #expect(springValue == respringValue)
            
            spring.update(value: &springValue, velocity: &springVelocity, target: 1.2, deltaTime: 0.16)
            respring.update(value: &respringValue, velocity: &respringVelocity, target: 1.2, deltaTime: 0.16)
            #expect(springValue == respringValue)
            
            spring.update(value: &springValue, velocity: &springVelocity, target: 1.2, deltaTime: 1)
            respring.update(value: &respringValue, velocity: &respringVelocity, target: 1.2, deltaTime: 1)
            #expect(springValue == respringValue)
        }
    }
    with {
        let spring = SwiftUI.Spring(duration: 0.7, bounce: 0.2)
        let respring = Respring.Spring(duration: 0.7, bounce: 0.2)
        
        let springValue = spring.value(target: 1.2, initialVelocity: 0.1, time: 0.2)
        let respringValue = respring.value(target: 1.2, initialVelocity: 0.1, time: 0.2)
        #expect(springValue == respringValue)
        
        let springForce = spring.force(target: 1.2, position: 0.1, velocity: 0.2)
        let respringForce = respring.force(target: 1.2, position: 0.1, velocity: 0.2)
        #expect(springForce == respringForce)
        
        let springVelocity = spring.velocity(target: 1.2, initialVelocity: 0.1, time: 0.2)
        let respringVelocity = respring.velocity(target: 1.2, initialVelocity: 0.1, time: 0.2)
        #expect(springVelocity == respringVelocity)
    }
    with {
        let spring = SwiftUI.Spring()
        let respring = Respring.Spring()
        
        let springValue = spring.value(target: 1.2, time: 0.2)
        let respringValue = respring.value(target: 1.2, time: 0.2)
        #expect(springValue == respringValue)
        
        let springForce = spring.force(target: 1, position: 0.3, velocity: 0)
        let respringForce = respring.force(target: 1, position: 0.3, velocity: 0)
        #expect(springForce == respringForce)
    }
    with {
        let spring = SwiftUI.Spring(duration: 0.7, bounce: -0.2)
        let respring = Respring.Spring(duration: 0.7, bounce: -0.2)
        
        let springValue = spring.value(target: 1.2, time: 0.2)
        let respringValue = respring.value(target: 1.2, time: 0.2)
        #expect(springValue == respringValue)
        
        let springForce = spring.force(target: 0.2, position: 1, velocity: -0.2)
        let respringForce = respring.force(target: 0.2, position: 1, velocity: -0.2)
        #expect(springForce == respringForce)
        
        let springVelocity = spring.velocity(target: 1.2, initialVelocity: 0.1, time: 0.2)
        let respringVelocity = respring.velocity(target: 1.2, initialVelocity: 0.1, time: 0.2)
        #expect(springVelocity == respringVelocity)
    }
    with {
        let spring = SwiftUI.Spring(mass: 0.6, stiffness: 0.1, damping: 10, allowOverDamping: true)
        let respring = Respring.Spring(mass: 0.6, stiffness: 0.1, damping: 10, allowOverDamping: true)
        
        let springValue = spring.value(target: 2, time: 0.3)
        let respringValue = respring.value(target: 2, time: 0.3)
        #expect(springValue == respringValue)
        
        let springForce = spring.force(target: 0.2, position: 1, velocity: -0.2)
        let respringForce = respring.force(target: 0.2, position: 1, velocity: -0.2)
        #expect(springForce == respringForce)
    }
}
