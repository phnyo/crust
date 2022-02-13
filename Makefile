crust:
	cargo b --release

test: crust
	bash ./test.sh

clean:
	rm -f ./target/release/crust tmp.*

.PHONY: test clean

