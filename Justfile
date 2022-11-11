build-release: 
    rustup target add x86_64-unknown-linux-musl
    cargo build --release --target x86_64-unknown-linux-musl
    mkdir -p bin
    cp target/x86_64-unknown-linux-musl/release/project-issue-state-sync bin/
    upx --best --lzma bin/project-issue-state-sync