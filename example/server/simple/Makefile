GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean
PROTOC=protoc
PROTOS=

PROTOC_GEN_GO=--go_out=./proto --go_opt=paths=source_relative --go-grpc_out=./proto --go-grpc_opt=paths=source_relative

BINARY_NAME=simple

out = "."

all: clean build run

build:
	$(PROTOC) $(PROTOC_GEN_GO) greeter.proto timekeeper.proto productfinder.proto debugger.proto test.proto
	$(GOBUILD) -o $(BINARY_NAME) -v ./main.go

clean:
	$(GOCLEAN)
	rm -f $(BINARY_NAME)
	rm -f proto/greeter.pb.go
	rm -f proto/greeter_grpc.pb.go
	rm -f proto/timekeeper.pb.go
	rm -f proto/timekeeper_grpc.pb.go
	rm -f proto/productfinder.pb.go
	rm -f proto/productfinder_grpc.pb.go
	rm -f proto/debugger.pb.go
	rm -f proto/debugger_grpc.pb.go
	rm -f proto/test.pb.go
	rm -f proto/test_grpc.pb.go

run: build
	./$(BINARY_NAME)
