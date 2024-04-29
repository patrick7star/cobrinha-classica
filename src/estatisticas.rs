
/**
 Aqui serão tanto coletados, como possíveis
 de armazenamento os dados gerados no jogo,
 como os dados "externos" ao jogos, tipo quantia
 de teclas pressionadas e a dimensão da janela
 de tal, e etc.
*/

// própria 'lib'.
use crate::{
   Cobrinha, Ponto, 
   Direcao, Alvos, VELOCIDADE, 
};
// biblioteca padrão do Rust:
use std::time::{Instant, Duration};
use std::fmt::{self, Display};
use std::primitive::bool;

// módulos complementares a este:
#[allow(unused)]
mod selecao;
mod serializacao;
mod utilitarios_basicos;
use utilitarios_basicos::*;
// re-exportando conteúdos para este aqui.
pub use serializacao::*;

/* O registro da cobrinha e dos bugs "devorados"
 * na seguinte ordem: 
 *    --> atual posição
 *    --> atual direção 
 *    --> comprimento 
 *    --> quantia de bugs restantes
 *    --> taxa de captura de bichinhos 
 * são importantes, pois todas grandezeas acima
 * variam ao longo do jogo.  */ 
pub type Shot = (Ponto, Direcao, u16, u8, u8);
/* dimensão, onde o primeiro elemento 
 * é a 'altura' da tela, e a segunda 
 * é sua 'lagura'.  */
type Dimensao = (u16, u16);

#[derive(Clone)]
pub struct Dados {
   // formato: altura x largura. 
   pub dimensao: Dimensao,
   // comprimento inicial da cobrinha.
   pub comprimento: u16,
   // tempo de duração do jogo.
   pub tempo_duracao: Option<Duration>,
   // taxa de captura por minuto.
   taxa_captura: u8,
   /* tupla com rastros: Posição, Direção,
    * se há um 'bug' naquele ponto e etc; 
    * em cada instante da cobrinha. Todas
    * registradas colocada numa fila. */ 
   pub fila_rastros: Vec<Shot>,
   // verifica missão cumprida.
   pub vitoria: bool,
   // quantia de bichos.
   pub total_de_bugs: u8,
   // velocidades de atualização de frames em milisegundos.
   velocidade: u16,
   // atributos auxiliares:
   cronometro: Option<Instant>,
}

impl Dados {
   pub fn gera(cobra:&Cobrinha, bugs:&Alvos, 
   dimensao:(u16, u16)) -> Self {
      Dados {
         dimensao,
         // atributos mais ou menos fixos.
         tempo_duracao: None,
         velocidade: VELOCIDADE as u16,
         vitoria: bugs.sem_alvos(),
         // valor inicial.
         total_de_bugs: (bugs.qtd_alvos_restantes() as u8),
         // inicialmente vázio.
         fila_rastros: Vec::new(), 
         taxa_captura: 0,
         // tamanho inicial da cobrinha.
         comprimento: cobra.tamanho() as u16,
         // iniciando contagem ...
         cronometro: Some(Instant::now()),
      }
   }
   
   pub fn atualiza(&mut self, cobra:&Cobrinha, bugs:&Alvos) { 
      // registrando tempo decorrido todo o jogo até aqui.
      self.tempo_duracao = Some(
         self.cronometro
         .unwrap()
         .elapsed()
      );

      // passou um frame, então registrar algo.
      // registrando cada passo dado pela cobrinha.
      self.fila_rastros.push((
         cobra.posicao(),
         cobra.sentido(),
         cobra.tamanho() as u16,
         bugs.qtd_alvos_restantes() as u8,
         self.taxa_captura
      ));

      // verifica vitória.
      if bugs.sem_alvos()
         { self.vitoria = true; }

      // registrar capturas por minuto.
      let t = self.cronometro.unwrap().elapsed().as_secs();
      if t %  60 == 0 {
         let m = self.total_de_bugs;
         let n = bugs.qtd_alvos_restantes();
         self.taxa_captura = (m - n as u8) as u8;
      }
   }
}

impl Display for Dados {
   fn fmt(&self, formatador:&mut fmt::Formatter<'_>) 
   -> fmt::Result {
      let indice = self.fila_rastros.len() - 1;
      let qtd_devorados:u8 = {
         let qi = self.fila_rastros[indice].3;
         let qf = self.total_de_bugs;
         qf - qi
      };
      // comprimento final da cobrinha.
      let cf:u16 = self.fila_rastros[indice].2;
      return write!(
         formatador,
         "\n----- INFO DO JOGO ------
         \rresultado: {}
         \rdimensão: {}x{}
         \rcaptura: {}/min
         \rduração: {}
         \rpassos dados: {}
         \rbugs devorados:{}
         \raumento: {}\n", 
         traduz(self.vitoria),
         self.dimensao.0, self.dimensao.1,
         self.taxa_captura,
         tempo_legivel(self.tempo_duracao.unwrap()),
         self.fila_rastros.len(),
         qtd_devorados,
         cf - self.comprimento
      );
   }
}


impl OutraSerializacao for Shot {
   fn serializa(&self) -> Vec<u8> {
      // acumulador de bytes.
      let mut bytes: Vec<u8> = Vec::new();
      // Ponto:
      bytes.extend_from_slice(self.0.serializa().as_slice());
      // Direção:
      bytes.extend_from_slice(self.1.serializa().as_slice());
      // Comprimento:
      bytes.extend_from_slice(&self.2.to_be_bytes()[..]);
      // Bug's devorados:
      bytes.push(self.3);
      // Taxa de captura média:
      bytes.push(self.4);
      return bytes;
   }

