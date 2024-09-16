# syntax=docker/dockerfile:1.9
ARG RUST_VERSION=1.80
ARG RUST_RELEASE_MODE=debug

ARG AMD_BUILDER_IMAGE=rust:${RUST_VERSION}
# Repo: https://github.com/raskyld/lemmy-cross-toolchains
ARG ARM_BUILDER_IMAGE="ghcr.io/raskyld/aarch64-lemmy-linux-gnu:v0.4.0"

ARG AMD_RUNNER_IMAGE=debian:bookworm-slim
ARG ARM_RUNNER_IMAGE=debian:bookworm-slim

ARG UNAME=cyberbus
ARG UID=1000
ARG GID=1000

# AMD64 builder
FROM --platform=${BUILDPLATFORM} ${AMD_BUILDER_IMAGE} AS build-amd64

ARG RUST_RELEASE_MODE
ARG RUSTFLAGS

RUN mkdir -p /data/repo/cyberbus-backend
WORKDIR /data/repo/cyberbus-backend

COPY . ./

# Debug build
RUN --mount=type=cache,target=/data/repo/cyberbus-backend/target set -ex; \
    if [ "${RUST_RELEASE_MODE}" = "debug" ]; then \
    cargo build ; \
    mv target/"${RUST_RELEASE_MODE}"/cyberbus_backend ./cyberbus-backend; \
    fi

# Release build
RUN --mount=type=cache,target=/data/repo/cyberbus-backend/target set -ex; \
    if [ "${RUST_RELEASE_MODE}" = "release" ]; then \
    cargo clean --release; \
    cargo build --release; \
    mv target/"${RUST_RELEASE_MODE}"/cyberbus_backend ./cyberbus-backend; \
    fi

# ARM64 builder
# NB(raskyld): this is a hack to be able to COPY --from= this image, because the variable doesn't
# seem to be expended in --form arg of COPY :(
FROM --platform=linux/amd64 ${ARM_BUILDER_IMAGE} AS build-arm64

ARG RUST_RELEASE_MODE
ARG RUSTFLAGS

RUN mkdir -p /data/repo/cyberbus-backend
WORKDIR /data/repo/cyberbus-backend
USER 10001:10001

COPY --chown=cyberbus:cyberbus . ./

ENV PATH="/home/cyberbus/.cargo/bin:${PATH}"
ENV RUST_RELEASE_MODE=${RUST_RELEASE_MODE} 

# Debug build
RUN --mount=type=cache,target=./target,uid=10001,gid=10001 set -ex; \
    if [ "${RUST_RELEASE_MODE}" = "debug" ]; then \
    cargo build ; \
    mv "./target/$CARGO_BUILD_TARGET/$RUST_RELEASE_MODE/cyberbus_backend" /data/repo/cyberbus-backend/cyberbus-backend; \
    fi

# Release build
RUN --mount=type=cache,target=./target,uid=10001,gid=10001 set -ex; \
    if [ "${RUST_RELEASE_MODE}" = "release" ]; then \
    cargo clean --release; \
    cargo build --release; \
    mv "./target/$CARGO_BUILD_TARGET/$RUST_RELEASE_MODE/cyberbus_backend" /data/repo/cyberbus-backend/cyberbus-backend; \
    fi


# amd64 base runner
FROM ${AMD_RUNNER_IMAGE} AS runner-linux-amd64

# Add system packages that are needed: federation needs CA certificates, curl can be used for healthchecks
RUN apt update && apt install -y libssl-dev libpq-dev ca-certificates curl
RUN mkdir -p /data/repo/cyberbus-backend

COPY --from=build-amd64 --chmod=0755 /data/repo/cyberbus-backend /data/repo/cyberbus-backend

# arm base runner
FROM ${ARM_RUNNER_IMAGE} AS runner-linux-arm64

RUN apt update && apt install -y libssl-dev libpq-dev ca-certificates curl
RUN mkdir -p /data/repo/cyberbus-backend

COPY --from=build-arm64 --chmod=0755 /data/repo/cyberbus-backend /data/repo/cyberbus-backend

# Final image that use a base runner based on the target OS and ARCH
FROM runner-${TARGETOS}-${TARGETARCH}

LABEL org.opencontainers.image.authors="The Lemmy Authors"
LABEL org.opencontainers.image.source="https://github.com/cyberbus-net/cyberbus-backend"
LABEL org.opencontainers.image.licenses="None"
LABEL org.opencontainers.image.description="Cloud control API for cyberbus utils"

ARG UNAME
ARG GID
ARG UID

RUN groupadd -g ${GID} -o ${UNAME} && \
    useradd -m -u ${UID} -g ${GID} -o -s /bin/bash ${UNAME}
USER $UNAME

ENTRYPOINT ["/data/repo/cyberbus-backend/cyberbus-backend"]
EXPOSE 8536
STOPSIGNAL SIGTERM
