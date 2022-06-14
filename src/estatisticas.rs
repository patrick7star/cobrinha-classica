
/**
 Aqui serão tanto coletados, como possíveis
 de armazenamento os dados gerados no jogo,
 como os dados "externos" ao jogos, tipo quantia
 de teclas pressionadas e a dimensão da janela
 de tal, e etc.
*/

use crate::{
   Cobrinha, Ponto, 
   Direcao, Alvos, VELOCIDADE, 
   serializacao::{Serializacao, UnicoByte}
};
use std::time::{Instant, Duration};
use std::fmt::{self, Display};
use std::primitive::bool;

/* O registro da cobrinha e dos bugs "devorados"
 * na seguinte ordem: atual posição; 
 * atual direção; comprimento; quantia de bugs
 * restantes; taxa de captura de bichinhos. */ 
pub type Shot = (Ponto, Direcao, u16, u8, u8);

pub struct Dados {
   // formato: altura x largura. 
   dimensao: (u16, u16),
   // comprimento inicial da cobrinha.
   comprimento: u16,
   // tempo de duração do jogo.
   tempo_duracao: Option<Duration>,
   // taxa de captura por minuto.
   taxa_captura: u8,
   /* tupla com rastros: Posição, Direção,
    * se há um 'bug' naquele ponto e etc; 
    * em cada instante da cobrinha. Todas
    * registradas colocada numa fila. */ 
   fila_rastros: Vec<Shot>,
   // verifica missão cumprida.
   vitoria: bool,
   // quantia de bichos.
   total_de_bugs: u8,
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

// pega o booleano e traduz em termos de vitória/derrota.
fn traduz(resultado:bool) -> &'static str {
   match resultado {
      true => "VENCEU",
      false => "PERDEU"
   }
}

