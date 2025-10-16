/**
  Aqui ficarão as funções que recuperam/e gravam os dados gerados, estes 
  que são gerados durante a partida.
*/

// biblioteca do Rust:
use std::io::{Read, Write};
use std::fs::{OpenOptions, File};
use std::convert::{TryInto};
// Próprio caixote:
use crate::{computa_caminho, Dados, Serializacao};

// Deixando mais legível outputs das funções abaixo:
type Confirmacao = Result<(), &'static str>;
type Bytes = Result<Vec<Dados>, &'static str>;

// nome do arquivo.
const BANCO:&str = "data/partidas.dat";

#[allow(clippy::unused_io_amount)]
pub fn salva_no_bd(dados:Vec<u8>) -> Confirmacao {
/* Salva a string de bytes no disco. */
   let caminho = computa_caminho(BANCO);

	if cfg!(debug_assertions)
		{ println!("Caminho de 'partidas.dat': '{}'", caminho.display()); }
   
   let mut arquivo:File = {
      OpenOptions::new().append(true).create(true)
      .open(caminho).unwrap()
   };
   // Total de bytes vai primeiro.
   let total_de_bytes: u64 = dados.len() as u64;
   let bytes = total_de_bytes.to_be_bytes();
   arquivo.write(&bytes[..]).unwrap();

   // agora os dados do jogo em sí.
   match arquivo.write(&dados[..]) {
      Ok(qtd) => 
         { println!("Foram gravado {} bytes da partida.", qtd); Ok(()) }
      Err(_) => 
         { Err("erro ao gravar!!!") }
   }
}

pub fn carrega_do_bd() -> Bytes {
/* Recupera a string de bytes do disco. */
   let mut arquivo: File = {
      OpenOptions::new()
      .read(true)
      .open(computa_caminho(BANCO))
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
      let bytes:Vec<_> = conteudo.drain(0..).collect();
      // let total:u64 = u64::from_be_bytes(vetor_para_array(bytes));
      let array = bytes.try_into().unwrap();
      let total = u64::from_be_bytes(array);
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

   #[test]
   fn caminho_ao_bdd() {
      let caminho = computa_caminho(BANCO);
      println!("{}", caminho.display());
   }
}
