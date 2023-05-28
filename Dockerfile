FROM --platform=linux/amd64 debian AS builder
WORKDIR /code
COPY . .
RUN cp .env.prod .env
RUN apt update -y
RUN apt install -y curl \
                   build-essential \
                   libunwind-dev \
                   libssl-dev \
                   lldb \
                   pkg-config \
                   binutils-dev \
                   git \
                   libpq-dev   # Install PostgreSQL client library and headers

# Install OpenSSL libraries
RUN apt install -y openssl libssl-dev

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo clean
RUN cargo build --config net.git-fetch-with-cli=true --release
ENV PORT 8080
CMD diesel database setup && ./target/debug/ketalk