/*!
 Uma instância só para janela. Faz tudo que a antiga faz, mas compacta o 
 código por motivos de legibilidade e refatoração.
 Aqui também terá a derivada, que cuida só do processo de codificação, ou 
 debug.
*/

// bibliotecas externas:
extern crate pancurses;
use pancurses::{
   start_color, initscr, endwin, napms,
   COLOR_WHITE,COLOR_YELLOW, COLOR_GREEN,
   init_color, COLOR_PAIR, init_pair, curs_set,
   Window
};

// biblioteca padrão do Rust:
use std::time::Duration;
use std::convert::TryInto;

const FUNDO:i16 = 99;
const BORDA:i16 = 98;
const CORPO:i16 = 97;


pub struct Tela {
   janela: Window,
   /* taxa de atualização da tela. */
   taxa: Duration,
   /* tempo que será exibido até o 'encerramento' 
    * for conclúido. */
   exibicao_final: Duration,
   /* se mostra ou não o rótulo dado no canto
    * da janela. */
   marca_dagua: Option<&'static str>,
   // total de limpezas de telas realizadas.
   ciclos: u16,
   altura: i32,
   largura: i32
}

impl Tela {
   #[allow(clippy::identity_op)]
   pub fn nova(termino: Duration, taxa: Duration,
     rotulo: Option<&'static str>) -> Result<Self, &str>
   {
      // meio segundo de mínimo, e máximo até 15.
      let minimo = Duration::from_secs_f32(0.5);
      let maximo = Duration::from_secs(15);
      if termino < minimo ||  termino > maximo
         { return Err("tempo de 'término' incompátivel"); }
      /* a taxa de atualização pode variar apenas entre
       * um terço à três segundos. */
      let minimo = Duration::from_secs_f32(0.3);
      let maximo = Duration::from_secs_f32(3.2);
      if taxa < minimo ||  taxa > maximo
         { return Err("'atualização de tela' incompátivel"); }
      // iniciando janela ...
      let tabuleiro = initscr();
      // configurando-a ...
      start_color();
      tabuleiro.nodelay(true);
      tabuleiro.keypad(true);
      curs_set(0);

      /* paleta de cores(
       * background: #9BBA5A
       * borda: #272F17
       * cobrinha: #2B331A
       */
      // criando novas cores e pitando fundo e borda.
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
      tabuleiro.refresh();
      let (altura, largura) = tabuleiro.get_max_yx();

      // criando instância...
      Ok(Self {
         janela: tabuleiro, taxa,
         exibicao_final: termino,
         ciclos: 0, marca_dagua: rotulo,
         altura, largura
      })
   }
   pub fn atualiza(&mut self) {
      // contando o total de vezes...
      self.ciclos += 1;
      // se houver alguma marca d'agua, então desenha-lá.
      if let Some(texto) = self.marca_dagua {
         let (y, x) = self.janela.get_max_yx();
         let t = texto.len() as i32;
         self.janela.mvaddstr(y-2, x-(t+3), texto);
      }
      self.janela.refresh();
      // pausa para a limpeza.
      napms(self.taxa.as_millis().try_into().unwrap());
      self.janela.erase();
   }
   pub fn dimensao(&self) -> (i32, i32)
      { (self.altura, self.largura) }
}

// fecha janela "certinho", quando sai.
impl Drop for Tela {
   fn drop(&mut self) {
      let t = self.exibicao_final.as_millis();
      napms(t.try_into().unwrap());
      endwin();
      println!(
         "a interface gráfica foi terminada com sucesso!
         \ratualização de quadros: {}", self.ciclos
      );
   }
}

/* permite tirá referência da janela 
 * interna da estrutura, sendo que está 
 * permite um monte de funções nela. */
impl AsMut<Window> for Tela {
   fn as_mut(&mut self) -> &mut Window
      { &mut self.janela }
}

#[allow(clippy::identity_op)]
pub fn remenda_borda_da_tela(janela: &mut Window) {
   /* Como a cobrinha se choca e atravessa as barreiras, mesmo quando perde,
    * o desenho representando ela é sobre posto, logo tal função conserta
    * isso apenas redesenhando a borda em cada frame lançado. */
   janela.border(0, 0, 0, 0, 0, 0, 0, 0);
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn instanciacaoBasica() {
      let mut tela = Tela::nova(
         Duration::from_secs(5),
         Duration::from_secs_f32(0.6),
         Some("modo debug")
      ).expect("instancia não funcionou");
      let exemplo = "i'll wait for you for thousands of years
      \rdarling, you're the only exception, you are the only
      \rexception!!! Maybe i know something, we've got so
      \rkeep the confortable... distance. Because not through
      \ryou, you're are the only exception.
      \rI miss you. I never found sway before, remind of you
      \rin close left, smelt like you, when you are away,
      \r when you're gone, piss out my heart miss you,
      \rwhen you're gone, get me through, make okay that
      \ri miss you, i know you were, everything my heart is so.
      \ri need you here, made okay that i miss you.";
      let mut concatenador = String::new();
      let mut linha = 5i32;
      for (q, palavra) in exemplo.split_whitespace().enumerate() {
         if q % 9 != 0 {
            concatenador.push_str(palavra);
            concatenador.push(' ');
         } else {
            concatenador.clear();
            linha += 1;
         }
         // pega instância da 'janela' interna.
         tela.as_mut().mvaddstr(linha, 10, concatenador.as_str());
         tela.atualiza();
      }
   }
}
