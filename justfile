build:
    cargo build

migrate-status:
    sea migrate status

migrate-up:
    sea migrate up

generate-entities:
    sea generate entity --with-serde both --seaography --output-dir model/src/entities/

generate-graphql:
    sea generate entity --seaography --output-dir graphql/src/entities/
    seaography-cli graphql graphql/src/entities $DATABASE_URL kumou_graphql

pgcli:
    pgcli $DATABASE_URL

install-dioxus-cli:
    cargo install dioxus-cli

install-sea-orm-cli:
    cargo install sea-orm-cli@=1.0.0-rc.3

install-seaography-cli:
    cargo install seaography-cli@=1.0.0-rc.2