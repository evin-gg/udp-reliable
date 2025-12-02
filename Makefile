proxyIP = 192.168.1.95
proxyPORT = 40000

serverIP = 192.168.1.95
serverPORT = 40001

build:
	cargo clean
	cargo build

nclient:
	cargo build --bin client
	./target/debug/client --target-ip  $(proxyIP) --target-port $(proxyPORT) --timeout 1 --max-retries 1

nserver:
	cargo build --bin server
	./target/debug/server --listen-ip $(serverIP) --listen-port $(serverPORT)

client:
	cargo build --bin client
	./target/debug/client --target-ip $(proxyIP) --target-port $(proxyPORT) --timeout 2 --max-retries 10
	
server:
	cargo build --bin server
	./target/debug/server --listen-ip $(serverIP) --listen-port $(serverPORT)

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

proxy2:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 0 \
		--server-drop 50 \
		--client-delay 0 \
		--server-delay 0 \
		--client-delay-time-min 0 \
		--client-delay-time-max 0 \
		--server-delay-time-min 0 \
		--server-delay-time-max 0 \

proxy3:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 0 \
		--server-drop 100 \
		--client-delay 0 \
		--server-delay 0 \
		--client-delay-time-min 0 \
		--client-delay-time-max 0 \
		--server-delay-time-min 0 \
		--server-delay-time-max 0 \

proxy4:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 0 \
		--server-drop 0 \
		--client-delay 50 \
		--server-delay 0 \
		--client-delay-time-min 100 \
		--client-delay-time-max 500 \
		--server-delay-time-min 0 \
		--server-delay-time-max 0 \

proxy5:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 0 \
		--server-drop 0 \
		--client-delay 100 \
		--server-delay 0 \
		--client-delay-time-min 2500 \
		--client-delay-time-max 3000 \
		--server-delay-time-min 0 \
		--server-delay-time-max 0 \

proxy6:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 50 \
		--server-drop 0 \
		--client-delay 50 \
		--server-delay 0 \
		--client-delay-time-min 2500 \
		--client-delay-time-max 3000 \
		--server-delay-time-min 0 \
		--server-delay-time-max 0 \

proxy7:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 50 \
		--server-drop 0 \
		--client-delay 0 \
		--server-delay 0 \
		--client-delay-time-min 0 \
		--client-delay-time-max 0 \
		--server-delay-time-min 0 \
		--server-delay-time-max 0 \

proxy8:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 100 \
		--server-drop 0 \
		--client-delay 0 \
		--server-delay 0 \
		--client-delay-time-min 0 \
		--client-delay-time-max 0 \
		--server-delay-time-min 0 \
		--server-delay-time-max 0 \

proxy9:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 0 \
		--server-drop 0 \
		--client-delay 0 \
		--server-delay 50 \
		--client-delay-time-min 2500 \
		--client-delay-time-max 3000 \
		--server-delay-time-min 2500 \
		--server-delay-time-max 3000 \

proxy10:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 0 \
		--server-drop 0 \
		--client-delay 0 \
		--server-delay 100 \
		--client-delay-time-min 2500 \
		--client-delay-time-max 3000 \
		--server-delay-time-min 2500 \
		--server-delay-time-max 3000 \

proxy11:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 0 \
		--server-drop 50 \
		--client-delay 0 \
		--server-delay 50 \
		--client-delay-time-min 2500 \
		--client-delay-time-max 3000 \
		--server-delay-time-min 2500 \
		--server-delay-time-max 3000 \

proxy12:
	cargo build --bin proxy
	./target/debug/proxy --listen-ip $(proxyIP) \
		--listen-port $(proxyPORT) \
		--target-ip $(serverIP) \
		--target-port $(serverPORT) \
		--client-drop 50 \
		--server-drop 50 \
		--client-delay 50 \
		--server-delay 50 \
		--client-delay-time-min 2500 \
		--client-delay-time-max 3000 \
		--server-delay-time-min 2500 \
		--server-delay-time-max 3000 \