   fn deserializa(bytes:&[u8]) -> Shot {
      let array: [u8; 2] = [
         *bytes.get(3).unwrap(), 
         *bytes.get(4).unwrap()
      ];
      ( Ponto::deserializa(bytes.get(0..2).unwrap()),
        Direcao::deserializa(bytes.get(2..3).unwrap()),
        u16::from_be_bytes(array),
        *bytes.get(5).unwrap(), 
        *bytes.get(6).unwrap()
      )
   }
}


impl OutraSerializacao for Dimensao {
   fn serializa(&self) -> Vec<u8> {
      // pega os bytes do primeiro elemento(altura).
      let mut bytes: Vec<u8> = self.0.serializa();
      // "concatena" o segundo neste(largura).
      bytes.extend_from_slice(self.1.serializa().as_slice());
      return bytes;
   }
   fn deserializa(bytes: &[u8]) -> Dimensao {
      if bytes.len() != 4
         { panic!("não têm os 4 bytes exigidos!"); }
      ( u16::deserializa(bytes.get(0..2).unwrap()),
        u16::deserializa(bytes.get(2..4).unwrap()) )
   }
}

impl Serializacao for Dados {
   fn serializa(&self) -> Vec<u8> {
      // array-dinâmica com todos bytes a serem adicionados.
      let mut bytes:Vec<u8> = Vec::new();
      // serialização de todos atributos.
      let dimensao_altura = self.dimensao.0.to_be_bytes();
      let dimensao_largura = self.dimensao.1.to_be_bytes();
      let comprimento = self.comprimento.to_be_bytes();
      let taxa_captura = self.taxa_captura;
      let vitoria = self.vitoria.serializa();
      let total_de_bugs = self.total_de_bugs;
      let velocidade = self.velocidade.to_be_bytes();
      // até esvaziar...
      let mut fila_rastros:Vec<u8> = Vec::new();
      let qtd:u16 = self.fila_rastros.len() as u16;
      for e in self.fila_rastros.iter() {
         fila_rastros.extend_from_slice(&e.serializa()[..]);
      }
      let tempo_duracao = {
         self.tempo_duracao
         .unwrap()
         .as_secs()
         .to_be_bytes()
      };

      /* adicionando na "string de bytes", na 
       * ordem que foram codificados como dito. */
      // adicionando ao todo 4 bytes, 2 para cada.
      bytes.extend_from_slice(&dimensao_altura[..]);
      bytes.extend_from_slice(&dimensao_largura[..]);
      // adicionando mais 2 bytes.
      bytes.extend_from_slice(&comprimento[..]);
      // adicionando 8 bytes.
      bytes.extend_from_slice(&tempo_duracao[..]);
      // adicionando 1 byte.
      bytes.push(taxa_captura);
      /* adicionando ao todo o tamanho da array vezes
       * o tamanho da tupla, que têm valore estáticos
       * somando em 7 bytes. Primeiro, adiciona um 
       * inteiro positivo de 16-bits que representa
       * o total de tuplas na array.*/
      bytes.extend_from_slice(&qtd.to_be_bytes()[..]);
      bytes.extend_from_slice(fila_rastros.get(0..).unwrap());
      // só 1 byte.
      //bytes.push(vitoria);
      bytes.extend_from_slice(vitoria.as_slice());
      // 1 byte também.
      bytes.push(total_de_bugs);
      // 2 bytes.
      bytes.extend_from_slice(&velocidade[..]);
      return bytes;
   }

   fn deserializa(mut linguicao:Vec<u8>) -> Dados {
      // primeiro atributo(dimensão).
      let dimensao = Dimensao::deserializa(linguicao.drain(0..4).as_slice());
      // segundo atributo(comprimento).
      let comprimento = u16::deserializa(linguicao.drain(0..2).as_slice());
      // terceiro atributo(taxa de duração).
      let t = Duration::deserializa(linguicao.drain(0..8).as_slice());
      let tempo_duracao = Some(t);
      // quarto atributo(taxa de captura).
      let taxa_captura: u8 = linguicao.remove(0);
      // quinto atributo(fila de rastros).
      let mut qtd = u16::deserializa(linguicao.drain(0..2).as_slice());
      let mut fila_rastros: Vec<Shot> = Vec::new();
      // contabilizando remoções.
      while qtd > 0 {
         let sete_bytes = linguicao.drain(0..7);
         let tupla = Shot::deserializa(sete_bytes.as_slice());
         fila_rastros.push(tupla);
         qtd -= 1;
      }
      // sexto atributo(vitória).
      let vitoria = bool::deserializa(linguicao.drain(0..1).as_slice());
      // sétimo atributo(total de BUG's).
      let total_de_bugs = linguicao.remove(0);
      // oitavo atributo(velocidade).
      let velocidade = u16::deserializa(linguicao.drain(0..2).as_slice());
      // têm que está vázio.
      assert!(linguicao.is_empty());
      /* criando tipo de dados com todas seus
       * atributos. */
      return Self {
         dimensao,
         comprimento,
         tempo_duracao,
         taxa_captura,
         fila_rastros,
         vitoria,
         total_de_bugs,
         velocidade,
         // irrelevante.
         cronometro: None,
      };
   }
}

