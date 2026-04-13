# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.10](https://github.com/Roberdan/convergio-sdk/compare/v0.1.9...v0.1.10) (2026-04-13)


### Features

* prepare SDK crates for crates.io publishing ([78fa3ea](https://github.com/Roberdan/convergio-sdk/commit/78fa3eafa393f1b732311ee4b4214c42c06f40bb))
* prepare SDK crates for crates.io publishing ([7cea0a5](https://github.com/Roberdan/convergio-sdk/commit/7cea0a5c9b79184f5c7f5611b305ac0cc63f8166))

## [0.1.9](https://github.com/Roberdan/convergio-sdk/compare/v0.1.8...v0.1.9) (2026-04-12)


### Documentation

* add .env.example with required environment variables ([#21](https://github.com/Roberdan/convergio-sdk/issues/21)) ([e919cbc](https://github.com/Roberdan/convergio-sdk/commit/e919cbc98ed647bc5a4d9821c8fe1d6946b286dc))

## [0.1.8](https://github.com/Roberdan/convergio-sdk/compare/v0.1.7...v0.1.8) (2026-04-12)


### Bug Fixes

* **security:** comprehensive SDK security audit fixes ([#19](https://github.com/Roberdan/convergio-sdk/issues/19)) ([63d93df](https://github.com/Roberdan/convergio-sdk/commit/63d93dff7c57c0e71827084fef0ad15517fead01))

## [0.1.7](https://github.com/Roberdan/convergio-sdk/compare/v0.1.6...v0.1.7) (2026-04-12)


### Features

* add telemetry, db, and security crates to SDK (Phase 2) ([dfe6bfa](https://github.com/Roberdan/convergio-sdk/commit/dfe6bfa6ee884b346304a4364210a0599ce41e95))
* add telemetry, db, security crates (Phase 2) ([c48c6d4](https://github.com/Roberdan/convergio-sdk/commit/c48c6d4199f77b7eadb372893b3ec85341a89979))
* complete Fase A — reusable workflows, scorecard, SBOM, ADR-003 ([40eb7eb](https://github.com/Roberdan/convergio-sdk/commit/40eb7ebf0b6d9d14868600586dbda0b4a9f0a760))
* complete Fase A — reusable workflows, scorecard, SBOM, CONSTITUTION, ADR-003 ([6390704](https://github.com/Roberdan/convergio-sdk/commit/63907042b2957757b1f6dfe82de78a0ac203504b))
* initial SDK with convergio-types extracted from monorepo ([d560e6f](https://github.com/Roberdan/convergio-sdk/commit/d560e6f3088d9c9ac58934d22499a9cab0626865))
* quality framework — adversarial tests, cargo-deny, ADR, agent-first docs ([bb7118d](https://github.com/Roberdan/convergio-sdk/commit/bb7118d53afa1233339675b06f0468cf505dbd89))
* quality framework — adversarial tests, cargo-deny, ADR, agent-first docs ([71e8e5c](https://github.com/Roberdan/convergio-sdk/commit/71e8e5c66c7ef290dddef8f3a5d15ee4e57d9bfa))


### Bug Fixes

* add MPL-2.0 to license whitelist (option-ext dep) ([4d5a71c](https://github.com/Roberdan/convergio-sdk/commit/4d5a71c86cc6d0270ae33426417790043b592d7a))
* allow workspace wildcard deps in cargo-deny ([495971f](https://github.com/Roberdan/convergio-sdk/commit/495971f84af5d7b357734717d8e5f243efac1482))
* **ci:** explicit crate version for release-please + dependabot ignore rules ([770dc29](https://github.com/Roberdan/convergio-sdk/commit/770dc292a564d14baa812a81299b04a78c00761c))
* **ci:** fix release-please + optimize CI pipeline ([89c9881](https://github.com/Roberdan/convergio-sdk/commit/89c9881555b49fc050d620f8a86a94a8604b1975))
* **ci:** fix release-please config + optimize CI pipeline ([99644df](https://github.com/Roberdan/convergio-sdk/commit/99644dfba13b6449389964d3ed68cd9a762c1814))
* **ci:** include Cargo.lock for security audit ([1175c1c](https://github.com/Roberdan/convergio-sdk/commit/1175c1cc6456b8eab3b4fd085c29b06ee5716814))
* **ci:** point release-please to crate path, not workspace root ([6ef96dc](https://github.com/Roberdan/convergio-sdk/commit/6ef96dc5129f5da9f241cdcfd09fe736d19da48f))
* **ci:** replace rustsec/audit-check with cargo-audit directly ([d0b62a7](https://github.com/Roberdan/convergio-sdk/commit/d0b62a7520bccaec3684098a1be29497da91345c))
* **ci:** sync Cargo.lock + auto-update for release PRs ([e0bc839](https://github.com/Roberdan/convergio-sdk/commit/e0bc839e283d00324a3e69442ceb2e5a3ea5c737))
* **ci:** sync Cargo.lock + auto-update for release-please PRs ([fdb4e67](https://github.com/Roberdan/convergio-sdk/commit/fdb4e676b22fd590f441994f5e89ac2f2ef81481))
* **ci:** use PAT for fully automatic releases ([4c5bd87](https://github.com/Roberdan/convergio-sdk/commit/4c5bd877378dd328a30bc242e0597cea0ec1c1b4))
* **ci:** use PAT for release-please and lockfile-update workflows ([b4ca710](https://github.com/Roberdan/convergio-sdk/commit/b4ca7109dbe20dad3ea72973fa13e50b9b062097))
* **ci:** use simple release-type for workspace + correct manifest path ([b81b41d](https://github.com/Roberdan/convergio-sdk/commit/b81b41d5bebc5f475add1d235bd451fbfd0465cf))
* **ci:** use Swatinem/rust-cache + fix release-please for workspace ([17d3962](https://github.com/Roberdan/convergio-sdk/commit/17d39622f9ca97a97fb332a2ea15b8ffd21c9fac))
* keep manifesto full text in README, add canonical source link ([4b17931](https://github.com/Roberdan/convergio-sdk/commit/4b17931eb52da242710e1f808a67b1a2d4983753))
* standardize release tags to v0.x.y format ([07ddf4a](https://github.com/Roberdan/convergio-sdk/commit/07ddf4a277ca75bd865e146156c4edf1c00b8eed))
* update deny.toml to v2 format ([a9c793f](https://github.com/Roberdan/convergio-sdk/commit/a9c793f260c8baaf2ba4bb080d5d368c09a14ae7))


### Documentation

* add Agentic Manifesto + AGENTS.md ([2132c9b](https://github.com/Roberdan/convergio-sdk/commit/2132c9b86703ae05f5165b3d1c30a3d1ca902414))
* add Agentic Manifesto to README + AGENTS.md universal rules ([c908e9d](https://github.com/Roberdan/convergio-sdk/commit/c908e9dbffc84078fe29391802aebcce5470a5ab))

## [0.1.6](https://github.com/Roberdan/convergio-sdk/compare/convergio-sdk-v0.1.5...convergio-sdk-v0.1.6) (2026-04-11)


### Features

* complete Fase A — reusable workflows, scorecard, SBOM, ADR-003 ([40eb7eb](https://github.com/Roberdan/convergio-sdk/commit/40eb7ebf0b6d9d14868600586dbda0b4a9f0a760))
* complete Fase A — reusable workflows, scorecard, SBOM, CONSTITUTION, ADR-003 ([6390704](https://github.com/Roberdan/convergio-sdk/commit/63907042b2957757b1f6dfe82de78a0ac203504b))


### Bug Fixes

* keep manifesto full text in README, add canonical source link ([4b17931](https://github.com/Roberdan/convergio-sdk/commit/4b17931eb52da242710e1f808a67b1a2d4983753))

## [0.1.5](https://github.com/Roberdan/convergio-sdk/compare/convergio-sdk-v0.1.4...convergio-sdk-v0.1.5) (2026-04-11)


### Features

* quality framework — adversarial tests, cargo-deny, ADR, agent-first docs ([bb7118d](https://github.com/Roberdan/convergio-sdk/commit/bb7118d53afa1233339675b06f0468cf505dbd89))
* quality framework — adversarial tests, cargo-deny, ADR, agent-first docs ([71e8e5c](https://github.com/Roberdan/convergio-sdk/commit/71e8e5c66c7ef290dddef8f3a5d15ee4e57d9bfa))


### Bug Fixes

* add MPL-2.0 to license whitelist (option-ext dep) ([4d5a71c](https://github.com/Roberdan/convergio-sdk/commit/4d5a71c86cc6d0270ae33426417790043b592d7a))
* allow workspace wildcard deps in cargo-deny ([495971f](https://github.com/Roberdan/convergio-sdk/commit/495971f84af5d7b357734717d8e5f243efac1482))
* update deny.toml to v2 format ([a9c793f](https://github.com/Roberdan/convergio-sdk/commit/a9c793f260c8baaf2ba4bb080d5d368c09a14ae7))

## [0.1.4](https://github.com/Roberdan/convergio-sdk/compare/convergio-sdk-v0.1.3...convergio-sdk-v0.1.4) (2026-04-11)


### Bug Fixes

* **ci:** use PAT for fully automatic releases ([4c5bd87](https://github.com/Roberdan/convergio-sdk/commit/4c5bd877378dd328a30bc242e0597cea0ec1c1b4))
* **ci:** use PAT for release-please and lockfile-update workflows ([b4ca710](https://github.com/Roberdan/convergio-sdk/commit/b4ca7109dbe20dad3ea72973fa13e50b9b062097))

## [0.1.3](https://github.com/Roberdan/convergio-sdk/compare/convergio-sdk-v0.1.2...convergio-sdk-v0.1.3) (2026-04-11)


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
* **ci:** sync Cargo.lock + auto-update for release PRs ([e0bc839](https://github.com/Roberdan/convergio-sdk/commit/e0bc839e283d00324a3e69442ceb2e5a3ea5c737))
* **ci:** sync Cargo.lock + auto-update for release-please PRs ([fdb4e67](https://github.com/Roberdan/convergio-sdk/commit/fdb4e676b22fd590f441994f5e89ac2f2ef81481))
* **ci:** use simple release-type for workspace + correct manifest path ([b81b41d](https://github.com/Roberdan/convergio-sdk/commit/b81b41d5bebc5f475add1d235bd451fbfd0465cf))
* **ci:** use Swatinem/rust-cache + fix release-please for workspace ([17d3962](https://github.com/Roberdan/convergio-sdk/commit/17d39622f9ca97a97fb332a2ea15b8ffd21c9fac))


### Documentation

* add Agentic Manifesto + AGENTS.md ([2132c9b](https://github.com/Roberdan/convergio-sdk/commit/2132c9b86703ae05f5165b3d1c30a3d1ca902414))
* add Agentic Manifesto to README + AGENTS.md universal rules ([c908e9d](https://github.com/Roberdan/convergio-sdk/commit/c908e9dbffc84078fe29391802aebcce5470a5ab))

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
