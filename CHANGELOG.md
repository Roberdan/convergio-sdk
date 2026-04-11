# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2](https://github.com/Roberdan/convergio-sdk/compare/convergio-sdk-v0.1.1...convergio-sdk-v0.1.2) (2026-04-11)


### Documentation

* add Agentic Manifesto + AGENTS.md ([2132c9b](https://github.com/Roberdan/convergio-sdk/commit/2132c9b86703ae05f5165b3d1c30a3d1ca902414))
* add Agentic Manifesto to README + AGENTS.md universal rules ([c908e9d](https://github.com/Roberdan/convergio-sdk/commit/c908e9dbffc84078fe29391802aebcce5470a5ab))

## [0.1.1](https://github.com/Roberdan/convergio-sdk/compare/convergio-sdk-v0.1.0...convergio-sdk-v0.1.1) (2026-04-11)


### Features

* add telemetry, db, and security crates to SDK (Phase 2) ([dfe6bfa](https://github.com/Roberdan/convergio-sdk/commit/dfe6bfa6ee884b346304a4364210a0599ce41e95))
* add telemetry, db, security crates (Phase 2) ([c48c6d4](https://github.com/Roberdan/convergio-sdk/commit/c48c6d4199f77b7eadb372893b3ec85341a89979))
* initial SDK with convergio-types extracted from monorepo ([d560e6f](https://github.com/Roberdan/convergio-sdk/commit/d560e6f3088d9c9ac58934d22499a9cab0626865))


### Bug Fixes

* **ci:** explicit crate version for release-please + dependabot ignore rules ([770dc29](https://github.com/Roberdan/convergio-sdk/commit/770dc292a564d14baa812a81299b04a78c00761c))
* **ci:** fix release-please + optimize CI pipeline ([89c9881](https://github.com/Roberdan/convergio-sdk/commit/89c9881555b49fc050d620f8a86a94a8604b1975))
* **ci:** fix release-please config + optimize CI pipeline ([99644df](https://github.com/Roberdan/convergio-sdk/commit/99644dfba13b6449389964d3ed68cd9a762c1814))
* **ci:** include Cargo.lock for security audit ([1175c1c](https://github.com/Roberdan/convergio-sdk/commit/1175c1cc6456b8eab3b4fd085c29b06ee5716814))
* **ci:** point release-please to crate path, not workspace root ([6ef96dc](https://github.com/Roberdan/convergio-sdk/commit/6ef96dc5129f5da9f241cdcfd09fe736d19da48f))
* **ci:** replace rustsec/audit-check with cargo-audit directly ([d0b62a7](https://github.com/Roberdan/convergio-sdk/commit/d0b62a7520bccaec3684098a1be29497da91345c))
* **ci:** use simple release-type for workspace + correct manifest path ([b81b41d](https://github.com/Roberdan/convergio-sdk/commit/b81b41d5bebc5f475add1d235bd451fbfd0465cf))
* **ci:** use Swatinem/rust-cache + fix release-please for workspace ([17d3962](https://github.com/Roberdan/convergio-sdk/commit/17d39622f9ca97a97fb332a2ea15b8ffd21c9fac))

## [0.1.0] — 2026-04-11

### Added
- Initial extraction of `convergio-types` from monorepo
- Extension trait, Manifest, DomainEvent, ApiError, Config
- CI pipeline with fmt, clippy, test, security audit
- Dependabot for automated dependency updates
