FROM baiduxlab/sgx-rust

ENV rust_toolchain=nightly-2021-09-27

RUN bash /root/05_rust.sh

WORKDIR /usr/src/oram

