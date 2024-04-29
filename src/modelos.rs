
// biblioteca padrão do Rust:
use std::fmt::{Display, Debug, Formatter, Result as R};
use std::cmp::PartialEq;
use std::ops::AddAssign;


// direções de movimentos.
#[derive(Copy, Clone, Debug)]
pub enum Direcao { Norte, Oeste, Leste, Sul }

impl PartialEq for Direcao {
   // verifica se são iguais.
   fn eq(&self, other:&Self) -> bool {
      // qual direção o argumento da direita retorna.
      let direita:char = match *self {
         Direcao::Norte => 'N',
         Direcao::Leste => 'L',
         Direcao::Sul => 'S',
         Direcao::Oeste => 'O',
      };
      // qual direção o argumento da esquerda retorna.
      let esquerda:char = match other {
         Direcao::Norte => 'N',
         Direcao::Leste => 'L',
         Direcao::Sul => 'S',
         Direcao::Oeste => 'O',
      };
      // ele são iguais?   
      return direita == esquerda;
      /*
      match *self {
         other => true,
         _ => false,
      }*/

   }
   // retorna a negação do primeiro método.
   fn ne(&self, other:&Self) -> bool {
      ! self.eq(other)
   }
}

// implementado recíproca.
impl Direcao {
   pub fn oposto(&self) -> Self {
      match *self {
         Direcao::Norte => Direcao::Sul,
         Direcao::Oeste=> Direcao::Leste,
         Direcao::Leste=> Direcao::Oeste,
         Direcao::Sul=> Direcao::Norte
      }
   }
}

/* estrutura para localizar qualquer coisa 
 * na tela, ou a abstração de tela.  */
#[derive(Copy, Clone)]
pub struct Ponto { pub y:u8, pub x: u8 }

impl Ponto {
   pub fn novo(y: u8, x: u8) -> Self
      { Self { y, x } }
}

use std::default::Default;

impl Default for Ponto {
   fn default() -> Self 
      { Ponto::novo(1, 1) }
}

impl Display for Ponto {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> R{
      return write!(
         formatador, 
         "linha={0} coluna={1}", 
         self.y, self.x
      );
   }
}

impl Debug for Ponto {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> R{
      return write!(
         formatador, 
         "y={0} x={1}", 
         self.x, self.y
      );
   }
}

impl PartialEq for Ponto {
   fn eq(&self, ponto:&Ponto) -> bool {
      self.y == ponto.y && self.x == ponto.x
   }
   fn ne(&self, ponto:&Ponto) -> bool {
      return !(self.eq(ponto));
   }
}

// objeto que se movimenta na tela. 
#[derive(Clone, Copy)]
pub struct Seta {
   // onde o direcionador está tendendo...
   pub sentido:Direcao,
   // atual posição na tela(y,x).
   pub posicao:Ponto,
   // símbolo do direcionador.
   pub forma:char,
   // sentido marcado após alteração do primeiro sentido.
   pub antiga_posicao:Ponto,
}

impl Seta {
   // cria joystick...
   pub fn cria(dir:Direcao, linha:u8, 
   coluna:u8, simbolo:char) -> Seta {
      // retorna estrutura criada.
      Seta { 
         sentido: dir,
         posicao:Ponto::novo(linha,coluna),
         forma: simbolo,
         //antiga_posicao:Ponto{y:0, x:0},
         antiga_posicao: Default::default()
      }
   }

   // faz um movimento...
   pub fn faz_passo(&mut self, novo:Direcao) {
      self.sentido = novo;
      self.antiga_posicao.y = self.posicao.y;
      self.antiga_posicao.x = self.posicao.x;
      match self.sentido {
         Direcao::Norte => { 
            if self.posicao.y >= 1
               { self.posicao.y -= 1; }
         },
         Direcao::Sul => {self.posicao.y += 1;}, 
         Direcao::Oeste => { 
            if self.posicao.x >= 1
               { self.posicao.x -= 1; }
         },
         Direcao::Leste => {self.posicao.x += 1;}
      };
   }
}

/* uma sequência de objetos "setas", construindo
 * assim a cobrinha. */
pub struct Cobrinha {
   pub cabeca:Seta,
   pub membros:Vec<Seta>
}

