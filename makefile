
all: teste

salva:
	tar -cvf ../versões/cobrinha-classica.v1.0.3.tar src/ tests/ data/ Cargo.toml makefile

testes:
	cargo test -q --offline -- --show-output \
	criando_simples_linque_externo_ao_caixote

release:
	cargo build --release --offline
