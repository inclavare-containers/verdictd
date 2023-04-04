FROM rust:1.67-slim as builder

WORKDIR /usr/src

ENV RATS_TLS_COMMIT 5de6fc3
ENV VERDICTD_COMMIT 1d632be

ENV ALIYUN_PCCS_URL "https://sgx-dcap-server.cn-beijing.aliyuncs.com/sgx/certification/v4/"

# Install Build Dependencies
RUN apt-get update && apt install -y \
clang \
cmake \
curl \
git \
gnupg \
libclang-dev \
libcurl4-openssl-dev \
libprotobuf-dev \
libssl-dev \
llvm-dev \
make \
pkg-config \
protobuf-compiler \
wget
RUN wget https://go.dev/dl/go1.20.1.linux-amd64.tar.gz
RUN tar -C /usr/local -xzf go1.20.1.linux-amd64.tar.gz
ENV PATH="/usr/local/go/bin:${PATH}"

# Install TDX Build Dependencies
RUN curl -L https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | apt-key add -
RUN echo 'deb [arch=amd64] https://download.01.org/intel-sgx/sgx_repo/ubuntu focal main' | tee /etc/apt/sources.list.d/intel-sgx.list
RUN apt-get update
RUN apt-get install -y \
libtdx-attest \
libtdx-attest-dev \
libsgx-dcap-ql-dev \
libsgx-dcap-default-qpl \
libsgx-dcap-quote-verify \
libsgx-dcap-quote-verify-dev

# Intel PCCS URL Configurations
RUN sed -i "s|\"pccs_url\":.*$|\"pccs_url\":\"$ALIYUN_PCCS_URL\",|" /etc/sgx_default_qcnl.conf;

# Build and Install rats-tls
RUN git clone https://github.com/inclavare-containers/rats-tls
RUN cd rats-tls; \
git reset --hard ${RATS_TLS_COMMIT}; \
cmake -DBUILD_SAMPLES=on -H. -Bbuild; \
make -C build install

# Build and Install verdictd
COPY . .

RUN make && make install

FROM ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive

WORKDIR /usr/src

COPY --from=builder /usr/local/bin/verdictd /usr/local/bin/verdictd
COPY --from=builder /usr/local/lib/libopa.so /usr/local/lib/libopa.so
COPY --from=builder /usr/local/bin/verdict /usr/local/bin/verdict
COPY --from=builder /usr/local/lib/rats-tls /usr/local/lib/rats-tls
RUN apt-get update && apt install -y curl gnupg git cmake && \
    curl -L https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | apt-key add - && \
    echo 'deb [arch=amd64] https://download.01.org/intel-sgx/sgx_repo/ubuntu focal main' | tee /etc/apt/sources.list.d/intel-sgx.list   && \
    apt-get update && \
    apt-get install -y \
    libsgx-dcap-ql-dev \
    libsgx-dcap-default-qpl \
    libsgx-dcap-quote-verify \
    libsgx-dcap-quote-verify-dev && \
    rm -rf /var/lib/apt/lists/*
