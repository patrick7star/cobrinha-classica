
// biblioteca padrão:
use std::ops::AddAssign;
use std::time::Instant;
// próprio caixote.
use crate::{Ponto, Cobrinha};

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
   cronometro: Instant
}

// métodos complementares.
trait Modulo { fn abs(&self, _:u8) -> u8; }
trait CaldaPosicao { fn calda(&self) -> Ponto; }

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
         cronometro: Instant::now()
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
