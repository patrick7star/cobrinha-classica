/* 
 * cria alvos especializados para serem plotados,
 * e quando devorados, somem. O tipo de dado responde
 * a chamada de se há ainda bichinhos.
 */

// meus módulos:
use super::computacoes::{pontos_aleatorios_consertado};
use crate::{Ponto};


// a quantia de pontos é decidido conjuntamente com
// a constante de outro modo, geral em todo o jogo.

pub struct Alvos {
   // todos pontos que a cobrinha terá que alçar
   // numa array.
   pub posicoes: Vec<Ponto>,
   // todos pontos que ela já atingiu, índice da array.
   atingidos: Vec<Ponto>,
   // caractére representando o bicho/alvo.
   pub forma: char,
   // atual alvo.
   atual_alvo: Option<Ponto>,
   // quantia inicial de bichos.
   total: u16
}

impl Alvos {
   // cria os alvos, dado uma delimitação de linhas e colunas.
   pub fn cria(linhas:u8, colunas:u8, qtd:u16) -> Alvos {
      // gera 'n' pontos gerados aleatoriamente.
      let mut metas = pontos_aleatorios_consertado(linhas, colunas, qtd);
      // atual alvo:
      let aa = metas.remove(metas.len() / 2);

      // cria alvo.
      Alvos{
         posicoes: metas,
         atingidos: Vec::new(),
         forma:'&',
         atual_alvo: Some(aa),
         total: qtd
      }
   }

   pub fn sem_alvos(&self) -> bool {
      /* se houver a mesma quantia de índices
       * da array, então pegou todos bichos. */
      self.posicoes.is_empty() && 
      self.atingidos.len() as u16 == self.total
   }

   pub fn captura_valido(&mut self, local:Ponto) -> bool {
      let ponto = match self.atual_alvo {
         Some(p) => p,
         None => { return false; }
      };
      if local == ponto { 
         // captura antes ...
         self.atingidos.push(local); 
         // colocando novo alvo.
         if self.posicoes.is_empty() 
            { self.atual_alvo = None; }
         else {
            let meio = self.posicoes.len() / 2;
            self.atual_alvo = Some(self.posicoes.remove(meio));
         }
         // confirma "pergunta".
         return true;
      }
      return false;
   }

   pub fn a_mostrar(&self) -> Option<Ponto>
      { return self.atual_alvo; }

   pub fn qtd_alvos_restantes(&self) -> u16 { 
      let q = self.total; 
      let p = self.atingidos.len() as u16;
      return q - p;
   }
}

