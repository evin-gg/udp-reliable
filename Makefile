build:
	cargo clean
	cargo build

client:
	cargo build --bin client
	./target/debug/client --target-ip 192.168.1.96 --target-port 50000 --timeout 1 --max-retries 5
	
server:
	cargo build --bin server
	./target/debug/server --target-ip 192.168.1.96 --target-port 50000
