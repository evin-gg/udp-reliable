build:
	cargo clean
	cargo build

client:
	cargo run --bin client 192.168.1.96 50000 2 3
	
server:
	cargo run --bin server 192.168.1.96 50000
