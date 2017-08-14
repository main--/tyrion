# tyrion

```
$ bash build.sh 
   Compiling libc v0.2.29
   Compiling cfg-if v0.1.2
   Compiling gcc v0.3.51
   Compiling rustc-demangle v0.1.4
   Compiling backtrace-sys v0.1.12
   Compiling backtrace v0.3.2
   Compiling tyrion v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 9.67 secs
   Compiling libc v0.2.29
   Compiling gcc v0.3.51
   Compiling rustc-demangle v0.1.4
   Compiling cfg-if v0.1.2
   Compiling backtrace-sys v0.1.12
   Compiling backtrace v0.3.2
   Compiling tyrion v0.1.0
    Finished release [optimized + debuginfo] target(s) in 13.17 secs
$ ls -al target/debug/tyrion target/release/tyrion{,.dbg}
-rwxr-xr-x 2 tywin users 1192536 Aug 14 15:31 target/debug/tyrion
-rwxr-xr-x 2 tywin users  486712 Aug 14 15:32 target/release/tyrion
-rwxr-xr-x 1 tywin users  415504 Aug 14 15:32 target/release/tyrion.dbg
$
```

## Symbols inside the binary

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/tyrion`
Hello, world!
panic occurred: "test panic"
stack backtrace:
   0:       0x2bbfcf3b04 - backtrace::backtrace::libunwind::trace
                        at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.2/src/backtrace/libunwind.rs:53
                         - backtrace::backtrace::trace<closure>
                        at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.2/src/backtrace/mod.rs:42
   1:       0x2bbfcf3eaf - backtrace::capture::{{impl}}::new
                        at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.2/src/capture.rs:64
   2:       0x2bbfcebbba - tyrion::main::{{closure}}
                        at src/main.rs:41
   3:       0x2bbfd10d61 - std::panicking::rust_panic_with_hook::h087de63cca18ed60
   4:       0x2bbfce7415 - std::panicking::begin_panic<&str>
                        at /build/rust/src/rustc-1.19.0-src/src/libstd/panicking.rs:511
   5:       0x2bbfceb710 - tyrion::test
                        at src/main.rs:12
   6:       0x2bbfceb782 - tyrion::main
                        at src/main.rs:81
   7:       0x2bbfd414eb - __rust_maybe_catch_panic
Aborted (core dumped)
```

## Symbols in `.dbg` file

```
$ cargo run --release
    Finished release [optimized + debuginfo] target(s) in 0.0 secs
     Running `target/release/tyrion`
Hello, world!
panic occurred: "test panic"
stack backtrace (symbols from symbol file):
0x0000000000007e9e: backtrace::backtrace::libunwind::trace at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.2/src/backtrace/libunwind.rs:53
 (inlined by) backtrace::backtrace::trace<closure> at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.2/src/backtrace/mod.rs:42
0x0000000000007fe3: backtrace::capture::{{impl}}::new at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.2/src/capture.rs:64
0x0000000000006ec5: tyrion::main::{{closure}} at /overtime/git/tyrion/src/main.rs:41
0x0000000000020381: ?? ??:0
0x0000000000006b54: std::panicking::begin_panic<&str> at /build/rust/src/rustc-1.19.0-src/src/libstd/panicking.rs:511
0x0000000000006d84: tyrion::test at /overtime/git/tyrion/src/main.rs:12
 (inlined by) tyrion::main at /overtime/git/tyrion/src/main.rs:81
0x0000000000050b0b: malloc_usable_size at ??:?
Aborted (core dumped)
```

## No symbols

```
$ ./tyrion-no-symbols
Hello, world!
panic occurred: "test panic"
stack backtrace (no symbols found):
0x7e9e 0x7fe3 0x6ec5 0x20381 0x6b54 0x6d84 0x50b0b 
Aborted (core dumped)
```

## Symbolicate

```
$ addr2line -Cipf -e target/release/tyrion -a 0x7e9e 0x7fe3 0x6ec5 0x20381 0x6b54 0x6d84 0x50b0b
0x0000000000007e9e: backtrace::backtrace::libunwind::trace at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.2/src/backtrace/libunwind.rs:53
 (inlined by) backtrace::backtrace::trace<closure> at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.2/src/backtrace/mod.rs:42
0x0000000000007fe3: backtrace::capture::{{impl}}::new at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.2/src/capture.rs:64
0x0000000000006ec5: tyrion::main::{{closure}} at /overtime/git/tyrion/src/main.rs:41
0x0000000000020381: ?? ??:0
0x0000000000006b54: std::panicking::begin_panic<&str> at /build/rust/src/rustc-1.19.0-src/src/libstd/panicking.rs:511
0x0000000000006d84: tyrion::test at /overtime/git/tyrion/src/main.rs:12
 (inlined by) tyrion::main at /overtime/git/tyrion/src/main.rs:81
0x0000000000050b0b: malloc_usable_size at ??:?
```
