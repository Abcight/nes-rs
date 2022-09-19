export CARGO_INCREMENTAL=0;
export RUSTFLAGS='-Cinstrument-coverage';
export LLVM_PROFILE_FILE='nes-rs-%p-%m.profraw';
cargo +nightly test;

grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html