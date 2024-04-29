build:
    cargo build

migrate-status:
    sea migrate status

migrate-up:
    sea migrate up

generate-entity:
    sea generate entity --with-serde both --output-dir model/src/entity/

pgcli:
    pgcli $DATABASE_URL

install-dioxus-cli:
    cargo install dioxus-cli

install-sea-orm-cli:
    cargo install sea-orm-cli@=1.0.0-rc.3