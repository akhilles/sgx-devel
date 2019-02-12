FROM fedora:27

RUN set -eux && \
    dnf -y update && \
    dnf -y install make git clang which findutils && \
    dnf clean all

# SGX
RUN set -eux && \
    cd /opt && \
    curl -# https://download.01.org/intel-sgx/linux-2.4/fedora27-server/sgx_linux_x64_sdk_2.4.100.48163.bin -o sgx_sdk.bin && \
    echo "yes" | sh sgx_sdk.bin

# Rust
RUN set -eux && \
    curl https://sh.rustup.rs -# | sh -s -- --default-toolchain 1.32.0 -y && \
    cp /root/.cargo/bin/* /usr/local/bin

# Rust SGX SDK
RUN set -eux && \
    cd /opt && \
    git clone -b rust-stable --single-branch https://github.com/baidu/rust-sgx-sdk.git

# Environment variables
ENV SGX_MODE SW
ENV SGX_SDK /opt/sgxsdk
ENV RUST_SGX_SDK /opt/rust-sgx-sdk
ENV LD_LIBRARY_PATH $SGX_SDK/sdk_libs

WORKDIR /sgx-devel

CMD "bash"
