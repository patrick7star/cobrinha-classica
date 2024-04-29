
// bibliotecas externas:
extern crate rand;
// importando da minha biblioteca:
use cobrinha_classica::*;


// execução de testes...
fn main() {
   //use cobrinha_classica::VELOCIDADE;
   use std::time::Duration;

   let mut tabuleiro = Tela::nova(
      Duration::from_millis(700),
      Duration::from_millis(500),
      Some("teste_inicial")
   ).expect("erro na criação da instância");
   // obtendo dimensão do terminal...
   let (linhas, colunas) = tabuleiro.dimensao();
   // obtendo a posição do meio da tela.
   let meio = Ponto::novo( 
      (linhas / 2) as u8, 
      (colunas / 2) as u8 
   );

   // instânciando cobrinha e os bichinhos/alvos.
   let mut cobra = Cobrinha::criar(meio);
   cobrinha_proporcional(&mut cobra, (linhas, colunas));
   // enrrola ela antes de aparecer em tela.
   encaracola(&mut cobra);
   let mut metas = Alvos::cria(
      (linhas-1) as u8, 
      (colunas-1) as u8, 
      rand::random::<u16>() % 100 
   );
   // rodando o jogo, e colhendo dados.
   let dados_do_jogo = roda_jogo(&mut tabuleiro, &mut cobra, &mut metas); 

   // finalizando terminal...
   drop(tabuleiro);
   // visualizando informação...
   println!("{}", dados_do_jogo);

   // salvando o resultado ...
   match salva_no_bd(dados_do_jogo.serializa()) {
      Ok(_) => 
         { println!("partida registrada com sucesso."); }
      Err(erro) => 
         { println!("ERRO:[{}]", erro); }
   };

   // criando links ao executável.
   links::linka_executaveis("cobrinha");
   links::linca_executaveis_externamente("cobrinha-jogo").unwrap();
}
