rustup default beta
cargo new --lib foo
cd foo
echo "mod qux;" > src/lib.rs
echo "mod bar;" > src/qux.rs
mkdir src/qux
touch src/qux/bar.rs
cargo build
cbindgen -o ../generated_binding.h