.PHONY: native native-run native-test host-test legacy-test bootstrap-b1-arbitrary-test bootstrap-non-rust-test bootstrap-byte-determinism-test bootstrap-second-stage-test bootstrap-clean-env-test bootstrap-self-rebuild-test test package clean

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

doctor:
	bash scripts/doctor.sh

bootstrap-test:
	./scripts/bootstrap/verify_b0_artifacts.sh --release

bootstrap-b1-test:
	./scripts/bootstrap/verify_b1_lexer.sh

bootstrap-b1-arbitrary-test:
	./scripts/bootstrap/verify_b1_arbitrary_blocks.sh

bootstrap-b3-test:
	./scripts/bootstrap/verify_b3_foundations.sh

bootstrap-vm-test:
	./scripts/bootstrap/verify_vm_platform.sh

bootstrap-clean-repo-test:
	./scripts/bootstrap/assert_clean_repo_root.sh

bootstrap-refactor-smoke-test:
	./scripts/bootstrap/run_zap_refactor_smoke.sh

bootstrap-non-rust-test:
	./scripts/bootstrap/verify_non_rust_seed_pipeline.sh

bootstrap-byte-determinism-test:
	./scripts/bootstrap/verify_b4_byte_determinism.sh

bootstrap-second-stage-test:
	./scripts/bootstrap/verify_b4_second_stage_rebuild.sh

bootstrap-clean-env-test:
	./scripts/bootstrap/verify_b4_clean_environment.sh

bootstrap-self-rebuild-test: bootstrap-byte-determinism-test bootstrap-second-stage-test bootstrap-clean-env-test

legacy-test:
	cd legacy && python3 -m unittest -v test_zap.py

test: legacy-test native-test host-test bootstrap-test bootstrap-b1-test bootstrap-b1-arbitrary-test bootstrap-b3-test bootstrap-vm-test bootstrap-clean-repo-test bootstrap-refactor-smoke-test bootstrap-non-rust-test bootstrap-self-rebuild-test

package: native
	./package_release.sh x86_64-unknown-linux-gnu

# The Python prototype is optional reference tooling; native is the default runtime.
clean:
	cargo clean --manifest-path native/Cargo.toml
	rm -f bin/zap bin/zap.exe
	rm -rf dist
