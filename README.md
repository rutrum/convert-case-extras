# Convert Case Extras

Extra patterns, boundaries, cases, and utilities on top of [`convert_case`](https://github.com/rutrum/convert-case).

## Changelog

### convert-case-extras v0.1.0

* `case::TOGGLE`: words start with lower case and remainder are upper case
* `case::ALTERNATING`: words alternate between upper and lower case
* `random` feature:
    * `case::RANDOM`: letters are randomly lower case or upper case
    * `case::PSEUDO_RANDOM`: pairs of letters are randomly lower-upper, or upper-lower
