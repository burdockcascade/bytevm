# Changelog

## [0.3.0](https://github.com/burdockcascade/bytevm/compare/v0.2.0...v0.3.0) (2025-03-25)


### Features

* streamline function call handling ([2277c49](https://github.com/burdockcascade/bytevm/commit/2277c494277010d19ef5241bb8658245f18aab00))
* unify push instructions to use Variant enum for improved type handling ([7f814ab](https://github.com/burdockcascade/bytevm/commit/7f814ab0b7b6913e3cdce117eb4f5fbde23afed1))
* update function calls to use Variant::Identifier for consistency ([7e1b340](https://github.com/burdockcascade/bytevm/commit/7e1b34043003f76f4565aca0c30b6a50e1b7c25b))


### Bug Fixes

* correct capacity allocation for function call arguments ([3a927ec](https://github.com/burdockcascade/bytevm/commit/3a927ec4e75a6b079409307f95a67b06edef39ca))

## [0.2.0](https://github.com/burdockcascade/bytevm/compare/v0.1.0...v0.2.0) (2025-03-23)


### Features

* add push_local method to stack and improve function call argument handling ([9d3cf7f](https://github.com/burdockcascade/bytevm/commit/9d3cf7fdd26381b645175dc4ce02c856ee1c7f42))
* add VmOptions to VM struct for configurable options ([c67a6c4](https://github.com/burdockcascade/bytevm/commit/c67a6c41c4f3fd0c9c6061d5438db48482ff76ca))
* implement Display trait for Variant type ([bd849fd](https://github.com/burdockcascade/bytevm/commit/bd849fd742a1fc6ce55dcce81045bb23b764d206))
* initialize default symbols with user-defined main function and update tests to use default program state ([b0eedcf](https://github.com/burdockcascade/bytevm/commit/b0eedcfac85606dacbff5ebf4bbe793266d16f43))
* introduce VmOptions ([0bf855d](https://github.com/burdockcascade/bytevm/commit/0bf855db98692db5c15acc6028f5f0fa335c398a))
* update FunctionCall instruction to use u8 for argument count and add PushIdentifier instruction ([ad77bfb](https://github.com/burdockcascade/bytevm/commit/ad77bfb8e0a3dc0400e2fc908e8ae6d92bea849b))


### Bug Fixes

* change run_time type from f64 to u128 for improved precision ([490c5a2](https://github.com/burdockcascade/bytevm/commit/490c5a2ffcfc068c520b59493062ef4abab5bec9))
* improve entry point resolution by checking for user-defined main function ([a52a490](https://github.com/burdockcascade/bytevm/commit/a52a49066a52018973394de7c51799c91a563e56))
* rename Equals instruction to Equal for consistency ([ffeba6b](https://github.com/burdockcascade/bytevm/commit/ffeba6be78bfb5907ab47e20b18f22fb0d5346e8))
* rename Greater instruction to GreaterThan for consistency ([2b62e5f](https://github.com/burdockcascade/bytevm/commit/2b62e5f59e0edc5de9dcb1d2e77e011a745e0f32))
* update Panic instruction to remove String parameter and handle message extraction ([ac5c422](https://github.com/burdockcascade/bytevm/commit/ac5c422afcf34376c4d32822680a65de958f7367))

## 0.1.0 (2025-03-10)


### Features

* add initial implementation of a simple bytecode virtual machine with CI configuration ([bd260ca](https://github.com/burdockcascade/bytevm/commit/bd260ca045a82059cc560f3614455cf942b199a5))
