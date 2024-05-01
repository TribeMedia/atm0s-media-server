# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/TribeMedia/atm0s-media-server/releases/tag/media-server-core-v0.1.0) - 2024-05-01

### Added
- bitrate allocator with both egress and ingress. ([#268](https://github.com/TribeMedia/atm0s-media-server/pull/268))
- bitrate control with Twcc and Remb ([#265](https://github.com/TribeMedia/atm0s-media-server/pull/265))
- channel pub-sub feature and tests. cluster integration test ([#262](https://github.com/TribeMedia/atm0s-media-server/pull/262))
- add cluster metadata publish and subscribe options: peer and track info ([#260](https://github.com/TribeMedia/atm0s-media-server/pull/260))

### Fixed
- missing clear room_map in cluster cause room failed to restart ([#267](https://github.com/TribeMedia/atm0s-media-server/pull/267))

### Other
- BREAKING CHANGE: switching to sans-io-runtime ([#257](https://github.com/TribeMedia/atm0s-media-server/pull/257))