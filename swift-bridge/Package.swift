// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "NaturalLanguageBridge",
    platforms: [.macOS(.v13)],
    products: [
        .library(
            name: "NaturalLanguageBridge",
            type: .static,
            targets: ["NaturalLanguageBridge"]
        ),
    ],
    targets: [
        .target(
            name: "NaturalLanguageBridge",
            path: "Sources/NaturalLanguageBridge",
            publicHeadersPath: "include"
        ),
    ]
)
