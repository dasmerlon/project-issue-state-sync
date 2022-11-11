build-release: 
    rustup target add x86_64-unknown-linux-musl
    cargo build --release --target x86_64-unknown-linux-musl
    mkdir -p bin
    cp target/x86_64-unknown-linux-musl/release/project-workflow-extensions bin/
    upx --best --lzma bin/project-workflow-extensions