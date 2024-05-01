# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/TribeMedia/atm0s-media-server/releases/tag/media-server-protocol-v0.1.0) - 2024-05-01

### Added
- bitrate allocator with both egress and ingress. ([#268](https://github.com/TribeMedia/atm0s-media-server/pull/268))
- bitrate control with Twcc and Remb ([#265](https://github.com/TribeMedia/atm0s-media-server/pull/265))
- add cluster metadata publish and subscribe options: peer and track info ([#260](https://github.com/TribeMedia/atm0s-media-server/pull/260))
- connector support http export transport ([#233](https://github.com/TribeMedia/atm0s-media-server/pull/233))
- connector with persistent queue  ([#161](https://github.com/TribeMedia/atm0s-media-server/pull/161))
- F32p2 conversion to from f32 ([#152](https://github.com/TribeMedia/atm0s-media-server/pull/152))
- connector external event log - protobuf ([#132](https://github.com/TribeMedia/atm0s-media-server/pull/132))

### Fixed
- try fixing protoc release ([#155](https://github.com/TribeMedia/atm0s-media-server/pull/155))

### Other
- BREAKING CHANGE: switching to sans-io-runtime ([#257](https://github.com/TribeMedia/atm0s-media-server/pull/257))
- release server 0.1.1 ([#123](https://github.com/TribeMedia/atm0s-media-server/pull/123))
