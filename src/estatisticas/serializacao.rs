
/* Alguns traits para serialização de 
 * objetos específicos, que são muito
 * comuns nos programas aqui codificados.
 */



/** Serialização de um tipo de dado dinâmico, que
 * pode aumentar ou diminuir ao longo da execução.
 */
pub trait Serializacao {
   /* serão gravados na ordem que foram
    * codificados, e, por consequência,
    * a deserialização segue a mesma 
    * regra. 
    * numa array-dinâmica, pois pode variar sempre
    * para o mesmo tipo de dado. */
   fn serializa(&self) -> Vec<u8>;

   fn deserializa(_: Vec<u8>) -> Self;
}

pub trait OutraSerializacao {
   /* serão gravados na ordem que foram
    * codificados, e, por consequência,
    * a deserialização segue a mesma 
    * regra. 
    * numa array-dinâmica, pois pode variar sempre
    * para o mesmo tipo de dado. */
   fn serializa(&self) -> Vec<u8>;

   /* pega slice de bytes e converte no dado
    * que está implementando tal trait. Se
    * a quantia de bytes passada não equivale
    * com o tamanho do tipo, o programa
    * irá "entrar em panico". */
   fn deserializa(_: &[u8]) -> Self;
}

use std::time::Duration;
use crate::{Ponto, Direcao};

// para facilitação da codificação e legibilidade.

impl OutraSerializacao for Duration {
   /* transforma o Duration numa sequência
    * "big-endian" de bytes. */
   fn serializa(&self) -> Vec<u8> {
      self.as_secs()
      .to_be_bytes()
      .to_vec()  
   }
   /* transforma de uma slice de bytes,
    * que deve ser "big-endian", pois a  --
    * serialização foi supostamente combinada
    * desta maneira --  no tipo Duration. */
   fn deserializa(bytes:&[u8]) -> Duration {
      let quantia = bytes.len();
      // se não houver a quantia certa de
      // bytes, que no caso aqui é oito.
      if quantia != 8 {
         if quantia < 8
            { panic!("bytes insufientes '{}'", quantia); }
         else
            { panic!("bytes demasiados '{}'", quantia); }
      }

      let mut buffer: [u8; 8] = [0; 8];
      for (p, b) in bytes.iter().enumerate() 
         { buffer[p] = *b; }

      /* converte num inteiro-positivo 64-bits, e
       * posteriormente no 'Duration'. */
      Duration::from_secs(u64::from_be_bytes(buffer)) 
   }
}

impl OutraSerializacao for bool {
   /* transforma bolleano no byte 1(verdadeiro),
    * ou 0(falso). */
   fn serializa(&self) -> Vec<u8>
      { if *self { vec![1u8] } else { vec![0u8] } }

   // um byte para bool.
   fn deserializa(byte:&[u8]) -> bool { 
      if byte.len() != 1
         { panic!("fragmentos de bytes inválido, é só necessário um"); }
      match byte.first() { 
         Some(&0) => false, 
         Some(&1) => true,
         Some(&(2..=u8::MAX)) =>
            { panic!("dado deserializado não válido"); }
         None =>
            { panic!("sem qualquer slice"); }
      }
   }
}

impl OutraSerializacao for Direcao {
   fn serializa(&self) -> Vec<u8> {
      // baseado no tipo:
      match *self {
         Direcao::Norte => vec![b'N'],
         Direcao::Sul => vec![b'S'],
         Direcao::Leste => vec![b'L'],
         Direcao::Oeste => vec![b'O'],
      }
   }
   // transforma mero byte na direção equivalente.
   fn deserializa(byte:&[u8]) -> Direcao {
      match byte.first() {
         Some(&b'N') =>  { Direcao::Norte }
         Some(&b'S') => { Direcao::Sul }
         Some(&b'L') => { Direcao::Leste }
         Some(&b'O') => { Direcao::Oeste }
         None | _ => 
            { panic!("byte passado inválido!"); }
      }
   }
}

impl OutraSerializacao for Ponto {
   // transforma o tipo de enum num byte.
   fn serializa(&self) -> Vec<u8> {
      // retornando dois bytes, representando cada atributo.
      return vec![self.x, self.y];
   }
   fn deserializa(bytes:&[u8]) -> Ponto {
      if bytes.len() != 2
         { panic!("só é preciso de 2 bytes."); }
      // retorna nova instância do ponto.
      return Ponto {
         // convertendo para um valor 8-bits sem sinal...
         x: bytes[0],
         // o outro valor...
         y: bytes[1]
      };
   }
}

impl OutraSerializacao for u16 {
   // transforma o tipo de inteiro-positivo 16-bits.
   fn serializa(&self) -> Vec<u8> 
      { self.to_be_bytes().to_vec() }

   // transforma 2-bytes num u16.
   fn deserializa(bytes:&[u8]) -> u16 {
      if bytes.len() != 2
         { panic!("só é preciso de 2 bytes."); }
      let array: [u8; 2] = [bytes[0], bytes[1]];
      u16::from_be_bytes(array)
   }
}
