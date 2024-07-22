
/* Cobrinhas se movimento automamente aqui para se manipular seus limites.
 */

use cobrinha_classica::*;
use std::time::{Duration, Instant};
use std::convert::TryInto;
use pancurses::{Window, napms};
use std::collections::VecDeque;

/* A atual direção, quantas vezes repetir isso. */
type UnidadeMovimento = (Direcao, u8);
type Fila = VecDeque<UnidadeMovimento>;
const VELOCIDADE_IDEAL: u16 = VELOCIDADE as u16 / 2; // milisegundos.

struct Trajetoria {
   // Próximos movimentos à fazer.
   fila_de_passos: Fila,

   /* Cada passo planejado acima é enfileirado nesta fila, assim cada 
    * mini trajeto, é realizado quando esta fila esgota de direções.*/
   atual_ciclo: VecDeque<Direcao>,

   // Tempo de disparo para fazer cada movimento(em milisegundos).
   pub velocidade: u16,

   /* Loop infinito? Após acabar todas direções, começar o ciclo de novo,
    * porém da atual posição, e não de onde se partiu. */
   sem_fim: bool
}

impl Trajetoria {
   pub fn fazer(fila_de_passos: Fila) -> Self { 
   /* Opção que apenas recebe as trajeitórias configuradas, todos os 
    * demais é feito como padrão. */ 
       Self { 
          fila_de_passos, velocidade: VELOCIDADE_IDEAL, 
          sem_fim: true, atual_ciclo: VecDeque::new()
       } 
   }

   fn realiza_mini_trajeto(&mut self, obj: &mut Cobrinha) {
      if let Some(dir) = self.atual_ciclo.pop_front()
         { obj.mover(dir); napms(self.velocidade as i32); }
   }

   pub fn seguir(&mut self, obj: &mut Cobrinha) {
      if !self.atual_ciclo.is_empty()
         { self.realiza_mini_trajeto(obj); return (); }

      // Criando atual ciclo...
      if let Some(sentido_seguinte) = self.fila_de_passos.pop_front() { 
         let (dir, qtd) = sentido_seguinte;

         // Repete tal direção 'n' vezes.
         for _ in 1..=qtd 
            { self.atual_ciclo.push_back(dir); }
         /* Adiciona movimentos no final da fila novamente, para ficar em 
          * loop na mesma trajetória. Tal só é validada, se a opção de 
          * 'ciclo' foi ativada. */
         if self.sem_fim
            {  self.fila_de_passos.push_back((dir, qtd)); }
      }
   }
}

fn anima_a_cobrinha_com_as_seguintes_coordenadas<J>(board: &mut J, 
  snake:&mut Cobrinha, mut movimento_a_realizar: Trajetoria, 
  tempo_de_animacao: Duration) where J: AsMut<Window> 
{
   let janela = board.as_mut();
   #[allow(non_snake_case)]
   let (Y, X) = (
      janela.get_max_y() as u8,
      janela.get_max_x() as u8
   );
   let dimensao = (Y, X);
   let timer = Instant::now();

   while timer.elapsed() < tempo_de_animacao {
      movimento_a_realizar.seguir(snake);
      teletransporta_cobrinha(dimensao, snake);
      plota_cobrinha(janela, &snake);
      
      // atualização de frame do jogo.
      remenda_borda_da_tela(janela);
      janela.refresh();
   }
}

#[test]
fn se_teletransportando_entre_as_barreiras() {
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
      Ponto::novo(y, x), Direcao::Leste, 15, '#'
   );
   let toda_sua_trajetoria = Trajetoria::fazer(
      Fila::from([
         (Direcao::Sul, 65), (Direcao::Oeste, 55), (Direcao::Norte, 27),
         (Direcao::Leste, 14), (Direcao::Norte, 40), (Direcao::Oeste, 77)
      ])
   );

   // rodando o jogo, e colhendo dados.
   anima_a_cobrinha_com_as_seguintes_coordenadas(
      &mut tabuleiro, &mut snake,
      toda_sua_trajetoria, 
      Duration::from_secs(35)
   ); 
   // finalizando terminal...
   drop(tabuleiro);
}

#[test]
fn cobrinha_em_zig_zag() {
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
   let movimentacao_em_zig_zag = Trajetoria::fazer(
      Fila::from([
         (Direcao::Sul, 5), (Direcao::Oeste, 35), (Direcao::Norte, 17),
         (Direcao::Leste, 4),
         // Começo do zig-zag...
         (Direcao::Sul, 3), (Direcao::Leste, 3),
         (Direcao::Sul, 3), (Direcao::Leste, 3),
         (Direcao::Sul, 3), (Direcao::Leste, 3),
         (Direcao::Sul, 3), (Direcao::Leste, 2),
         (Direcao::Sul, 3), (Direcao::Leste, 2),
         (Direcao::Sul, 2), (Direcao::Leste, 3),
         (Direcao::Sul, 2), (Direcao::Leste, 3),
         (Direcao::Sul, 1), (Direcao::Leste, 2),
         (Direcao::Sul, 1), (Direcao::Leste, 2)
      ])
   );

   // rodando o jogo, e colhendo dados.
   anima_a_cobrinha_com_as_seguintes_coordenadas(
      &mut tabuleiro, &mut snake,
      movimentacao_em_zig_zag, 
      Duration::from_secs(35)
   ); 
   // finalizando terminal...
   drop(tabuleiro);
}
