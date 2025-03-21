# Changelog

## [0.2.0](https://github.com/burdockcascade/bytevm/compare/v0.1.0...v0.2.0) (2025-03-21)


### Features

* add VmOptions to VM struct for configurable options ([c67a6c4](https://github.com/burdockcascade/bytevm/commit/c67a6c41c4f3fd0c9c6061d5438db48482ff76ca))
* implement Display trait for Variant type ([bd849fd](https://github.com/burdockcascade/bytevm/commit/bd849fd742a1fc6ce55dcce81045bb23b764d206))
* introduce VmOptions ([0bf855d](https://github.com/burdockcascade/bytevm/commit/0bf855db98692db5c15acc6028f5f0fa335c398a))


### Bug Fixes

* rename Equals instruction to Equal for consistency ([ffeba6b](https://github.com/burdockcascade/bytevm/commit/ffeba6be78bfb5907ab47e20b18f22fb0d5346e8))
* rename Greater instruction to GreaterThan for consistency ([2b62e5f](https://github.com/burdockcascade/bytevm/commit/2b62e5f59e0edc5de9dcb1d2e77e011a745e0f32))
* update Panic instruction to remove String parameter and handle message extraction ([ac5c422](https://github.com/burdockcascade/bytevm/commit/ac5c422afcf34376c4d32822680a65de958f7367))

## 0.1.0 (2025-03-10)


### Features

* add initial implementation of a simple bytecode virtual machine with CI configuration ([bd260ca](https://github.com/burdockcascade/bytevm/commit/bd260ca045a82059cc560f3614455cf942b199a5))
