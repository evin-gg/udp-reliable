proxyIP = 127.0.0.1
proxyPORT = 40000

serverIP = 192.168.1.96
serverPORT = 40000

build:
	cargo clean
	cargo build

nclient:
	cargo build --bin client
	./target/debug/client --target-ip 192.168.1.96 --target-port 50000 --timeout 1 --max-retries 1

nserver:
	cargo build --bin server
	./target/debug/server --target-ip 192.168.1.96 --target-port 50000

client:
	cargo build --bin client
	./target/debug/client --target-ip $(proxyIP) --target-port $(proxyPORT) --timeout 1 --max-retries 0
	
server:
	cargo build --bin server
	./target/debug/server --target-ip $(serverIP) --target-port $(serverPORT)

proxy:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 0 \
		--server-drop 0 \
		--client-delay 0 \
		--server-delay 0 \
		--client-delay-time-min 0 \
		--client-delay-time-max 0 \
		--server-delay-time-min 0 \
		--server-delay-time-max 0 \

logger:
	python ./src/graph.py