impl Display for Dados {
   fn fmt(&self, formatador:&mut fmt::Formatter<'_>) -> fmt::Result {
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

// implementando serialização do tipo 'Duration'.
impl Serializacao for Duration {
   fn serializa(&self) -> Vec<u8> {
      self.as_secs()
      .to_be_bytes()
      .to_vec()  
   }
}

// estruturas quebra-galhos.
struct Boleano(bool);
struct Duracao(Duration);
struct Tupla(Shot);

impl Duracao {
   pub fn deserializa(bytes:[u8; 8]) -> Duration 
      { Duration::from_secs(u64::from_be_bytes(bytes)) }
}

impl UnicoByte for Direcao {
   fn serializa(&self) -> u8 {
      // baseado no tipo:
      match *self {
         Direcao::Norte => b'N',
         Direcao::Sul => b'S',
         Direcao::Leste => b'L',
         Direcao::Oeste => b'O',
      }
   }
}

impl Direcao {
   // transforma mero byte na direção equivalente.
   pub fn deserializa(byte:u8) -> Direcao {
      if (byte as char) == 'N' 
         { Direcao::Norte }
      else if (byte as char) == 'S' 
         { Direcao::Sul }
      else if (byte as char) == 'L' 
         { Direcao::Leste }
      else { Direcao::Oeste }
   }
}

impl Serializacao for Ponto {
   // transforma o tipo de enum num byte.
   fn serializa(&self) -> Vec<u8> {
      // retornando dois bytes, representando cada atributo.
      return [self.x, self.y].to_vec();
   }

}
impl Ponto {
   // 2 bytes num Ponto.
   pub fn deserializa(bytes:[u8; 2]) -> Ponto {
      // retorna nova instância do ponto.
      return Ponto {
         // convertendo para um valor 8-bits sem sinal...
         x: bytes[0],
         // o outro valor...
         y: bytes[1]
      };
   }
}

impl UnicoByte for bool {
   fn serializa(&self) -> u8 
      { if *self { 1 } else { 0 } }
}

impl Boleano {
   // um byte para bool.
   pub fn deserializa(byte:u8) -> bool 
      { match byte { 0 => false, _ => true } }
}

impl Serializacao for Shot {
   fn serializa(&self) -> Vec<u8> {
      let bytes_i = self.0.serializa();
      let bytes_ii = self.2.to_be_bytes();
      return [
         bytes_i[0], bytes_i[1],
         self.1.serializa(),
         bytes_ii[0], bytes_ii[1],
         self.3,
         self.4
      ].to_vec();
   }
}

impl Tupla {
   fn deserializa(bytes:[u8; 7]) -> Shot {
      let bytes_i = [bytes[0], bytes[1]];
      let bytes_ii = [bytes[3], bytes[4]];
      (
         Ponto::deserializa(bytes_i), 
         Direcao::deserializa(bytes[2]),
         u16::from_be_bytes(bytes_ii),
         bytes[5],
         bytes[6]
      )
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
      bytes.push(vitoria);
      // 1 byte também.
      bytes.push(total_de_bugs);
      // 2 bytes.
      bytes.extend_from_slice(&velocidade[..]);
      return bytes;
   }
}

impl Dados {
   pub fn deserializa(mut linguicao:Vec<u8>) -> Dados {
      // primeiro atributo(dimensão).
      let bytes_i:[u8; 2] = [
         linguicao.remove(0),
         linguicao.remove(0)
      ];
      let bytes_ii:[u8; 2] = [
         linguicao.remove(0),
         linguicao.remove(0)
      ];
      let dimensao = (
         u16::from_be_bytes(bytes_i),
         u16::from_be_bytes(bytes_ii)
      );
      // segundo atributo(comprimento).
      let bytes:[u8; 2] = [
         linguicao.remove(0),
         linguicao.remove(0)
      ];
      let comprimento = u16::from_be_bytes(bytes);
      // terceiro atributo(taxa de duração).
      let bytes:[u8; 8] = [
         linguicao.remove(0),
         linguicao.remove(0),
         linguicao.remove(0),
         linguicao.remove(0),
         linguicao.remove(0),
         linguicao.remove(0),
         linguicao.remove(0),
         linguicao.remove(0)
      ];
      let td = Duracao::deserializa(bytes);
      let tempo_duracao = Some(td);
      // quarto atributo(taxa de captura).
      let taxa_captura: u8 = linguicao.remove(0);
      // quinto atributo(fila de rastros).
      let bytes:[u8; 2] = [
         linguicao.remove(0),
         linguicao.remove(0)
      ];
      let mut qtd: u16 = u16::from_be_bytes(bytes);
      let mut fila_rastros: Vec<Shot> = Vec::new();
      // contabilizando remoções.
      while qtd > 0 {
         let bytes: [u8; 7] = [
            linguicao.remove(0),
            linguicao.remove(0),
            linguicao.remove(0),
            linguicao.remove(0),
            linguicao.remove(0),
            linguicao.remove(0),
            linguicao.remove(0)
         ];
         let tupla:Shot = Tupla::deserializa(bytes);
         fila_rastros.push(tupla);
         qtd -= 1;
      }
      // sexto atributo(vitória).
      let byte = linguicao.remove(0);
      let vitoria = Boleano::deserializa(byte);
      // sétimo atributo(total de BUG's).
      let byte = linguicao.remove(0);
      let total_de_bugs = byte;
      // oitavo atributo(velocidade).
      let bytes:[u8; 2] = [
         linguicao.remove(0),
         linguicao.remove(0)
      ];
      let velocidade = u16::from_be_bytes(bytes);
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

// pega o tempo em segundos e transforma
// numa legitíma string, informando o 
// tempo de forma legível. O range aqui
// não é muito amplos, pois o jogo sempre
// gera algo nestes intervalo(minutos e 
// segundos).
fn tempo_legivel(t:Duration) -> String {
   let tempo = t.as_secs_f32();
   if tempo > 60.0 
      { format!("{:0.1} min", tempo / 60.0) }
   else
      { format!("{} seg", tempo as u8) }
}
