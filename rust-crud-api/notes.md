# Notes 

### Cargo.TOML

```
# Implementation of the serialize and deserialize traits

[dependencies]
postgres = "0.19.9"
serde = "1.0.217"
serde_json = "1.0.134"
serde_derive = "1.0.217"
build = "0.0.2"

```
- Search(Google): cargo + dependencies = version
- Serde: implementation of the serialize and deserialize traits


### Dockerfile

```
# build
FROM rust:1.83 as builder

WORKDIR /app

# accept
ARG DBASE_URL

ENV DBASE_URL=$DBASE_URL

COPY . .

RUN cargo build --release

# Production stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust-crud-api .

CMD ["./rust-crud-api"]  

  
```
- Build: rustc --version

- RUN cargo build --release or cargo build
  - cargo build: great for development and debugging.
  - cargo build --release: perfect for when you're ready to ship your code to production.

- FROM debian:
  - bookworm: full stable release of Debian 12 with all standard packages.
  - bookworm-slim: a lighter and more minimalist version of Debian 12, with only the essential packages. (Recomend!)


### Docker-compose

```
//version: '3.9' > No recomend! 

services:
  rustapp:
    container_name: rustapp
    image: nfoj/rustapp:1.0.0
    build:
      context: .
      dockerfile: Dockerfile
      args:
        DBASE_URL: postgres://postgres:postgres@db:5432/postgres
    ports:
      - '8080:8080'
    depends_on:
      - db
  db:
    container_name: db
    image: postgres:12
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: postgres
    ports:
      - '5432:5432'
    volumes:
      - pgdata:/var/lib/postgresql/data

volumes:
  pgdata: {}
  
```

- Version: i don't recommend putting the version line.
- image: username_or_organization/image_name:version
  - You can also declare a Docker Hub image: ubuntu:20.04

- db
  - image: postgres:12



### Update - Postgres

- image: postgres:12 > 16

https://github.com/paperless-ngx/paperless-ngx/discussions/6669#discussioncomment-9445644
