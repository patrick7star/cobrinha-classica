
/** 
  Aqui ficarão as funções que recuperam/e
  gravam os dados gerados, estes que são 
  gerados durante a partida.
*/

// biblioteca do Rust:
use std::io::{Read, Write};
use std::fs::{OpenOptions, File};

use crate::{Dados, Serializacao};

type Confirmacao = Result<(), &'static str>;
type Bytes = Result<Vec<Dados>, &'static str>;

// nome do arquivo.
const BD:&'static str = "partidas.dat";

/// salva a string de bytes no disco.
pub fn salva_no_bd(dados:Vec<u8>) -> Confirmacao {
   // abre arquivo para "anexação de dados".
   let mut arquivo:File = {
      OpenOptions::new()
      .append(true)
      .create(true)
      .open("partidas.dat")
      .unwrap()
   };

   // total de bytes vai primeiro.
   let total_de_bytes: u64 = dados.len() as u64;
   let bytes = total_de_bytes.to_be_bytes();
   arquivo.write(&bytes[..]).unwrap();

   // agora os dados do jogo em sí.
   match arquivo.write(&dados[..]) {
      Ok(qtd) => 
         { println!("gravado {} bytes", qtd); Ok(()) }
      Err(_) => 
         { Err("erro ao gravar!!!") }
   }
}

// converte um iterador numa array de 8 elementos.
fn vetor_para_array(mut vetor:Vec<u8>) -> [u8; 8] {
   let mut array: [u8; 8] = [0; 8];
   for (i, e) in vetor.drain(0..).enumerate() 
      { array[i] = e; }
   return array;
}

/// recupera a string de bytes do disco.
pub fn carrega_do_bd() -> Bytes { 
   let mut arquivo: File = {
      OpenOptions::new()
      .read(true)
      .open(BD)
      .unwrap()
   };
   // lendo todos bytes do arquivo.
   let mut conteudo:Vec<u8> = Vec::new(); 
   // todas partidas filtradas.
   let mut dados: Vec<Dados> = Vec::new();

   // lendo todos dados do arquivo.
   match arquivo.read_to_end(&mut conteudo) {
      Ok(total) => 
         { println!("foram lidos {} bytes", total); }
      Err(_) =>
         { return Err("não foi possível lê os dados"); }
   };

   // compartimentarizando ...
   while !conteudo.is_empty() {
      // total de bytes a drenar:
      let bytes:Vec<_> = conteudo.drain(0..8).collect();
      let total:u64 = u64::from_be_bytes(vetor_para_array(bytes));
      // drenando ...
      let bytes:Vec<_> = conteudo.drain(0..total as usize).collect();
      dados.push(Dados::deserializa(bytes));
   }
   Ok(dados)
}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn primeiro_teste_carrega_do_bd() {
      let mut lista = carrega_do_bd().unwrap();
      assert!(!lista.is_empty());
      let dado = lista.remove(0);
      println!("{}", dado);
      // avaliação manual.
      assert!(true);
   }

   #[test]
   fn segundo_teste_carrega_do_bd() {
      let mut lista = carrega_do_bd().unwrap();
      assert!(lista.len() > 1);
      // todos dados.
      for dado in lista.drain(0..) 
         { println!("{}", dado); }
      // avaliação manual.
      assert!(true);
   }
}
