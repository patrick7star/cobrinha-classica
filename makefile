
all: testes

VERSAO = v1.2.1
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

# Compilação, que já considera alguns binários compilados anteriormente, ou 
# seja, ele cuida apenas da compilação do código do projeto em sí. Por 
# enquanto, apenas funcional prá plataforma Windows, porém, fácil de transpor
# para Linxu, que é onde eu já tenho exemplos disso feito.
release-faster:
	rustc.exe -o ./bin/release.exe src/main.rs \
		-L./lib/windows_x86_64/ \
		-L./lib/windows_AMD64	\
		-L./target/debug/build
# Funciona apenas na máquina do desenvolvedor, que tem tal caminho já 
# pré-definido. Ela copia binários já compilados na plataforma/máquina, 
# más também uma 'lib' da computador pessoal que provavelmente você não terá.
# Claro, na hora de "instalar", provavelmente eles já virão com ele.
importa-lib:
	@cp -u $(RUST_CODES)/rust-utilitarios/target/release/libutilitarios.rlib \
		./lib/windows_x86_64/
	@echo "A biblioteca 'utilitarios' já foi copiada na máquina pessoal do dev."
