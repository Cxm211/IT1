
# test reported bindings
cat reported_bindings.rs > test/src/main.rs
cd test
cargo test > ../error_message.txt 2>&1

# test generated bindings
cat generated_bindings.rs > test/src/main.rs
cd test
cargo test > ../error_message.txt 2>&1