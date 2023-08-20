# ocaml-rust-on-apple-silicon-demo

A minimal reproducible example of an OCaml executable that links to some Rust
code that throws, than immediately catches a panic resulting in the process
aborting depending on the value of RUST_BACKTRACE.

To build: `$ dune build bin/main.exe`

```
$ ./_build/default/bin/main.exe
Throwing panic...
thread '<unnamed>' panicked at 'test panic', src/lib.rs:5:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Panic successfully caught: test panic
```

```
$ RUST_BACKTRACE=1 ./_build/default/bin/main.exe
Throwing panic...
thread '<unnamed>' panicked at 'test panic', src/lib.rs:5:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:617:5
   1: core::panicking::panic_fmt
             at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/core/src/panicking.rs:67:14
   2: rust::rust_test_panic::{{closure}}
             at ./_build/default/rust/src/lib.rs:5:9
   3: std::panicking::try::do_call
             at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:524:40
   4: ___rust_try
   5: std::panicking::try
             at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:488:19
   6: std::panic::catch_unwind
             at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panic.rs:142:14
   7: rust_test_panic
             at ./_build/default/rust/src/lib.rs:3:11
   8: _caml_c_call
   9: _camlDune__exe__Main__code_begin
  10: _caml_startup__code_begin
  11: _caml_start_program
zsh: trace trap  RUST_BACKTRACE=1 ./_build/default/bin/main.exe
```

```
$ RUST_BACKTRACE=full ./_build/default/bin/main.exe
Throwing panic...
thread '<unnamed>' panicked at 'test panic', src/lib.rs:5:9
stack backtrace:
   0:        0x10069d418 - std::backtrace_rs::backtrace::libunwind::trace::h049f4907dd8a8d8c
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:        0x10069d418 - std::backtrace_rs::backtrace::trace_unsynchronized::h4ce0b38f7d35ba7f
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x10069d418 - std::sys_common::backtrace::_print_fmt::h88acb99cd148aee7
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/sys_common/backtrace.rs:65:5
   3:        0x10069d418 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h91afbe6669b655d0
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/sys_common/backtrace.rs:44:22
   4:        0x1006ece18 - core::fmt::rt::Argument::fmt::hc5caf41761ffbd03
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/core/src/fmt/rt.rs:138:9
   5:        0x1006ece18 - core::fmt::write::h49acb44dd8866c4a
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/core/src/fmt/mod.rs:1094:21
   6:        0x100692b14 - std::io::Write::write_fmt::h50f18f188ad39de8
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/io/mod.rs:1714:15
   7:        0x10069d270 - std::sys_common::backtrace::_print::h2b23e45f2c356880
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/sys_common/backtrace.rs:47:5
   8:        0x10069d270 - std::sys_common::backtrace::print::h372df14e9dedee87
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/sys_common/backtrace.rs:34:9
   9:        0x10069ff9c - std::panicking::panic_hook_with_disk_dump::{{closure}}::h4d5b79f506dc402d
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:278:22
  10:        0x10069fc84 - std::panicking::panic_hook_with_disk_dump::h8bffab858856fa2f
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:312:9
  11:        0x1006a064c - std::panicking::default_hook::h292fec1fc975a7ae
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:239:5
  12:        0x1006a064c - std::panicking::rust_panic_with_hook::h20cfc1df6210a32b
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:729:13
  13:        0x1006a0518 - std::panicking::begin_panic_handler::{{closure}}::h12a8656a87310740
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:619:13
  14:        0x10069d7f8 - std::sys_common::backtrace::__rust_end_short_backtrace::h8053032019e6ed0d
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/sys_common/backtrace.rs:151:18
  15:        0x1006a02c0 - rust_begin_unwind
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:617:5
  16:        0x10073f248 - core::panicking::panic_fmt::hd63f35461a902427
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/core/src/panicking.rs:67:14
  17:        0x100666e38 - rust::rust_test_panic::{{closure}}::h7ca4e99e5816c362
                               at /Users/mtakeda/dev/ocaml/ocaml-rust-on-apple-silicon-demo/_build/default/rust/src/lib.rs:5:9
  18:        0x1006684a0 - std::panicking::try::do_call::h85666218225f7e3f
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:524:40
  19:        0x10066855c - ___rust_try
  20:        0x100668424 - std::panicking::try::h74e833d9eb29248b
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panicking.rs:488:19
  21:        0x1006678cc - std::panic::catch_unwind::h779f8955ad0fe9f7
                               at /rustc/500647fd8138cc09e87edb08d62f81654fbf6ef8/library/std/src/panic.rs:142:14
  22:        0x1006692b0 - rust_test_panic
                               at /Users/mtakeda/dev/ocaml/ocaml-rust-on-apple-silicon-demo/_build/default/rust/src/lib.rs:3:11
  23:        0x10073a780 - _caml_c_call
  24:        0x1005935f0 - _camlDune__exe__Main__code_begin
  25:        0x100590444 - _caml_startup__code_begin
  26:        0x10073a7f0 - _caml_start_program
zsh: trace trap  RUST_BACKTRACE=full ./_build/default/bin/main.exe
```