


/** Seleciona elementos do banco, em ordem,
 baseado num atributo mencionado.
*/

use crate::{carrega_do_bd, Dados};

// array do tipo 'Dados'.
type Partidas = Vec<Dados>;

/* Apenas as partidas 'vencidas' ou 'perdidas'. */
pub fn filtra_por_resultado(vitorioso: bool) -> Partidas {
   match carrega_do_bd() {
      Ok(mut dados) => {
         let mut selecao = Partidas::new();
         for p in dados.drain(0..) {
            /*
            if vitorioso {
               if p.vitoria
                  { selecao.push(p); }
            } else {
               if !p.vitoria
                  { selecao.push(p); }
            }*/
            if vitorioso {
               if p.vitoria
                  { selecao.push(p); }
            } else if !p.vitoria
               { selecao.push(p); }
         }
         selecao
      } Err(_) =>
         { panic!("não foi possível carrega-lôs"); }
   }
}

// baseado nos atributos do tipo 'Dados'.
pub enum Ordenacao {
   // baseado no comprimento final da 'Cobrinha'.
   Comprimento,
   // ordenação baseada na área.
   Dimensao,
   TotalDeBugs,
   // tempo de duração da partida.
   TempoDeDuracao,
}

impl Ordenacao {
   pub fn maior_comprimento(a: &Dados, b: &Dados) -> bool {
   /* O que é verificad é se o primeiro parâmetro é maior que o segundo, 
    * simples assim. */
      let ca = {
      // Comprimento do primeiro.
         let xi = a.comprimento;
         let xf = comprimento_final(a);
         if xf > xi
            { xf - xi }
         else
            { xi - xf }
      };
      let cb = {
      // Comprimento do segundo.
         let xi = b.comprimento;
         let xf = comprimento_final(b);
         if xf > xi
            { xf - xi }
         else
            { xi - xf }
      };
      // verifica proposição.
      ca > cb
   }

   pub fn maior_dimensao(a: &Dados, b: &Dados) -> bool {
   /* Ordena baseado na maior dimensão, para menor. */
      let area_a = a.dimensao.0 * a.dimensao.1;
      let area_b = b.dimensao.0 * b.dimensao.1;

      area_a > area_b
   }
}

#[allow(clippy::needless_borrow)]
pub fn filtra(ordem: Ordenacao) -> Partidas {
/* Ordenada as lista puxada pela memória baseada no tipo de 'Ordenação' 
 * encomendada. */
   match carrega_do_bd() {
      Ok(dados) => {
         // assert!(dados.len() > 0);
         assert!(!dados.is_empty());
         let mut selecao: Partidas = Vec::new();
         // insert sort(partidas mais longas à esquerda).
         for a in dados.iter() {
            let mut indice = 0;
            while indice < selecao.len() {
               let b = selecao[indice].clone();
               // ordenar no tipo de enum dado.
               match ordem {
                  Ordenacao::TempoDeDuracao => {
                     let ta = a.tempo_duracao.unwrap();
                     let tb = b.tempo_duracao.unwrap();
                     if ta > tb { break; }
                  } Ordenacao::Comprimento => {
                     if Ordenacao::maior_comprimento(&a, &b)
                        { break; }
                  } Ordenacao::Dimensao => {
                     if Ordenacao::maior_dimensao(&a, &b)
                        { break; }
                  } Ordenacao::TotalDeBugs => {
                     if a.total_de_bugs > b.total_de_bugs
                        { break; }
                  }
               }
               indice += 1;
            }
            selecao.insert(indice, a.clone());
         }
         assert_eq!(dados.len(), selecao.len());
         selecao
      } Err(_) =>
         { panic!("não foi possível carrega-lôs"); }
   }
}

fn comprimento_final(dados: &Dados) -> u16 {
   let ultimo = dados.fila_rastros.len() - 1;
   dados.fila_rastros[ultimo].2
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn filtro_por_resultadoFPR() {
      let lista_total = filtra(Ordenacao::TempoDeDuracao);
      let mut parte = filtra_por_resultado(true);
      let mut outra_parte = filtra_por_resultado(false);
      let t = lista_total.len();
      let p = parte.len();
      let o = outra_parte.len();
      println!(
         "\npercetual:\n\tDERROTAS({1:.1}%)\n\tVITÓRIAS:({0:.1}%)",
         ((p as f32) / (t as f32)) * 100.0,
         ((o as f32) / (t as f32)) * 100.0
      );
      assert_eq!(
         lista_total.len(), 
         parte.len() + outra_parte.len()
      );
      print!("\napena as \"partidas vitoriosas\":");
      for _ in 1..=5
         { print!("{}", parte.pop().unwrap()); }

      print!("\n\napenas as \"derrotas\":");
      for _ in 1..=5
         { print!("{}", outra_parte.pop().unwrap()); }
   }
   
   fn em_ordem_decrescente(l: Vec<Dados>) -> bool {
      let ultimo = l.len()-1;
      for i in 0..=ultimo-1 {
         let t = l[i].tempo_duracao.unwrap();
         let T = l[i+1].tempo_duracao.unwrap();
         if t < T
            { return false; }
      }
      return true;
   }

   #[test]
   fn FiltraPorTempo() {
      let lista = filtra(Ordenacao::TempoDeDuracao);
      let ultimo = lista.len()-1;
      for (p, a) in lista.iter().enumerate() { 
         // os dez primeiros ...
         if p < 10 
            { println!("{}", a); }
         // os dez últimos:
         if p >= ultimo - 10
            { println!("{}", a); }
      }
      /* verifica se estão ordenados em ordem
       * decrescente. */
      assert!(em_ordem_decrescente(lista));
   }

   #[test]
   fn FiltraPorComprimento() {
      let lista = filtra(Ordenacao::Comprimento);
      for (p, a) in lista.iter().enumerate() { 
         // os dez primeiros ...
         if p < 16 
            { println!("{}", a); }
      }
   }

   #[test]
   fn FiltraPorDimensao() {
      let lista = filtra(Ordenacao::Dimensao);
      let ultimo = lista.len()-1;
      for (p, a) in lista.iter().enumerate() { 
         // os dez primeiros ...
         if p < 16 
            { println!("{}", a); }
         if p >= ultimo - 6 
            { println!("{}", a); }
      }
   }

   use utilitarios::terminal_dimensao::{dimensao, Largura, Altura};
   use crate::estatisticas::Dimensao;

   fn cabe_dentro(j: Dimensao, t: Dimensao) -> bool 
      { t.0 <= j.0 && t.1 <= j.1 }

   #[test]
   fn CabeNaAtualTela() {
      let (H, L): Dimensao = {
         match dimensao() {
            Some((Largura(l), Altura(a))) => (a, l),
            None => { assert!(false); todo!() }
         }
      };
      let lista = filtra(Ordenacao::Dimensao);
      let total = lista.len() as f32;
      let mut cabivel = 0.0f32;
      let ultimo = lista.len()-1;
      println!("atual dimensão {}x{}", H, L);
      for dado in lista {
         let tela_dim = dado.dimensao;
         let janela_dim = (H, L);
         if !cabe_dentro(janela_dim, tela_dim)
            { continue; }
         println!("\t{}x{}", tela_dim.0, tela_dim.1);
         cabivel += 1.0;
      }
      println!(
         "cabém na tela do terminal:{:>5.1}%", 
         (cabivel/total) * 100.0
      );
   }
}
