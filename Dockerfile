FROM baiduxlab/sgx-rust

ENV SDK_URL=https://download.01.org/intel-sgx/sgx-linux/2.15/distro/ubuntu18.04-server/sgx_linux_x64_sdk_2.15.100.3.bin

RUN bash /root/03_sdk.sh

ENV rust_toolchain=nightly-2021-08-01

RUN bash /root/05_rust.sh

RUN apt-get update && \
  apt-get install -y clang \
  libclang-dev

#this is necessary to get mobilecoin to compile
RUN ln -s /opt/sgxsdk /opt/intel/sgxsdk

WORKDIR /usr/src/oram

#SGX_MODE=SW IAS_MODE=DEV cargo run