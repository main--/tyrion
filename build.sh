cargo build --release
objcopy --only-keep-debug target/release/tyrion tyrion.dbg
objcopy --strip-debug --strip-unneeded --add-gnu-debuglink=tyrion.dbg target/release/tyrion tyrion
