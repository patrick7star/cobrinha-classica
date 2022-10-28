


/** Seleciona elementos do banco, em ordem,
 baseado num atributo mencionado.
*/

use crate::{carrega_do_bd, Dados};

// array do tipo 'Dados'.
type Partidas = Vec<Dados>;

/* apenas as partidas 'vencidas' ou 
 * 'perdidas'. */
pub fn filtra_por_resultado(vitorioso: bool) -> Partidas {
   // carrega o banco de dados ...
   match carrega_do_bd() {
      Ok(mut dados) => {
         let mut selecao = Partidas::new();
         for p in dados.drain(0..) {
            if vitorioso {
               if p.vitoria
                  { selecao.push(p); }
            } else {
               if !p.vitoria
                  { selecao.push(p); }
            }
         }
         return selecao;
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
/* relegando aqui tal tarefa para não ter
 * que ficar aninhando o código, ainda mais
 * com pouco margem. */
impl Ordenacao {
   /* O que é verificad é 
    * se o primeiro parâmetro é maior que o
    * segundo, simples assim. */
   pub fn maior_comprimento(a: &Dados, b: &Dados) -> bool {
      // comprimento do primeiro.
      let ca = {
         let xi = a.comprimento;
         let xf = comprimento_final(a);
         if xf > xi
            { xf - xi }
         else
            { xi - xf }
      };
      // comprimento do segundo.
      let cb = {
         let xi = b.comprimento;
         let xf = comprimento_final(b);
         if xf > xi
            { xf - xi }
         else
            { xi - xf }
      };
      // verifica proposição.
      return ca > cb;
   }
   /* ordena baseado na maior dimensão, 
    * para menor. */
   pub fn maior_dimensao(a: &Dados, b: &Dados) -> bool {
      let area_a = a.dimensao.0 * a.dimensao.1;
      let area_b = b.dimensao.0 * b.dimensao.1;
      return area_a > area_b;
   }
}

/* ordenada as lista puxada pela memória baseada
 * no tipo de 'Ordenação' encomendada. */
pub fn filtra(ordem: Ordenacao) -> Partidas {
   // carrega o banco de dados ...
   match carrega_do_bd() {
      Ok(dados) => {
         assert!(dados.len() > 0);
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
         return selecao;
      } Err(_) =>
         { panic!("não foi possível carrega-lôs"); }
   }
}

fn comprimento_final(dados: &Dados) -> u16 {
   let ultimo = dados.fila_rastros.len() - 1;
   return dados.fila_rastros[ultimo].2;
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
}
