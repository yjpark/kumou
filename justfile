build:
    cargo build

generate-entities:
    sea generate entity --with-serde both --ignore-tables refinery_schema_history --output-dir model/src/entities/

generate-graphql:
    sea generate entity --seaography --ignore-tables refinery_schema_history --output-dir graphql/src/entities/
    seaography-cli graphql graphql/src/entities $DATABASE_URL kumou_graphql

install-dioxus-cli:
    cargo install dioxus-cli

install-sea-orm-cli:
    cargo install sea-orm-cli@=1.0.0-rc.3

install-seaography-cli:
    cargo install seaography-cli@=1.0.0-rc.2