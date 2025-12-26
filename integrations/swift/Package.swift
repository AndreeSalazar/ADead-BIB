// swift-tools-version:5.5
// ADead-BIB Swift Package
// Author: Eddi Andreé Salazar Matos

import PackageDescription

let package = Package(
    name: "ADead",
    platforms: [
        .macOS(.v11),
        .iOS(.v14),
        .watchOS(.v7),
        .tvOS(.v14)
    ],
    products: [
        .library(
            name: "ADead",
            targets: ["ADead"]
        ),
    ],
    dependencies: [],
    targets: [
        .target(
            name: "ADead",
            dependencies: [],
            path: "Sources/ADead"
        ),
        .testTarget(
            name: "ADeadTests",
            dependencies: ["ADead"]
        ),
    ]
)
