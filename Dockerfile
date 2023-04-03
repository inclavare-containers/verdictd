FROM registry.cn-hangzhou.aliyuncs.com/alinux/alinux3 as builder

WORKDIR /usr/src/verdictd

ENV RATS_TLS_COMMIT 5de6fc3
ENV VERDICTD_COMMIT 6579237

COPY . .

# Install Build Dependencies
RUN yum install -y yum-utils
RUN yum install -y \
clang \
cmake \
curl \
git \
gnupg \
clang-devel \
openssl-devel \
protobuf-devel \
llvm-devel \
make \
pkg-config \
protobuf-compiler \
wget \
tar
RUN wget https://go.dev/dl/go1.20.1.linux-amd64.tar.gz
RUN tar -C /usr/local -xzf go1.20.1.linux-amd64.tar.gz
ENV PATH="/usr/local/go/bin:${PATH}"

# Install TDX Dependencies
RUN wget https://download.01.org/intel-sgx/sgx-dcap/1.15/linux/distro/Anolis86/sgx_rpm_local_repo.tgz; \
tar xzvf sgx_rpm_local_repo.tgz; \
yum-config-manager --add-repo file://$(realpath sgx_rpm_local_repo); \
yum install -y --setopt=install_weak_deps=False --nogpgcheck libsgx-urts libtdx-attest libsgx-dcap-default-qpl libsgx-dcap-quote-verify

# Install rats-tls
RUN rpm -ivh /usr/src/verdictd/deps/rats-tls-tdx-0.6.4-1.al8.x86_64.rpm

# Install Rust toolchain
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y; \
echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Build and Install verdictd
RUN git reset --hard ${VERDICTD_COMMIT}; \
make verdictd && make install


FROM registry.cn-hangzhou.aliyuncs.com/alinux/alinux3

RUN yum install -y yum-utils
RUN yum install -y clang wget tar

# Install TDX Dependencies
RUN wget https://download.01.org/intel-sgx/sgx-dcap/1.15/linux/distro/Anolis86/sgx_rpm_local_repo.tgz; \
tar xzvf sgx_rpm_local_repo.tgz; \
yum-config-manager --add-repo file://$(realpath sgx_rpm_local_repo); \
yum install -y --setopt=install_weak_deps=False --nogpgcheck libsgx-urts libtdx-attest libsgx-dcap-default-qpl libsgx-dcap-quote-verify

# Install rats-tls
COPY --from=builder /usr/src/verdictd/deps/rats-tls-tdx-0.6.4-1.al8.x86_64.rpm /usr/src/rats-tls-tdx-0.6.4-1.al8.x86_64.rpm
RUN rpm -ivh /usr/src/rats-tls-tdx-0.6.4-1.al8.x86_64.rpm

# Intel PCCS URL Configurations
RUN sed -i "s|\"use_secure_cert\":.*$|\"use_secure_cert\":false,|" /etc/sgx_default_qcnl.conf;

COPY --from=builder /usr/local/lib/libopa.so /usr/local/lib/libopa.so
COPY --from=builder /usr/local/bin/verdictd /usr/local/bin/verdictd

CMD ["verdictd", "--listen", "0.0.0.0:30000"]

VOLUME /opt/verdictd
EXPOSE 30000