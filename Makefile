ifeq ($(OS),Windows_NT)
    CLEAR_COMMAND = @cls
    COPY_COMMAND = copy
    MKDIR_COMMAND = mkdir
    RM_COMMAND = rmdir /s /q
else
    CLEAR_COMMAND = @clear
    COPY_COMMAND = cp
    MKDIR_COMMAND = mkdir -p
    RM_COMMAND = rm -rf
endif

# Rust library path
RUST_VM_PATH = ./rust_vm
RUST_LIB_NAME = rust_vm.dll
RUST_LIB_TARGET_DIR = $(RUST_VM_PATH)/target/release

# Go binary path
GO_BIN_DIR = ./bin
GO_BIN_NAME = hvm.exe

test:
	@go test ./...

test-verbose:
	@go test -v ./...

test-race:
	@go test ./... --race

build: rust-build go-build

# Build rust library
rust-build:
	@echo "Building Rust library..."
	@cd $(RUST_VM_PATH) && cargo build --release
	@echo "Copying Rust library to project root..."
	@$(MKDIR_COMMAND) $(GO_BIN_DIR)
	@$(COPY_COMMAND) $(RUST_LIB_TARGET_DIR)\$(RUST_LIB_NAME) .

# Build go binary
go-build:
	@echo "Building Go binary..."
	@go build -o $(GO_BIN_DIR)\$(GO_BIN_NAME)

#run: build
#	@$(CLEAR_COMMAND)
#	@$(GO_BIN_DIR)/$(GO_BIN_NAME)

run:
	@CGO_ENABLED=$(CGO_ENABLED)
	@go run main.go


clean:
	@echo "Cleaning build artifacts..."
	@$(RM_COMMAND) $(GO_BIN_DIR)
	@del $(RUST_LIB_NAME)
	@cd $(RUST_VM_PATH) && cargo clean
