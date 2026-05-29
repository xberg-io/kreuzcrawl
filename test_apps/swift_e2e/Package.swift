// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .binaryTarget(name: "Kreuzcrawl", url: "https://github.com/kreuzberg-dev/kreuzcrawl/releases/download/v0.3.0-rc.38/Kreuzcrawl-rs.artifactbundle.zip", checksum: "__ALEF_SWIFT_CHECKSUM__"),
    ],
    targets: [
        .testTarget(
            name: "KreuzcrawlE2ETests",
            dependencies: [.binaryTarget(name: "Kreuzcrawl")]
        ),
    ]
)
