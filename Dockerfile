FROM rust:1-bookworm AS builder

ARG LLVM_VERSION=21.1.8

ENV LLVM_INSTALL_DIR=/opt/llvm \
    LLVM_CONFIG_PATH=/opt/llvm/bin/llvm-config \
    CLANG=/opt/llvm/bin/clang \
    CLANGXX=/opt/llvm/bin/clang++ \
    PATH=/opt/llvm/bin:/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
    LD_LIBRARY_PATH=/opt/llvm/lib

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gnupg \
    make \
    python3 \
    tar \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

COPY tools/install_llvm_prebuilt.sh tools/install_llvm_prebuilt.sh
RUN LLVM_VERSION="${LLVM_VERSION}" tools/install_llvm_prebuilt.sh

COPY . .
RUN ./setup.sh

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    libffi8 \
    libstdc++6 \
    zlib1g \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/xyo /usr/local/bin/xyo

ENTRYPOINT ["xyo"]
CMD ["--help"]
