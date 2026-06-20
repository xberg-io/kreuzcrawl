// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/kreuzberg-dev/kreuzcrawl", from: "0.3.0-rc.83"),
    ],
    targets: [
        .testTarget(
            name: "KreuzcrawlE2ETests",
            dependencies: [.product(name: "Kreuzcrawl", package: "kreuzcrawl")]
        ),
    ]
)
