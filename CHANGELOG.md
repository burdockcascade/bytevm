# Changelog

## [0.7.0](https://github.com/burdockcascade/bytevm/compare/v0.6.1...v0.7.0) (2025-06-08)


### Features

* add end_function instruction and corresponding test case ([4762fc4](https://github.com/burdockcascade/bytevm/commit/4762fc4299d0eb4722d5bed6e1ca8dba5a61b855))
* Refactored instruction execution for increased performance ([#11](https://github.com/burdockcascade/bytevm/issues/11)) ([447e286](https://github.com/burdockcascade/bytevm/commit/447e286805154c1a388cb4ed22b0750284d19bf3))

## [0.6.1](https://github.com/burdockcascade/bytevm/compare/v0.6.0...v0.6.1) (2025-05-29)


### Bug Fixes

* fix resolving function name to index ([#8](https://github.com/burdockcascade/bytevm/issues/8)) ([6d6adfd](https://github.com/burdockcascade/bytevm/commit/6d6adfdfc3db04ac2a931d0fba2fa17e4d163a3c))

## [0.6.0](https://github.com/burdockcascade/bytevm/compare/v0.5.0...v0.6.0) (2025-04-19)


### Features

* add push_index method and update array operations to use Index variant ([0688371](https://github.com/burdockcascade/bytevm/commit/06883718255a82f43dc9e0131669d263a0719005))
* refactored to improve performance ([e83d438](https://github.com/burdockcascade/bytevm/commit/e83d43814f7d2231a5e6bc791d0901e1ea912b26))

## [0.5.0](https://github.com/burdockcascade/bytevm/compare/v0.4.1...v0.5.0) (2025-04-13)


### Features

* add add_symbol method to insert entries into the symbol table ([15c3c65](https://github.com/burdockcascade/bytevm/commit/15c3c655b60e3b32f908e89d14b9a96966864c70))
* Add GlobalReference and FunctionPointer for function calls ([57c3a65](https://github.com/burdockcascade/bytevm/commit/57c3a65199061d15d53f7e4d6796189cd5d7e19f))
* add support for addition of Array and Dictionary variants ([cf98251](https://github.com/burdockcascade/bytevm/commit/cf982517b9d62a655d40a52bc03b2fe20aa1e086))
* implement equality checks for Array and Dictionary variants ([c47171e](https://github.com/burdockcascade/bytevm/commit/c47171e61bb2710f40c501939251be2eef3e94d9))
* introduce BlockEncoder for building instruction blocks ([f44902c](https://github.com/burdockcascade/bytevm/commit/f44902c702424ef379d9451c918cfa240bf20ac9))
* introduce ProgramBuilder for improved program construction ([9471f37](https://github.com/burdockcascade/bytevm/commit/9471f375a847c34b6fddf02b7250b97fe8ee80cd))
* introduct prelude ([ec76ad2](https://github.com/burdockcascade/bytevm/commit/ec76ad27f61f5c0a1a1828906e1b636935b93574))


### Bug Fixes

* change run_time type to Duration in VmExecutionResult ([578ee41](https://github.com/burdockcascade/bytevm/commit/578ee41599bdadb3d37d93207fd0b67677e15931))
* correct syntax for config-file in release.yml ([4d991d6](https://github.com/burdockcascade/bytevm/commit/4d991d62feff141acbc1eda79cfaee5374ca488b))
* update function arity in test cases for variable operations ([ac559f2](https://github.com/burdockcascade/bytevm/commit/ac559f280d147b74c98c0663750e5e28f0c777e7))

## [0.4.1](https://github.com/burdockcascade/bytevm/compare/v0.4.0...v0.4.1) (2025-04-02)


### Bug Fixes

* handle invalid instruction pointer in runtime loop ([8a31e8e](https://github.com/burdockcascade/bytevm/commit/8a31e8eb9118c415f728b5070dc0aac98c2a0e42))
* handle optional returned result from function ([1642423](https://github.com/burdockcascade/bytevm/commit/1642423f7df555fe0cda7a11df824f3bd8600724))

## [0.4.0](https://github.com/burdockcascade/bytevm/compare/v0.3.0...v0.4.0) (2025-04-02)


### Features

* add benchmarking for while loop using Criterion ([9776e56](https://github.com/burdockcascade/bytevm/commit/9776e567255cb3cea19d0aea64269f49156f8dfd))
* add Fibonacci benchmark using Criterion ([775e959](https://github.com/burdockcascade/bytevm/commit/775e959a220b03cac36cd006cf6efcf20d438de5))
* add GetArrayLength instruction and corresponding tests for array handling ([bb93536](https://github.com/burdockcascade/bytevm/commit/bb935366c21030f580363bc35de94eefb330b997))
* add GetDictionaryKeys instruction and corresponding tests ([ceb8981](https://github.com/burdockcascade/bytevm/commit/ceb89811b4ca6dd4af5f240977caf3d92d7b5822))
* add global table for functions ([3fa61e1](https://github.com/burdockcascade/bytevm/commit/3fa61e19d37af35986facbbc4be2b06315090a8c))
* add Print instruction for output handling ([37c9767](https://github.com/burdockcascade/bytevm/commit/37c976720bc7d437cdae03ff9a4a4dd661d77d7d))
* add tracing for program execution details in runtime ([c70c56d](https://github.com/burdockcascade/bytevm/commit/c70c56d95bf98cb8cfbba56c5695b35c92d5a729))
* enhance program structure with function-based approach for main and user-defined functions ([7c68183](https://github.com/burdockcascade/bytevm/commit/7c681838b0d5571ebe1b135d312ac63f14b2468a))
* enhance tracing in runtime loop for better debugging ([9e98d0e](https://github.com/burdockcascade/bytevm/commit/9e98d0e8a6d4667d51f988cc396074b568223d56))


### Bug Fixes

* handle unexpected cases in function pointer execution ([2d2e939](https://github.com/burdockcascade/bytevm/commit/2d2e939c2f50c5cc1dd43bf3ef9d3ad1adbcc579))
* improve error message for missing dictionary key ([f28658f](https://github.com/burdockcascade/bytevm/commit/f28658ffd6b712999a8f164185671501af8117f2))
* remove redundant semicolon in function call instruction ([fd5c61c](https://github.com/burdockcascade/bytevm/commit/fd5c61c2ef95b827d77afa5eda8b7dc2e46d7661))
* update jump instruction handling to use base address for correct program counter calculation ([51641c3](https://github.com/burdockcascade/bytevm/commit/51641c385a0e0c8839fc38738d7155bebd19e864))
* update string addition handling in Variant to support different types ([4556625](https://github.com/burdockcascade/bytevm/commit/45566250cdb26d752da2fe86074fec27fab89987))

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
