// Bibliotecas externas:
extern crate rand;
// Importando da minha biblioteca:
use cobrinha_classica::*;
// Bibliote padrão do Rust:
use std::time::Duration;
use std::env::args as ARGS;


// execução de testes...
#[allow(non_snake_case)]
fn main() {
   let mut tabuleiro = Tela::nova(
      Duration::from_millis(700),
      Duration::from_millis(500),
      Some("teste_inicial")
   ).expect("erro na criação da instância");
   // obtendo dimensão do terminal...
   let (linhas, colunas) = tabuleiro.dimensao();
   // obtendo a posição do meio da tela.
   let meio = Ponto::novo((linhas / 2) as u8, (colunas / 2) as u8);
   // instânciando cobrinha e os bichinhos/alvos.
   let (LIN, COL) = ((linhas - 1) as u8, (colunas - 1) as u8);
   let S = rand::random::<u16>() % 100;
   let mut metas = Alvos::cria(LIN, COL, S);
   let dados_do_jogo: Dados; 
   let mut cobra: Cobrinha;

   /* Seleciona o tipo de corpo da cobrinha. Por enquanto, só há três
    * modalidades: a classíca(com um símbolo hash); a nova, que é a de 
    * bolinha; e uma mista, onde o corpo é de bolinha, e a cabeça é a um
    * símbolo da 'velha'. Se nenhuma opção for explicitada, a última será
    * selecionada automaticamente. */
   if ARGS().any(|a| a == "--bolinha")
      { cobra = Cobrinha::criar_a(meio, 'o'); }
   else if ARGS().any(|a| a == "--velha")
      { cobra = Cobrinha::criar(meio); }
   else
      { cobra = Cobrinha::criar_b(meio); }

   cobrinha_proporcional(&mut cobra, (linhas, colunas));
   // Enrrola ela sobre o próprio corpo, antes de aparecer em tela.
   encaracola(&mut cobra);

   /* Duas modalidades de jogo: o padrão, que termina após colisão com a 
    * tela, e colisão com o próprio corpo; e "infinito", que se 
    * teletransporta ao outro lado após colidir com as paredes, e só 
    * termino mesmo quando colidido com o próprio corpo. */
   if ARGS().any(|a| a == "--sem-barreiras") { 
      dados_do_jogo = roda_jogo_sem_barreiras(
         &mut tabuleiro, 
         &mut cobra, 
         &mut metas
      );  
   } else { 
      dados_do_jogo = roda_jogo(
         &mut tabuleiro, 
         &mut cobra, 
         &mut metas
      );  
   }

   // finalizando terminal...
   drop(tabuleiro);
   // visualizando informação...
   println!("{}", dados_do_jogo);

   // Salvando o resultado, em caso de algum erro mostrar mensagem.
   if let Err(erro) = salva_no_bd(dados_do_jogo.serializa())
      { println!("ERRO:[{}]", erro); }

   // criando links ao executável.
   #[cfg(target_os="unix")]
   links::linka_executaveis("cobrinha");
   #[cfg(target_os="unix")]
   links::linca_executaveis_externamente("cobrinha-jogo").unwrap();
}
