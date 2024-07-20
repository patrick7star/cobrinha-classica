
all: testes

VERSAO = v1.1.1
salva:
	tar -cvf ../versões/cobrinha-classica.$(VERSAO).tar \
		src/ tests/ data/ Cargo.toml makefile

backups:
	@echo "\nVisualizando todas versões já produzidas...\n"
	@ls --size -h --sort=time -1 ../versões/cobrinha-classica*

testes:
	cargo test -q --offline -- --show-output \
	criando_simples_linque_externo_ao_caixote

jogada-teste:
	cargo test -q --offline --test jogadas_rapidas_e_faceis -- cobrinha_sem_barreiras

visualiza-movimento:
	cargo test -q --offline --test visualiza_movimento 

release:
	cargo build --release --offline
