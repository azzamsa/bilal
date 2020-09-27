VERSION=`awk '/version = "[0-9]\.[0-9]\.[0-9]"/' Cargo.toml | awk -F\" '{print $2}'`

# GNU/Linux
cargo build --release
strip target/release/bilal
mv target/release/bilal target/release/bilal-$VERSION

# Windows PC
cargo build --release --target=x86_64-pc-windows-gnu
mv target/x86_64-pc-windows-gnu/release/bilal.exe target/x86_64-pc-windows-gnu/release/bilal-$VERSION.exe
