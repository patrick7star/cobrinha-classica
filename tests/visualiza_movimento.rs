
/* Cobrinhas se movimento automamente aqui para se manipular seus limites.
 */

use cobrinha_classica::*;
use std::time::Duration;
use std::convert::TryInto;


#[test]
fn movimento_em_todas_direcoes() {
   let mut tabuleiro = Tela::nova(
      Duration::from_millis(700),
      Duration::from_millis(500),
      Some("teste_inicial")
   ).expect("erro na criação da instância");
   // obtendo dimensão do terminal...
   let (linhas, colunas) = tabuleiro.dimensao();
   // obtendo a posição do meio da tela.
   let y: u8 = (linhas / 2).try_into().unwrap();
   let x: u8 = (colunas / 2) as u8;
   // instânciando cobrinha e os bichinhos/alvos.
   let mut snake = Cobrinha::cria_personalizado(
      Ponto::novo(y, x), Direcao::Leste, 15, 'o'
   );

   // rodando o jogo, e colhendo dados.
   roda_animacao_movimento_continuo(&mut tabuleiro, &mut snake); 
   // finalizando terminal...
   drop(tabuleiro);
}