impl Cobrinha {
   /* cria instância da 'Cobrinha', partindo de 
    * um dado 'Ponto' */
   pub fn criar(posicao:Ponto) -> Cobrinha {
      // sentido inicial da cobrinha.
      let sentido_inicial = Direcao::Norte;
      // guiador do resto do corpo.
      let  guia = Seta::cria(
         sentido_inicial, 
         posicao.y, 
         posicao.x, '#'
      );
      // lista contendo todos membros.
      let mut corpo:Vec<Seta> = Vec::new();
      // criação de cinco membros...
      
      for i in 1..=5 {
         // contração para melhor legibilidade.
         let si = sentido_inicial;
         match sentido_inicial {
            Direcao::Leste => {
               corpo.push(Seta::cria(si, posicao.y, posicao.x-i, '#'));
            },
            Direcao::Oeste => {
               corpo.push(Seta::cria(si, posicao.y, posicao.x+i, '#'));
            },
            Direcao::Sul => {
               corpo.push(Seta::cria(si, posicao.y-i, posicao.x, '#'));
            },
            Direcao::Norte => {
               corpo.push(Seta::cria(si, posicao.y+i, posicao.x, '#'));
            }
         };
      }
     
      // retorno da cobrinha.
      Cobrinha { cabeca:guia, membros:corpo}
   }

   // move toda 'Cobrinha' um passo na 'Direcao' dada.
   pub fn mover(&mut self, novo:Direcao) {
      let mut aux_i:Direcao; 
      // inicialização com qualquer valor pois o 
      // compilador não aceita valores não inicializados.
      let mut aux_ii:Direcao = Direcao::Norte;
      // para alternar a leitura e gravação das
      // variáveis "aux_i" e "aux_ii".
      let mut interruptor = true;
      /* basicamente a implementação do algoritmo 
       * alternante acima, porém de forma genérica
       * para que possa abordar 'N' casos. */
      aux_i = self.cabeca.sentido;
      self.cabeca.faz_passo(novo);

      for p in 0..(self.membros.len()+1)-2 {
         if interruptor {
            aux_ii = self.membros[p].sentido;
            self.membros[p].faz_passo(aux_i);
         }
         else {
            aux_i = self.membros[p].sentido;
            self.membros[p].faz_passo(aux_ii);
         }
         interruptor = !interruptor;
      }

      // o último caso independente do loop,
      // que apenas lê o último sentido gravado.
      let q = self.membros.len()-1;
      if interruptor 
         { self.membros[q].faz_passo(aux_i); }
      else 
         { self.membros[q].faz_passo(aux_ii); }
   }
   
   // computa atual posição da cabeça da 'Cobrinha'.
   pub fn posicao(&self) -> Ponto 
      { return self.cabeca.posicao; }

   // computa quantia de membros da 'Cobrinha'.
   pub fn tamanho(&self) -> usize 
      { return self.membros.len() + 1; }

   // sentido atual da cobrinha.
   pub fn sentido(&self) -> Direcao
      { self.cabeca.sentido }
}

// adicionando um novo modo de adicionar novo membro.
impl AddAssign<usize> for Cobrinha {
   // implementando a adição.
   fn add_assign(&mut self, mut qtd:usize) {
      /* Processo de adicionar novo membro. Separando
       * assim, pois fatoração melhora a legibilidade
       * do código. */
      fn adiciona_novo_membro(cobra:&mut Cobrinha) {
         // obtendo o último membro da fila no momento.
         let ultimo_mbr = cobra.membros[cobra.membros.len()-1];
         // obtendo todas suas propriedades.
         let dir_ultimo_mbr = ultimo_mbr.sentido;
         let (l, c) = {
            /* reposicionando onde será criado baseado no
             * sentido e posição do ex-último-membro 
             * até o momento. */
            match ultimo_mbr.sentido {
               Direcao::Norte =>
                  (ultimo_mbr.posicao.y+1, ultimo_mbr.posicao.x),
               Direcao::Sul=>
                  (ultimo_mbr.posicao.y-1, ultimo_mbr.posicao.x),
               Direcao::Leste=>
                  (ultimo_mbr.posicao.y, ultimo_mbr.posicao.x-1),
               Direcao::Oeste=>
                  (ultimo_mbr.posicao.y, ultimo_mbr.posicao.x+1),
            }
         };
         // criando um novo membro baseado nisso.
         let novo_mbr = Seta::cria(dir_ultimo_mbr,l, c, '#');
         // no final da fila.
         cobra.membros.push(novo_mbr);
      }

      // executando a adição 'qtd' vezes.
      while qtd > 0 {
         // executa a adição de UM novo membro.
         adiciona_novo_membro(self);
         // contabilizando execuções.
         qtd -= 1;
      }
   }
}

