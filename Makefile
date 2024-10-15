default:
	cargo run -- --config ./src/promguard-config.yaml

shell:
	nix develop

update:
	cargo update

test:
	cargo test

cov:
	cargo llvm-cov

up:
	podman-compose -f compose/compose.yaml up -d --build

down:
	podman-compose -f compose/compose.yaml down

dashboard:
	jsonnet -J ./compose/vendor ./compose/promguard.jsonnet > ./compose/promguard.dashboard.json
