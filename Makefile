build:
	cargo clean
	cargo build

client:
	cargo run --bin client 192.168.1.96 50000 1 1
	
server:
	cargo run --bin server 192.168.1.96 50000
