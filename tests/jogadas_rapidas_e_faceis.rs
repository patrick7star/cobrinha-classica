

/** Onde tenho que realizar o jogo,
tendo vitórias as vezes, se não posso
alterar a quantia aleatórias de "alvos"
fica incapaz de fazer um testes no 
tempo, por mim delimitado.
*/

use cobrinha_classica::*;
use std::time::Duration;


// execução de testes...
#[test]
fn teste_enxuto() {
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
      3 + rand::random::<u16>() % 7 
   );
   // rodando o jogo, e colhendo dados.
   let dados_do_jogo = roda_jogo(&mut tabuleiro, &mut cobra, &mut metas); 

   // finalizando terminal...
   drop(tabuleiro);

   // visualizando informação...
   println!("{}", dados_do_jogo);
}

#[test]
#[allow(non_snake_case)]
fn cobrinha_sem_barreiras() {
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
   // total de bugs é no mínimo três, no máximo dez.
   let total_de_bugs = 3 + rand::random::<u16>() % 7; 
   let A = (linhas - 1) as u8;
   let L = (colunas - 1) as u8;
   let mut metas = Alvos::cria(A, L, total_de_bugs);

   // rodando o jogo, e colhendo dados.
   let dados_do_jogo = roda_jogo_sem_barreiras(
      &mut tabuleiro, 
      &mut cobra, 
      &mut metas
   ); 

   // finalizando terminal...
   drop(tabuleiro);

   // visualizando informação...
   println!("{}", dados_do_jogo);
}
