build:
    cargo build

generate-entity:
    sea generate entity --output-dir model/src/entity/

pgcli:
    pgcli $DATABASE_URL

install-dioxus-cli:
    cargo install dioxus-cli

install-sea-orm-cli:
    cargo install sea-orm-cli@=1.0.0-rc.3