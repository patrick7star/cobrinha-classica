
all: testes

salva:
	tar -cvf ../versões/cobrinha-classica.v1.1.0.tar \
		src/ tests/ data/ Cargo.toml makefile

testes:
	cargo test -q --offline -- --show-output \
	criando_simples_linque_externo_ao_caixote

jogada-teste:
	cargo test -q --offline --test jogadas_rapidas_e_faceis -- cobrinha_sem_barreiras

visualiza-movimento:
	cargo test -q --offline --test visualiza_movimento 

release:
	cargo build --release --offline
