release:
	cargo build --release

test:
	cargo run -- new test

install:
	cargo build --release
	sudo cp ./target/release/makelatex /bin/


uninstall:
	sudo rm /bin/makelatex