.PHONY: native native-run native-test host-test legacy-test test package clean

native:
	cargo build --release --locked --manifest-path native/Cargo.toml
	mkdir -p bin
	cp native/target/release/zap bin/zap
	chmod 0755 bin/zap
	bin/zap --version

native-run: native
	./bin/zap native_hello.zp

native-test:
	cargo test --manifest-path native/Cargo.toml --all-targets --all-features --locked

host-test:
	cargo test --manifest-path host/zap-host/Cargo.toml --all-targets --locked

bootstrap-test:
	./scripts/bootstrap/verify_b0_artifacts.sh --release

legacy-test:
	cd legacy && python3 -m unittest -v test_zap.py

test: legacy-test native-test host-test bootstrap-test

package: native
	./package_release.sh x86_64-unknown-linux-gnu

# The Python prototype is optional reference tooling; native is the default runtime.
clean:
	cargo clean --manifest-path native/Cargo.toml
	rm -f bin/zap bin/zap.exe
	rm -rf dist
