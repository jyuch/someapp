cargo clean
cargo build --release

Copy-Item 'target\release\someapp.exe' 'C:\temp'
