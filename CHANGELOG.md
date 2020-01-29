# Changelog

All notable changes to this project will be documented in this file.

This file's format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Added
### Changed
### Fixed

## [0.12.1] - 2020-01-29
### Fixed
* Fix compiling on macOS

## [0.12.0] - 2018-11-30
### Changed
* Upgraded cassandra-cpp-driver to 2.10.0 to get Datastax's fix for CPP-499

* Upgraded to version 2.8.0 of the C++ driver (was previously 2.4.3).  See that project's [CHANGELOG](https://github.com/datastax/cpp-driver/blob/master/CHANGELOG.md) for more details.  We noticed the following breaking changes:
  * `cass_error_result_actual` becomes `cass_error_result_responses_received`
  * `cass_error_result_required` becomes `cass_error_result_responses_required`
  * `CASS_WRITE_TYPE_UKNOWN` becomes `CASS_WRITE_TYPE_UNKNOWN`

## [0.11.0] - 2017-09-11
### Changed
- Regenerate using latest bindgen. Some type incompatibilities are possible, so bumping version as a precaution.

## [0.10.0] - 2017-08-02
### Changed
- Remove extraneous dependencies (e.g., log).
- Move examples to be proper examples.
- Remove use of error-chain; this is inappropriate for a -sys crate.
- Fix some warnings.
- Correct names of `cass_error_result_received`/`_required`.

- Historical note regarding version 0.9.0

At one point some development was done on preparing a version 0.9.0
using a newer version of bindgen, but the work was not completed or released. See the
[version-0.9](https://github.com/Metaswitch/cassandra-sys-rs/tree/version-0.9) branch
to follow that development. In due course this may be merged to master, but for the moment we recommend you use the
released versions.

## [0.8.8] - 2017-06-29
### Changed
- Fork crate.
- Remove unused method `cass_cluster_set_queue_size_log`.

## [0.8.7] - 2016-12-15
### Changed
- (Pre-fork version.)

[Unreleased]: https://github.com/Metaswitch/cassandra-sys-rs/compare/0.12.1...HEAD
[0.12.1]: https://github.com/Metaswitch/cassandra-sys-rs/compare/0.12.0...0.12.1
[0.12.0]: https://github.com/Metaswitch/cassandra-sys-rs/compare/0.11.0...0.12.0
[0.11.0]: https://github.com/Metaswitch/cassandra-sys-rs/compare/0.10.0...0.11.0
[0.10.0]: https://github.com/Metaswitch/cassandra-sys-rs/compare/0.8.8...0.10.0
[0.8.8]: https://github.com/Metaswitch/cassandra-sys-rs/compare/0.8.7...0.8.8
[0.8.7]: https://github.com/Metaswitch/cassandra-sys-rs/tree/0.8.7
