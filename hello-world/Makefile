UNTRUSTED_RUST_TARGET := app
TRUSTED_RUST_TARGET := libhelloworld_trusted.a
ENCLAVE_NAME := enclave.so
SIGNED_ENCLAVE_NAME := enclave.signed.so

SGX_MODE ?= SW
SGX_DEBUG ?= 1

### SGX

SGX_SDK ?= /opt/intel/sgxsdk
SGX_COMMON_CFLAGS := -m64
SGX_LIBRARY_PATH := $(SGX_SDK)/lib64
SGX_ENCLAVE_SIGNER := $(SGX_SDK)/bin/x64/sgx_sign
SGX_EDGER8R := $(SGX_SDK)/bin/x64/sgx_edger8r
ENCLAVE_EDL_FILES := build/enclave_t.c build/enclave_t.h build/enclave_u.c build/enclave_u.h

ifeq ($(SGX_DEBUG), 1)
	SGX_COMMON_CFLAGS += -O0 -g
	BUILD_DIR := target/debug
else
	SGX_COMMON_CFLAGS += -O2
	CARGO_FLAGS := --release
	BUILD_DIR := target/release
endif

### RUST SGX SDK

RUST_SGX_SDK ?= /opt/rust-sgx-sdk
CUSTOM_EDL_PATH := $(RUST_SGX_SDK)/edl
CUSTOM_COMMON_PATH := $(RUST_SGX_SDK)/common

### UNTRUSTED SETTINGS

UNTRUSTED_C_SRC := $(shell find untrusted/ -type f -name '*.c')
UNTRUSTED_RUST_SRC := $(shell find untrusted/ -type f -name '*.rs') $(shell find untrusted/ -type f -name 'Cargo.toml')
UNTRUSTED_INCLUDE_PATHS := -Iuntrusted -Iinclude -I$(SGX_SDK)/include -I$(CUSTOM_EDL_PATH)
UNTRUSTED_C_FLAGS := $(SGX_COMMON_CFLAGS) -fPIC -Wno-attributes $(UNTRUSTED_INCLUDE_PATHS)

### TRUSTED SETTINGS

ifneq ($(SGX_MODE), HW)
	TRTS_LIB := sgx_trts_sim
	TSERVICE_LIB := sgx_tservice_sim
else
	TRTS_LIB := sgx_trts
	TSERVICE_LIB := sgx_tservice
endif
CRYPTO_LIB := sgx_tcrypto
KEY_EXCHANGE_LIB := sgx_tkey_exchange
PROTECTED_FS_LIB := sgx_tprotected_fs

TRUSTED_C_SRC := $(shell find trusted/ -type f -name '*.c')
TRUSTED_RUST_SRC := $(shell find trusted/ -type f -name '*.rs') $(shell find trusted/ -type f -name 'Cargo.toml')
TRUSTED_INCLUDE_PATHS := -Itrusted -Iinclude -I$(CUSTOM_COMMON_PATH)/inc -I$(CUSTOM_EDL_PATH) -I$(SGX_SDK)/include -I$(SGX_SDK)/include/tlibc -I$(SGX_SDK)/include/stlport -I$(SGX_SDK)/include/epid
TRUSTED_LINK_LIBS := -Lbuild -lenclave
TRUSTED_COMPILE_FLAGS := $(SGX_COMMON_CFLAGS) -nostdinc -fvisibility=hidden -fpie -fstack-protector $(TRUSTED_INCLUDE_PATHS)
TRUSTED_LINK_FLAGS := $(SGX_COMMON_CFLAGS) -Wl,--no-undefined -nostdlib -nodefaultlibs -nostartfiles -L$(SGX_LIBRARY_PATH) \
	-Wl,--whole-archive -l$(TRTS_LIB) -l$(TSERVICE_LIB) -Wl,--no-whole-archive \
	-Wl,--start-group -lsgx_tstdc -lsgx_tcxx -l$(CRYPTO_LIB) $(TRUSTED_LINK_LIBS) -Wl,--end-group \
	-Wl,-Bstatic -Wl,-Bsymbolic -Wl,--no-undefined \
	-Wl,-pie,-eenclave_entry -Wl,--export-dynamic  \
	-Wl,--defsym,__ImageBase=0 \
	-Wl,--gc-sections \
	-Wl,--version-script=trusted/enclave.lds

.PHONY: all
all: build/$(UNTRUSTED_RUST_TARGET) build/$(SIGNED_ENCLAVE_NAME)

### EDL OBJECTS

$(ENCLAVE_EDL_FILES): $(SGX_EDGER8R) trusted/enclave.edl
	@$(SGX_EDGER8R) --trusted trusted/enclave.edl --search-path $(SGX_SDK)/include --search-path $(CUSTOM_EDL_PATH) --trusted-dir build
	@$(SGX_EDGER8R) --untrusted trusted/enclave.edl --search-path $(SGX_SDK)/include --search-path $(CUSTOM_EDL_PATH) --untrusted-dir build
	@echo "GEN    =>  $(ENCLAVE_EDL_FILES)"

### UNTRUSTED BUILD

build/enclave_u.o: build/enclave_u.c
	@$(CC) $(UNTRUSTED_C_FLAGS) -c $^ -o $@
	@echo "CC     =>  $@"

build/libenclave_u.a: build/enclave_u.o
	@$(AR) rcsD $@ $^

build/$(UNTRUSTED_RUST_TARGET): build/libenclave_u.a $(UNTRUSTED_RUST_SRC)
	@cd untrusted && cargo build $(CARGO_FLAGS)
	@cp untrusted/$(BUILD_DIR)/$(UNTRUSTED_RUST_TARGET) $@
	@echo "Cargo  =>  $@"

### TRUSTED BUILD

build/enclave_t.o: build/enclave_t.c
	@$(CC) $(TRUSTED_COMPILE_FLAGS) -c $^ -o $@
	@echo "CC     =>  $@"

build/libenclave.a: $(TRUSTED_RUST_SRC)
	@cd trusted && RUSTC_BOOTSTRAP=1 cargo build $(CARGO_FLAGS)
	@cp trusted/$(BUILD_DIR)/$(TRUSTED_RUST_TARGET) $@
	@echo "Cargo  =>  $@"

build/$(ENCLAVE_NAME): build/libenclave.a build/enclave_t.o
	@$(CXX) build/enclave_t.o -o $@ $(TRUSTED_LINK_FLAGS)
	@echo "LINK   =>  $@"

build/$(SIGNED_ENCLAVE_NAME): build/$(ENCLAVE_NAME)
	@$(SGX_ENCLAVE_SIGNER) sign -enclave $^ -out $@ -key trusted/enclave-private.pem -config trusted/enclave-config.xml
	@echo "SIGN   =>  $@"

### CLEAN

.PHONY: clean
clean:
	@rm -rf build

.PHONY: clean-rust
clean-rust:
	@cd trusted && cargo clean && rm -f Cargo.lock
	@cd untrusted && cargo clean && rm -f Cargo.lock