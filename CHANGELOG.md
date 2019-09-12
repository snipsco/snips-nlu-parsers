# Changelog
All notable changes to this project will be documented in this file.

## [0.4.2]
### Added
- Add support for datetime subtypes in French [#42](https://github.com/snipsco/snips-nlu-parsers/pull/42)

### Fixed
- Bump `rustling-ontology` to `0.19.2` to fix issue with chrono [#43](https://github.com/snipsco/snips-nlu-parsers/pull/43)

## [0.4.1] - 2019-09-02
### Changed
- Bump `snips-nlu-ontology` to `0.67.1` [#39](https://github.com/snipsco/snips-nlu-parsers/pull/39)
- Bump `rustling-ontology` to `0.19.1` [#40](https://github.com/snipsco/snips-nlu-parsers/pull/40)

## [0.4.0] - 2019-08-28
### Added
- Add `alternative_resolved_values` attribute to `GazetteerEntityMatch` [#36](https://github.com/snipsco/snips-nlu-parsers/pull/36)
- Add `max_alternative_resolved_values` parameter to main entity extraction APIs [#36](https://github.com/snipsco/snips-nlu-parsers/pull/36)

## [0.3.1] - 2019-07-23
### Added
- Add Builtin Entity support per Language [#29](https://github.com/snipsco/snips-nlu-parsers/pull/29)
- Bump `gazetteer-entity-parser` to `0.7.1` in order to handle license files [#28](https://github.com/snipsco/snips-nlu-parsers/pull/28)

## [0.3.0] - 2019-07-12
### Changed
- Bump `snips-nlu-ontology` to `0.65.0` [#26](https://github.com/snipsco/snips-nlu-parsers/pull/26)
- Bump `rustling-ontology` to `0.19.0`  [#26](https://github.com/snipsco/snips-nlu-parsers/pull/26)

## [0.2.3] - 2019-07-10
### Changed
- Bump `snips-nlu-ontology` to `0.64.8`

## [0.2.2] - 2019-06-18
### Added
- Gazetteer entity extension API [#18](https://github.com/snipsco/snips-nlu-parsers/pull/18))

### Changed
- Bumped Rustling to `0.18.1` [#20](https://github.com/snipsco/snips-nlu-parsers/pull/20)
- Extended Python dependencies version upper bound [#19](https://github.com/snipsco/snips-nlu-parsers/pull/19)

## [0.2.1] - 2019-04-08
### Added
- Expose complete and by language builtin entities json configuration retrieval API [#13](https://github.com/snipsco/snips-nlu-parsers/pull/13)

### Changed
- Bump `snips-nlu-ontology` to `0.64.6` [#16](https://github.com/snipsco/snips-nlu-parsers/pull/16)

## [0.2.0] - 2019-02-27
### Changed
- bump `snips-nlu-ontology` to `0.64.4`
- bump `rustling-ontology` to `0.18.0`
- bump `snips-nlu-utils` to `0.8.0`

## [0.1.2] - 2019-01-29
### Changed
- bump `snips-nlu-ontology` to `0.63.1`

## [0.1.1] - 2019-01-28
### Changed
- bump `snips-nlu-ontology` to `0.63.0`
- re-export `gazetteer-entity-parser` crate

[0.4.2]: https://github.com/snipsco/snips-nlu-parsers/compare/0.4.1...0.4.2
[0.4.1]: https://github.com/snipsco/snips-nlu-parsers/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/snipsco/snips-nlu-parsers/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/snipsco/snips-nlu-parsers/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/snipsco/snips-nlu-parsers/compare/0.2.3...0.3.0
[0.2.3]: https://github.com/snipsco/snips-nlu-parsers/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/snipsco/snips-nlu-parsers/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/snipsco/snips-nlu-parsers/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/snipsco/snips-nlu-parsers/compare/0.1.2...0.2.0
[0.1.2]: https://github.com/snipsco/snips-nlu-parsers/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/snipsco/snips-nlu-parsers/compare/0.1.0...0.1.1
