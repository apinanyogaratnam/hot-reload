# Build the binary
build:
	go build -o main .

# Run the binary after building
start: build
	./main
