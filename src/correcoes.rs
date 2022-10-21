
// biblioteca padrão:
use std::ops::AddAssign;

// próprio caixote.
use crate::{Ponto, Direcao, Cobrinha};

// biblioteca externa:
extern crate pancurses;
use pancurses::{ Window, A_REVERSE, napms };

/* Um dilutor do crescimento da Cobrinha
 * para não colidir com a parede, pelo menos
 * não tão frequente. */
pub struct Dilutor {
   // pilha de acrescimos a colocar.
   pilha: Vec<u8>,
   // atual acrescimo trabalhado.
   atual: Option<u8>,
   // seus limites impostos.
   dimensao: (Ponto, Ponto),
   // auxiliares:
   //cronometro: Instant
}

// métodos complementares.
trait Modulo 
   { fn abs(&self, _:u8) -> u8; }
trait CaldaPosicao 
   { fn calda(&self) -> Ponto; }

impl CaldaPosicao for Cobrinha {
   // retorna a posição da calda dela.
   fn calda(&self) -> Ponto {
      let ultimo = self.membros.len() - 1;
      let membro = self.membros[ultimo];
      return membro.posicao;
   }
}

impl Modulo for u8 {
   fn abs(&self, x:u8) -> u8 {
      if x > *self { x - *self }
      else { *self - x }
   }
}

impl Dilutor {
   // método construtor.
   pub fn instancia(d:(Ponto, Ponto))-> Self {
      // criando istância...
      Dilutor { 
         dimensao: d,
         pilha: Vec::new(), 
         atual:None, 
         //cronometro: Instant::now()
      }
   }

   pub fn pode_aumentar(&mut self, objeto: &Cobrinha) -> bool {
      // tira um da pilha para aumento.
      if self.atual == None || self.atual.unwrap() == 0
         { self.atual = self.pilha.pop(); }

      // verificando se está disponível para aumento.
      let e_possivel_crescer: bool = {
         // fronteiras da tela.
         let (lx, ly) = (
            self.dimensao.1.x - 1, 
            self.dimensao.1.y - 1
         );
         // posição da calda.
         let pc = objeto.calda();
         pc.x.abs(lx) >= 1 && pc.y.abs(ly) >= 1
      };

      /* apenas põe um membro na cobrinha se 
       * for possível, até o mais, fica em espera
       * o incremento. */
      match self.atual {
         Some(t) => {
            if e_possivel_crescer {
               // decrementa um adicionado.
               self.atual = Some(t-1);
               // permite o aumento.
               return true
            }
            // fora do limite, sem chance!
            false
         },
         // sem permisão para aumentar.
         None =>  false 
      }
   }
}
// adicionando um novo modo de adicionar novo membro.
impl AddAssign<u8> for Dilutor {
   // implementando a adição.
   fn add_assign(&mut self, qtd:u8) 
      { self.pilha.push(qtd) }
}

/* animação que pauso por um instante, para 
 * que se possa situar-se no jogo. A cobrinha
 * arranjada é mostrada. */
pub fn introducao(janela: &Window) {
   // dimensão da tela.
   let colunas = janela.get_max_x();
   // núcleo da mensagem.
   let mensagem = String::from("O jogo inicia em ...");

   // computa o ponto do meio.
   let (recuo, lin, col):(i32, i32, i32) = (
      mensagem.len() as i32 / 2 + 1, 
               3,
      (colunas / 2) as i32
   );

   // escrevendo ...
   janela.mv(lin, col - (recuo + 5));
   janela.color_set(13);
   janela.attrset(A_REVERSE);
   janela.addstr(mensagem.as_str());
   janela.addch(' ');
   // escrevendo contagem...
   for k in 0..=5 {
      // contagem está em ...
      let t = (5 - k).to_string();
      janela.addstr(t.as_str());
      janela.addch(' ');
      // tempo para próxima contagem..
      napms(700);
      janela.refresh();
   }
   janela.attroff(A_REVERSE);
   // limpa linha após mensagem colocada.
   janela.mv(lin, 1);
   janela.clrtoeol();
}

/* Encaracola cobrinha para quê não inicie
 * o jogo com a calda quebrando os limites
 * da tela. */
pub fn encaracola(cobra:&mut Cobrinha) {
   let t: f32 = cobra.tamanho() as f32;
   let dirs = [
      Direcao::Norte,
      Direcao::Leste,
      Direcao::Sul,
      Direcao::Oeste
   ];
   /* de onde vêm tal fórmula:
    * a cobrinha dá 1 passo ao 
    * norte; depois 2 passo ao Oeste;
    * então 3 passos ao Sul; 4 passos
    * ao Leste; 5 passos ao... Norte
    * outra vez; e etc. Isso até enrolá
    * todo seu corpo de comprimento 't'.
    * Portanto, os 'n' passos crescentes 
    * dados de modo circular têm que somar 
    * (1+2+3+...+n) menor que o comprimento
    * 't' dela. Assim, o melhor 'n' é o
    * valor da soma aritmética que  bate 
    * ou fica no limite de 't'. A  seguinte 
    * inequação: 1+2+3+...+n < t. Reduzindo
    * ela a um equivalente mais fácil de
    * resolução chegamos no seguinte:
    *       n^2 + n -2t < 0
    */
   let delta = 1.0 + 9.0 * t;
   let n = (-1.0 + delta.sqrt()) / 2.0;

   for k in 1..=(n as usize) {
      for _ in 1..=k 
         { cobra.mover(dirs[k % 4]); }
   }
}
