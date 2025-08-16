ifeq ($(OS),Windows_NT)
    CLEAR_COMMAND = @cls
    COPY_COMMAND = copy
    RM_COMMAND = rmdir /s /q
else
    CLEAR_COMMAND = @clear
    COPY_COMMAND = cp
    RM_COMMAND = rm -rf
endif

# Rust library path
RUST_STACK_VM_LIB_NAME = rust_stack_vm
RUST_STACK_VM_PATH = ./$(RUST_STACK_VM_LIB_NAME)
RUST_STACK_VM_DLL_TARGET = $(RUST_STACK_VM_LIB_NAME).dll
RUST_STACK_VM_LIB_TARGET_DIR = $(RUST_STACK_VM_LIB_NAME)/target/release

# Go binary path
GO_BIN_DIR = bin
GO_BIN_NAME = hvm

# Build rust library
rust-build:
	@echo "Building Rust library..."
	@cd $(RUST_STACK_VM_PATH) && cargo build --release
	@echo "Copying Rust library to project root..."
	@if not exist $(GO_BIN_DIR) mkdir $(GO_BIN_DIR)
	@copy "$(RUST_STACK_VM_LIB_TARGET_DIR)\$(RUST_STACK_VM_DLL_TARGET)" "./$(GO_BIN_DIR)"
	@echo "Rust library builded & dll file copied"

# Build go binary
go-build:
	@echo "Building Go binary..."
	@go build -o $(GO_BIN_DIR)\$(GO_BIN_NAME)
	@echo "Build go binary has been finished"

test:
	@cd $(RUST_STACK_VM_PATH) && cargo test
	@go test ./...

build: rust-build go-build

run: build
	@$(CLEAR_COMMAND)
	@$(GO_BIN_DIR)/$(GO_BIN_NAME)

clean:
	@echo "Cleaning up build artifacts..."
	@if exist $(GO_BIN_DIR) $(RM_COMMAND) $(GO_BIN_DIR)
	@cd $(RUST_STACK_VM_PATH) && cargo clean
	@echo "Cleanup finished."
