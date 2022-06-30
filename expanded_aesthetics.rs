#![feature(prelude_import)]
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    non_upper_case_globals
)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use pretty_assertions::assert_eq;
use rstest::rstest;
use rstest::fixture;
use rstest_reuse::template;
use rstest_reuse::apply;
use rstest_reuse::self;
#[cfg(test)]
mod mod_ansi {
    use super::rstest;
    use super::fixture;
    use super::assert_eq;
    use super::template;
    use super::apply;
    use super::rstest_reuse;
    /// this is what should look like when you convert a string into ansi codes with
    /// red, green, blue ..., functions
    pub fn generate_ansi(content: &str) -> String {
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["\u{1b}[31m", "\u{1b}[0m"],
                &[::core::fmt::ArgumentV1::new_display(&content)],
            ));
            res
        }
    }
    use core_dev::aesthetics::ansi::red;
    #[cfg(test)]
    fn test_red(input_content: &str) {
        let preprocessed_expected_result = generate_ansi(input_content);
        let result = red(input_content);
        {
            {
                match (&(result), &(preprocessed_expected_result)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                                &["assertion failed: `(left == right)`", "", "\n\n", "\n"],
                                &match (
                                    &"",
                                    &::core::fmt::Arguments::new_v1(&[], &[]),
                                    &::pretty_assertions::Comparison::new(left_val, right_val),
                                ) {
                                    args => [
                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                        ::core::fmt::ArgumentV1::new_display(args.2),
                                    ],
                                },
                            ))
                        }
                    }
                }
            };
        };
    }
    #[cfg(test)]
    mod test_red {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_1: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::test_red::case_1"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_1())),
        };
        fn case_1() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("hello")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            test_red(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_2: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::test_red::case_2"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_2())),
        };
        fn case_2() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("there")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            test_red(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_3: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::test_red::case_3"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_3())),
        };
        fn case_3() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("rust")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            test_red(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_4: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::test_red::case_4"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_4())),
        };
        fn case_4() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("is")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            test_red(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_5: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::test_red::case_5"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_5())),
        };
        fn case_5() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("the")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            test_red(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_6: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::test_red::case_6"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_6())),
        };
        fn case_6() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("best")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            test_red(input_content)
        }
    }
    #[cfg(test)]
    fn against_123(input_content: &str) {}
    #[cfg(test)]
    mod against_123 {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_1: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::against_123::case_1"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_1())),
        };
        fn case_1() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("hello")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            against_123(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_2: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::against_123::case_2"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_2())),
        };
        fn case_2() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("there")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            against_123(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_3: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::against_123::case_3"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_3())),
        };
        fn case_3() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("rust")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            against_123(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_4: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::against_123::case_4"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_4())),
        };
        fn case_4() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("is")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            against_123(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_5: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::against_123::case_5"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_5())),
        };
        fn case_5() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("the")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            against_123(input_content)
        }
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const case_6: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mod_ansi::against_123::case_6"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(case_6())),
        };
        fn case_6() {
            let input_content = {
                struct __Wrap<T>(std::marker::PhantomData<T>);
                trait __ViaParseDebug<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParseDebug<'a, T> for &&__Wrap<T>
                where
                    T: std::str::FromStr,
                    T::Err: std::fmt::Debug,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        T::from_str(input).unwrap()
                    }
                }
                trait __ViaParse<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a, T> __ViaParse<'a, T> for &__Wrap<T>
                where
                    T: std::str::FromStr,
                {
                    fn magic_conversion(&self, input: &'a str) -> T {
                        match T::from_str(input) {
                            Ok(v) => v,
                            Err(_) => {
                                {
                                    ::std::rt::panic_fmt(::core::fmt::Arguments::new_v1(
                                        &["Cannot parse \'", "\' to get "],
                                        &match (&input, &"& str") {
                                            args => [
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                            ],
                                        },
                                    ))
                                };
                            }
                        }
                    }
                }
                trait __ViaIdent<'a, T> {
                    fn magic_conversion(&self, input: &'a str) -> T;
                }
                impl<'a> __ViaIdent<'a, &'a str> for &&__Wrap<&'a str> {
                    fn magic_conversion(&self, input: &'a str) -> &'a str {
                        input
                    }
                }
                (&&&__Wrap::<&str>(std::marker::PhantomData)).magic_conversion("best")
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_display(&" TEST START ")],
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: '-',
                            align: ::core::fmt::rt::v1::Alignment::Center,
                            flags: 0u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Is(40usize),
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            };
            against_123(input_content)
        }
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &case_1, &case_2, &case_3, &case_4, &case_5, &case_6, &case_1, &case_2, &case_3, &case_4,
        &case_5, &case_6,
    ])
}
