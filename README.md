# Respring

An open-source implementation of SwiftUI's `Spring` animation.

This repository aims to provide an interface and behavior identical to SwiftUI's native `Spring` implementation.
This package allows you to use `Spring` to drive your animations without depending on SwiftUI.

## Usage

Use `Spring` to convert between different representations of spring parameters:

```swift
let spring = Spring(duration: 0.5, bounce: 0.3)
let (mass, stiffness, damping) = (spring.mass, spring.stiffness, spring.damping)
// (1.0, 157.9, 17.6)


let spring2 = Spring(mass: 1, stiffness: 100, damping: 10)
let (duration, bounce) = (spring2.duration, spring2.bounce)
// (0.63, 0.5)
```

You can also use it to query for a spring’s position and its other properties for a given set of inputs:

```swift
func unitPosition(time: TimeInterval) -> Double {
    let spring = Spring(duration: 0.5, bounce: 0.3)
    return spring.position(target: 1.0, time: time)
}
```

## Installation

Adding `Respring` to the dependencies value of your `Package.swift` or the Package list in Xcode.

```swift
dependencies: [
    .package(url: "https://github.com/ktiays/Respring.git", from: "1.0.0")
]
```

Normally you'll want to depend on the `Respring` target:

```swift
.product(name: "Respring", package: "Respring")
```
