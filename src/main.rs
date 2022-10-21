
// bibliotecas externas:
extern crate rand;
extern crate pancurses;
use pancurses::{
   start_color, initscr, endwin, napms,
   COLOR_WHITE,COLOR_YELLOW, COLOR_GREEN,
   init_color, COLOR_PAIR, init_pair, curs_set
};

// importando da minha biblioteca:
use cobrinha_classica::*;

const FUNDO:i16 = 99;
const BORDA:i16 = 98;
const CORPO:i16 = 97;

// execução de testes...
fn main() {
   // iniciando terminal...
   let tabuleiro = initscr();
   start_color();
   tabuleiro.nodelay(true);
   tabuleiro.keypad(true);
   curs_set(0);

   /* paleta de cores(
    * background: #9BBA5A
    * borda: #272F17
    * cobrinha: #2B331A
    */
   // criando novas cores.
   init_color(FUNDO, 204, 255, 158);
   init_color(BORDA, 0, 0, 0);
   init_color(CORPO, 0, 0, 0); 
   init_pair(3, COLOR_GREEN, FUNDO);
   init_pair(11, CORPO, FUNDO);
   init_pair(12, COLOR_YELLOW, FUNDO);
   init_pair(13, COLOR_WHITE, FUNDO);
   init_pair(14, BORDA, FUNDO);
   tabuleiro.bkgd(' ' as u32 | COLOR_PAIR(11));
   tabuleiro.border(0, 0, 0, 0, 0, 0, 0, 0 | COLOR_PAIR(14));

   // obtendo dimensão do terminal...
   let (linhas, colunas) = tabuleiro.get_max_yx();
   // obtendo a posição do meio da tela.
   let meio = Ponto { 
      y:(linhas / 2) as u8, 
      x:(colunas / 2) as u8 
   };

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
   let dados_do_jogo = roda_jogo(&tabuleiro, &mut cobra, &mut metas); 

   // finalizando terminal...
   napms(700);
   endwin();

   // visualizando informação...
   println!("{}", dados_do_jogo);

   // salvando o resultado ...
   match salva_no_bd(dados_do_jogo.serializa()) {
      Ok(_) => { println!("partida registrada com sucesso."); }
      Err(erro) => { println!("ERRO:[{}]", erro); }
   };

   // criando links ao executável.
   //linka_executaveis("cobrinha");
}
