// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Respring",
    platforms: [
        .macOS(.v10_13),
        .iOS(.v12),
        .watchOS(.v4),
        .tvOS(.v12),
    ],
    products: [
        .library(
            name: "Respring",
            targets: ["Respring"]
        )
    ],
    targets: [
        .target(
            name: "Respring"
        ),
        .testTarget(
            name: "RespringTests",
            dependencies: ["Respring"]
        ),
    ]
)
