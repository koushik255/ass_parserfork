# CHANGELOG

All notable changes to this project will be documented in this file.

## **AssParser**[(0.2.2)](https://crates.io/crates/ass_parser/0.2.2) 2025-04-11

### Fixed
- v0.2.2 [b86b471](https://github.com/Aavtic/ass_parser/commit/b86b47174db4486fa1fcff3f852d0cdf60fa0993) (Fix) Fixed issue with UTF-8 Encoded .ass files which contains [BOM](https://en.wikipedia.org/wiki/Byte_order_mark).


## [UNRELEASED] **AssParser** 2025-04-11

### Fixed
- [Compatible with older versions of .ass files](#6)

### Changed
- [33e249](https://github.com/Aavtic/ass_parser/commit/33e249edf588dfc32d3f1116b3be31f0b1c4136c) Since some of the components in .ass files are not found in some old files. It is better to assume that each component may or may not be present in the file.
Thus treating each component as an option.


## New Contributors
- [@meipeter](https://github.com/meipeter) Made his first contribution in #4
- [@Wat86](https://github.com/Wat86) Made his first contribution in #5

## Contributors
Here is the list of contributors to this project
Thank you so much for contributing!

- [@meipeter](https://github.com/meipeter)
- [@Wat86](https://github.com/Wat86)
