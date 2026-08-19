.PHONY: native native-run native-test package test clean

native:
	cargo build --release --manifest-path native/Cargo.toml
	mkdir -p bin
	cp native/target/release/zap bin/zap
	chmod 0755 bin/zap
	bin/zap --version

native-run: native
	./bin/zap native_hello.zp

native-test:
	cargo test --manifest-path native/Cargo.toml

package: native
	./package_release.sh x86_64-unknown-linux-gnu

# The Python prototype is optional reference tooling; native is the default runtime.
test:
	python3 -m unittest -v test_zap.py

clean:
	cargo clean --manifest-path native/Cargo.toml
	rm -f bin/zap bin/zap.exe
	rm -rf dist
