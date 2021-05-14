rustup toolchain install stable
FILE="$HOME/.cargo/bin/wasm-pack"
if [ -f "$FILE" ]; then
    echo "The wasm pack binary already exists. No nead to reinstall it."
else
	cargo install wasm-pack

wasm-pack build --target web