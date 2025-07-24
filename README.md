# Respring

An open-source implementation of SwiftUI's `Spring` animation.

This repository aims to provide an interface and behavior identical to SwiftUI's native `Spring` implementation.
This package allows you to use `Spring` to drive your animations without depending on SwiftUI.

## Usage

### Swift

Use `Spring` to convert between different representations of spring parameters:

```swift
let spring = Spring(duration: 0.5, bounce: 0.3)
let (mass, stiffness, damping) = (spring.mass, spring.stiffness, spring.damping)
// (1.0, 157.9, 17.6)


let spring2 = Spring(mass: 1, stiffness: 100, damping: 10)
let (duration, bounce) = (spring2.duration, spring2.bounce)
// (0.63, 0.5)
```

You can also use it to query for a spring's position and its other properties for a given set of inputs:

```swift
func unitPosition(time: TimeInterval) -> Double {
    let spring = Spring(duration: 0.5, bounce: 0.3)
    return spring.position(target: 1.0, time: time)
}
```

### Rust

The Rust implementation provides the same functionality with a similar API:

```rust
use respring::Spring;

let spring = Spring::with_duration_bounce(0.5, 0.3);
let (mass, stiffness, damping) = (spring.mass, spring.stiffness(), spring.damping());
// (1.0, 157.91367041742973, 17.59291886010284)

let spring2 = Spring::with_mass_stiffness_damping(1.0, 100.0, 10.0, false);
let (duration, bounce) = (spring2.duration(), spring2.bounce());
// (0.6283185307179586, 0.5)
```

Query for a spring's value and other properties for a given set of inputs:

```rust
fn unit_position(time: f64) -> f64 {
    let spring = Spring::with_duration_bounce(0.5, 0.3);
    // Using value method with target=1.0, initial_velocity=0.0
    spring.value(1.0, 0.0, time)
}
```

Both implementations also provide convenient preset springs:

```rust
// Rust
let smooth = Spring::smooth();     // No bounce
let snappy = Spring::snappy();     // Small bounce (0.15)
let bouncy = Spring::bouncy();     // Higher bounce (0.3)
```

```swift
// Swift  
let smooth = Spring.smooth         // No bounce
let snappy = Spring.snappy         // Small bounce (0.15)
let bouncy = Spring.bouncy         // Higher bounce (0.3)
```

## Installation

### Swift Package Manager

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

### Cargo (Rust)

Add this to your `Cargo.toml`:

```toml
[dependencies]
respring = "0.1.1"
```

Or install using cargo:

```bash
cargo add respring
```