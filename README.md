# SGX Development Environment in Fedora

`docker build -t sgx-devel .`

`docker run -it --rm -v $(pwd):/sgx-devel sgx-devel`

## Build

`cd hello-world && make`

## Run

`cd build && ./helloworld`

## Credit

Based on sdk and sample code from: https://github.com/baidu/rust-sgx-sdk
