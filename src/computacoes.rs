/*!
 Computa tanto elementos para os 'objetos', como recebe tais para que se 
 compute a melhor posição, direção, tempo, tamanho e etc para este mesmo, 
 claro recebendo mais parâmetros além do próprio.
*/
extern crate rand;
// caminho é basicamente(.. == crate).
use crate::{Direcao, Cobrinha, Ponto};


pub fn pontos_aleatorios(max_linhas:u8, max_colunas:u8, qtd:u16) 
  -> Vec<Ponto> 
{
/* Gera uma array com dez pontos aleatórios, dado a delimitação da tela 
 * passada como argumentos. */
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
   array
}

pub fn pontos_aleatorios_consertado(max_linhas:u8, max_colunas:u8, 
  qtd:u16) -> Vec<Ponto> 
{
// Gerando pontos aleatórios com função anterior.
   let mut pontos = pontos_aleatorios(max_linhas, max_colunas, qtd);
   // abreviação para variáveis longas.
   let (ml, mc) = (max_linhas, max_colunas);

   // Iterando pontos da array à busca de pontos que ficaram nas bordas.
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
   pontos
}

pub fn cobrinha_proporcional(cobra:&mut Cobrinha, dimensao:(i32, i32))
{
// Aumenta a cobrinha dado a área de jogo.
   let lins = dimensao.0 - 2;
   let cols = dimensao.1 - 2;
   // complementando baseado na área.
   let area_janela = (lins * cols) as f32;
   let area_constante = 21_f32 * 15_f32;
   let qtd_membros_restantes = 3.0 * area_janela / area_constante;
   *cobra += qtd_membros_restantes as usize;
}

#[allow(clippy::nonminimal_bool)]
pub fn piloto_automatico(cobra:&Cobrinha, lin:i32, col:i32) -> Direcao {
/* Após o termino da partida, toma a cobrinha e a dirige até a borda, e faz 
 * dá várias voltas, na borda, no sentido-horário. */
   // encurtando variáveis com alias.
   let x = cobra.cabeca.posicao.x as i32;
   let y = cobra.cabeca.posicao.y as i32;
   let s = cobra.cabeca.sentido;

   let na_borda:bool = {
   // Verifica se está na borda da tela.
      (x == 1 && (y >= 1 && y <= lin-2)) ||
      (x == col-2 && (y >= 1 && y <= lin-2)) ||
      (y == 1 && (x >= 1 && x <= col-2)) ||
      (y == col-2 && (x >= 1 && x <= col-2))
   };
   
   let fora_da_borda = { 
   /* Closure que computa se tal ponto está fora da borda. Basicamente, a 
    * negação do anterior para um ponto dado. */
      |p:Ponto| 
         (p.x > 1 && (p.x as i32) < col-2) &&
         (p.y > 1 && (p.y as i32) < lin-2)
   };
   
   if na_borda {
   /* Neste caso, na borda; supondo que já começou no sentido-horário, que 
    * é o planejado. */
      // canto-superior-esquerdo.
      let cse = { x == 1 && y == 1 };
      // canto-superior-direito.
      let csd = { x == col-2 && y == 1 };
      // canto-inferior-direito.
      let cid = {x == col-2 && y == lin-2 };
      // canto-inferior-esquerdo.
      let cie = { x == 1 && y == lin-2 };
      // tomando decisões baseado nisso.
      if cse { Direcao::Leste }
      else if csd { Direcao::Sul }
      else if cid { Direcao::Oeste }
      else if cie { Direcao::Norte }
      // se nenhum caso, apenos o sentido anterior.
      else { 
         /* verifica se a posição anterior estava fora da borda. */
         if fora_da_borda(cobra.cabeca.antiga_posicao) {
            match s {
               Direcao::Oeste => 
               // caso esteja, se vinher da direita, então vai para cima.
                  { Direcao::Norte }
               Direcao::Norte =>
               // se está vindo de baixo, vai para direita.
                  { Direcao::Leste }
               Direcao::Sul =>
               // se está vindo de cima, vai para esquerda.
                  { Direcao::Oeste }
               Direcao::Leste =>
               // o último caso é  vindo da esquerda, então vai para baixo.
                  { Direcao::Sul }
            }
            /*
            if s == Direcao::Oeste
            // caso esteja, se vinher da direita, então vai para cima.
               { Direcao::Norte }
            else if s == Direcao::Norte
            // se está vindo de baixo, vai para direita.
               { Direcao::Leste }
            else if s == Direcao::Sul
            // se está vindo de cima, vai para esquerda.
               { Direcao::Oeste }
            else 
            // o último caso é  vindo da esquerda, então vai para baixo.
               { Direcao::Sul }*/
         }
         /* nenhum acima, possívelmente um bug em execução,
          * não se pode fazer nada ainda no momento. Retorna
          * rota original.  */
         else { s }
      }
   }
   else { 
   // Continue indo no mesmo sentido.
      if (s == Direcao::Sul) && (y < lin-2) && (x > 1 && x < col-2)
      /* Como não cai em nenhum caso específico anterior, então vamos 
       * detalhar ainda mais, e aplicar uma solução simples. Como não 
       * funciona apenas para dobrar a parede vindo do Norte, vamos dá uma 
       * virada para este caso, para qualquer lado mais próximo, assim cai 
       * nos casos que funcionam. */
         { Direcao::Leste }
      else { s }
   }
}

pub fn colidiu(cobra:&Cobrinha, lin:i32, col:i32) -> bool {
/* Retorna se a cobrinha ultrapassou as paredes da tela gerada pelo 
 * ncurses. */
   // alias para posição da cobrinha.
   let y = cobra.cabeca.posicao.y as i32;
   let x = cobra.cabeca.posicao.x as i32;

   // verifica uma colisão com o próprio corpo.
   for membro in cobra.membros.iter() {
      let pc = cobra.posicao();
      let pm = membro.posicao;
      if pc == pm
         { return true; }
   }

   // colidiu na "parede esquerda" ou no "teto".
   if x == 0 || y == 0 { true }
   // intervalo permitido nas duas dimensões.
   else { !((x >= 1 && x <= col-2) && ( y >= 1 && y <= lin-2)) }
}

pub fn colidiu_com_ela_mesma(cobra: &Cobrinha) -> bool {
/* Verifica se ela colidiu com seu próprio corpo. O algoritmo para 
 * detectar tal, consiste em apenas verifica se a posição da cabeça(que
 * é controlada) ocupa a mesma posição de outro membro do corpo. */
   for membro in cobra.membros.iter() {
      let posicao_cabeca = cobra.posicao();
      let posicao_membro = membro.posicao;

      if posicao_cabeca == posicao_membro
         { return true; }
   }
   // Se não correspondeu com nenhuma parte, então tal colisão não ocorreu.
   false 
}
