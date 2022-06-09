extern crate rand;
// caminho é basicamente(.. == crate).
use super::objetos_modelos::*;


pub fn pontos_aleatorios(max_linhas:u8, 
max_colunas:u8, qtd:u16) -> Vec<Ponto> {
   /* gera uma array com dez pontos aleatórios, dado
    * a delimitação da tela passada como argumentos. */
   let mut array:Vec<Ponto> = Vec::new();
   // criando dez pontos aleatórios e alterando o 
   // inicial da array.
   for _ in 1..=qtd {
      // dimensões 'y' e 'x' aleatórias.
      let mut linha = rand::random::<u8>();
      let mut coluna = rand::random::<u8>(); 

      // enquanto os número aleatórios não forem 
      // dentro da delimitação da tela, continuar
      // sorteando.
      if linha == 0
         { linha += 1; }
      else if linha >= max_linhas
         { linha %= max_linhas; }

      if coluna == 0 
         { coluna += 1; }
      else if coluna >= max_colunas
         { coluna %= max_colunas; }

      // forma ponto e indexa a array.
      array.push(Ponto{y:linha, x:coluna});
   }
   // retorno dos pontos aleatórios.
   return array;
}

pub fn pontos_aleatorios_consertado(max_linhas:u8, 
max_colunas:u8, qtd:u16) -> Vec<Ponto> {
   // gerando pontos aleatórios com função anterior.
   let mut pontos = pontos_aleatorios(max_linhas, max_colunas, qtd);
   // abreviação para variáveis longas.
   let (ml, mc) = (max_linhas, max_colunas);

   // iterando pontos da array à busca de pontos
   // que ficaram nas bordas.
   for p in pontos.iter_mut() {
      // barra superior e lateral esquerda.
      if p.y == 0 || p.x == 0 {
         if p.y == 0 
            { p.y = 1; }
         else if p.x == 0 
            { p.x = 1; }
         else { 
            p.x = 1; 
            p.y = 1; 
         }
      }

      // barra inferior e lateral direita.
      else if p.y == ml || p.x == mc {
         if p.y == ml 
            { p.y = ml-1; }
         else if p.x == mc 
            { p.x = mc-1; }
         else { 
            p.x = mc-1; 
            p.y = ml-1; 
         }
      }
   }
   // retorna pontos com erros consertados.
   return pontos;
}
