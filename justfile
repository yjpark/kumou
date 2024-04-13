build:
    cargo build

build-doc:
    cargo doc --no-deps

serve-doc:
    simple-http-server -p 8001 --index --nocache target/doc

install-dioxus-cli:
    cargo install dioxus-cli


