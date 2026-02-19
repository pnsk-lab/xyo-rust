cargo build --release

for f in ./examples/*.sb3; do
    echo "--- $f"
    ./target/release/xyo run "$f"
done
