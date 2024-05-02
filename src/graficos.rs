
mod janelas;
pub use janelas::{remenda_borda_da_tela, Tela};
// biblioteca externa:
extern crate pancurses;
use pancurses::{
   Input::{Character, KeyDown, KeyUp, KeyRight, KeyLeft}, 
   Window, napms, COLOR_PAIR, beep, A_BLINK, A_BOLD
};
// biblioteca do Rust:
use std::time::Instant;
// importando da minha biblioteca:
use crate::{
   Alvos, Cobrinha, Ponto, Direcao,
   VELOCIDADE, colidiu, Dados, Dilutor,
   introducao, piloto_automatico, 
   string_extensao::StringExtensao
};

// desenha na tela os bichinhos a serem devorados.
fn plota_metas_melhorado(janela: &Window, meta:&Alvos) {
   match meta.a_mostrar() {
      Some(ponto) => { 
         janela.mv(ponto.y as i32, ponto.x as i32); 
         janela.attrset(A_BOLD);
         janela.attrset(A_BLINK);
         janela.addch(meta.forma as u32 | COLOR_PAIR(12));
         janela.attroff(A_BLINK);
         janela.attrset(A_BOLD);
      } None => ()
   };
}

// desenha na tela a cobrinha.
pub fn plota_cobrinha(janela: &Window, obj:&Cobrinha) {
   // pinta a cabeça da cobrinha.
   janela.mv( 
      obj.cabeca.posicao.y as i32,
      obj.cabeca.posicao.x as i32
   );
   janela.addch(obj.cabeca.forma as u32 | COLOR_PAIR(11));
   // limpa comida, deixa apenas farelo.
   janela.mv(
      obj.cabeca.antiga_posicao.y as i32,
      obj.cabeca.antiga_posicao.x as i32
   );
   janela.addch(' ' as u32);
   // agora com os membros.
   let mut n = 0;
   while n < obj.membros.len() {
      janela.mv( 
         obj.membros[n].posicao.y as i32,
         obj.membros[n].posicao.x as i32
      );
      janela.addch(
         (obj.membros[n].forma as u32) | 
         COLOR_PAIR(11)
      );
      // limpa comida, deixa apenas farelo.
      janela.mv(
         obj.membros[n].antiga_posicao.y as i32,
         obj.membros[n].antiga_posicao.x as i32
      );
      janela.addch(' ' as u32);
      n += 1;
   }

   // verifia se a cabeça está acima de algum membro.
   let esta_sobre_corpo:bool = {
      // parte do presuposto que não está.
      let mut confirma:bool = false;
      for membro in &obj.membros {
         // se estiver, registra isso.
         if obj.cabeca.posicao == membro.posicao
            { confirma = true; break; }
      }
      // retorna o resultado.
      confirma
   };
   /* pinta a cabeça da cobrinha com uma
    * cor diferente, pois está acima 
    * do corpo. */
   if esta_sobre_corpo {
      janela.mv( 
         obj.cabeca.posicao.y as i32,
         obj.cabeca.posicao.x as i32
      );
      janela.addch(obj.cabeca.forma as u32 | COLOR_PAIR(3));
   }
}

/* desenha a cobrinha onde quer que ela vá. Com a array de direções que 
 * são dado para ela "virar" a cada novo passo. 
 */
pub fn roda_jogo<J>(janela: &mut J, obj:&mut Cobrinha, 
obj_metas:&mut Alvos) -> Dados 
  where J: AsMut<Window> 
{
   let janela = janela.as_mut();
   // dimensão da tela.
   let (linhas, colunas) = janela.get_max_yx();
   let mut dir: Direcao;
   let mut visualiza_status = true;
   let mut ja_apagada = false;
   let mut abandonar = false;
   let mut metadados = Dados::gera(
      obj, obj_metas,
      (linhas as u16, colunas as u16)
   );
   let limite = (
      Ponto { y:1, x:1 },
      Ponto { 
         y:(linhas-1) as u8, 
         x:(colunas-1) as u8
      }
   );
   let mut controlador = Dilutor::instancia(limite);

   // apresentação ao iniciar o jogo.
   plota_cobrinha(janela, obj);
   introducao(janela);
   
   // enquanto todos alvos/bichos não se forem...
   while !obj_metas.sem_alvos() && !colidiu(obj, linhas, colunas) {
      // colhendo dados antes do próximo movimento.
      metadados.atualiza(obj, obj_metas);

      // sentido não oposto ao do atual.
      /*
      match janela.getch() {
         Some(KeyLeft) => {
            if obj.sentido() != Direcao::Oeste.oposto()
               { dir = Direcao::Oeste; }
            else
               { dir = obj.cabeca.sentido; }
         } Some(KeyRight) => {
            if obj.sentido() != Direcao::Leste.oposto()
               { dir = Direcao::Leste; }
            else
               { dir = obj.sentido(); }
         } Some(KeyUp) => {
            if obj.sentido() != Direcao::Norte.oposto()
               { dir = Direcao::Norte; }
            else
               { dir = obj.sentido(); }
         } Some(KeyDown) => {
            if obj.sentido() != Direcao::Sul.oposto()
               { dir = Direcao::Sul; }
            else
               { dir = obj.sentido(); }
         } Some(Character(tecla)) => {
            // sai do programa quebrando o loop.
            if tecla == 's' { break; }
            // ativa/desativa barra de status.
            else if tecla == 'b' {
               visualiza_status = !visualiza_status; 
               // apaga uma última vez.
               janela.mv(linhas - 2, 1);
               janela.clrtoeol();
               janela.refresh();
            }
            dir = obj.sentido();
         } _ => { dir = obj.sentido(); }
      };
      */
      dir = controladores_da_cobrinha(
         janela, obj,
         &mut visualiza_status, 
         &mut abandonar
      );

      // sai do 'loop'.
      if abandonar { break;}
      
      // move a cobrinha.
      obj.mover(dir);
      plota_cobrinha(janela, &obj);
      
      // se estiver no local de um "bicho", captura-lô.
      let devorou = obj_metas.captura_valido(obj.posicao());
      // se capturou algo, crescer a cobrinha em dois membros.
      if devorou { 
         // coloca na "fila de incremento".
         controlador += 5;
         // sinal de captura.
         beep();
      }
      // verifica se pode incrementar de um-em-um.
      if controlador.pode_aumentar(obj) 
         { *obj += 1 }; 
      // mostra bichos/locais restantes.
      plota_metas_melhorado(janela, obj_metas);
      // barra de status com informações importantes.
      if visualiza_status 
         { barra_status_flutuante(janela, obj, obj_metas, &mut ja_apagada); }

      janela.refresh();
      napms(VELOCIDADE); // um décimo de segundo.
   }
   // útlimo registro.
   metadados.atualiza(obj, obj_metas);

   /* colisão, apenas abandona o jogo imediamente,
    * sem rodar a animação. */
   if colidiu(obj, linhas, colunas) 
      { return metadados; }

   // animação de termino.
   animacao_final(janela, obj, obj_metas, 60, &metadados);

   return metadados;
}

/* barra de status flutuante: o mesmo que 
 * a outra, porém está interage com a cobrinha
 * e os alvos(bichinhos) quando têm espaço
 * o espaços conflitados entre sí.
 */
fn barra_status_flutuante(janela: &Window, cobra:&Cobrinha, 
bichos:&Alvos, ja_apagada: &mut bool) {
   // dimensão da tela.
   let tela = janela;
   let linhas = tela.get_max_y();

   let barra = { 
      format!("\trestantes:{}\tcomprimento:{}",
         bichos.qtd_alvos_restantes(),
         cobra.tamanho()
      )
   };
   
   // cálculo de centralização.
   let posicao_coluna:i32;
   posicao_coluna = 5;

   // cobrinha acima da barra de status.
   let cobrinha_em_cima = {
      // atalhos com renomeação.
      let p1:bool = {
         cobra.posicao().y as i32 == linhas-2 
         && cobra.posicao().x as i32 >= posicao_coluna
      };
      /* verificando se algum membro 
       * também está acima... */
      let p2:bool = {
         /* array com valores lógicos dizendo se cada
          * membro está ou não, acima da barrastatus.*/
         let mut respota_cada:Vec<bool>;
         respota_cada = vec![ false; cobra.membros.len() ];
         /* índice da array acima para acessar/modificar 
          * valor lógico. */
         let mut indice = 0;
         // verificando cada membro.
         for parte in &cobra.membros {
            // se estiver acima, "afirmar" isso na array.
            if parte.posicao.y as i32 == linhas-2 &&
               parte.posicao.x as i32 >= posicao_coluna
                  { respota_cada[indice] = true; }
            indice += 1;
         }
         /* se qualquer membro estiver acima, então a
          * cobrinha também está. */
         respota_cada.into_iter().any(|vl| vl)
      };
      /* se a cabeça e alguma parte do corpo estiver 
       * acima, marcar como a cobra também está. */
      p1 || p2 
   };

   // cobrinha sem sobrer a barra, ela é atualizada..
   if !cobrinha_em_cima {
      tela.mv(linhas-2, posicao_coluna);
      tela.color_set(13);
      tela.addstr(barra.as_str());
      *ja_apagada = false;
   }
   else {
      if !(*ja_apagada) {
         // move para a linha e apaga-a.
         tela.mv(linhas-2, 1);
         tela.clrtoeol();
         *ja_apagada = true;
      }
   }
}

/* Animação de uma quantia computada de segundos
 * em que haverá animação da cobrinha rodeando 
 * a tela. */
fn animacao_final(janela: &Window, cobra:&mut Cobrinha, 
  alvos: &mut Alvos, duracao: u64, dados: &Dados) 
{
   /* animando por um tempo após ter ganho o jogo.
    * continuar com a cobrinha por 1 seg e meio. */
   let mut ja_apagada = false;
   let tempo = Instant::now();
   let (linhas, colunas) = janela.get_max_yx();
   /* última atualizada, pois o último bichinho 
    * não será pintado corretamente. */
   while tempo.elapsed().as_secs() <= duracao { 
      // saída de emergência.
      match janela.getch() {
         Some(Character(tecla)) => {
            if tecla == 's'
               { break; }
            else if tecla == 'S'
               { break; }
         } None | _ => ()
      };

      // obtem sentido do próximo passo.
      let nova_dir = piloto_automatico(cobra, linhas, colunas);

      /* visualizando e movimento cobrinha. O
       * básico: apenas ela e a barra de status. */
      cobra.mover(nova_dir);
      plota_cobrinha(janela, cobra);
      plota_metas_melhorado(janela, alvos);
      barra_status_flutuante(janela, cobra, alvos, &mut ja_apagada);
      plota_caixa_flutuante(janela, dados);

      janela.refresh();
      napms(VELOCIDADE); 
   }
}

/* Caixa fluatuante com o resultado do jogo
 * no caso de vitória.  */
fn plota_caixa_flutuante(janela: &Window, dados: &Dados) {
   // dimensão da tela.
   let (linhas, colunas) = janela.get_max_yx();

   let dados_str = dados.to_string();
   // "qtd. de linhas" e "largura maior".
   let altura = (dados_str.lines().count() - 2) as i32;
   let largura = {
      dados_str.lines()
      .map(|s| s.len())
      .max().unwrap() as i32
   };
   // posição centralizada.
   let (y, x) = (
      (linhas - altura) / 2, 
      (colunas - largura) / 2
   );
   // pulando linha-em-branco e cabeçalho ...
   let iterador = dados_str.lines().map(|s| s.desacentua()).skip(2);

   for (k, linha) in iterador.enumerate() {
      janela.mv(y+(k as i32), x);
      let linha = linha.trim_start().trim_start_matches('\r');
      janela.addstr(linha);
   }

   janela.refresh();
}

/* Dá inicio ao jogo, só que esta aqui tem uma configuração especial, pois
 * não restringe a 'cobrinha' ao tamanho da tela, assim quando há colisão
 * transporta ela para outro lado. */
pub fn roda_jogo_sem_barreiras<J>(janela: &mut J, obj:&mut Cobrinha, 
obj_metas:&mut Alvos) -> Dados 
  where J: AsMut<Window> 
{
   let janela = janela.as_mut();
   // dimensão da tela.
   let (linhas, colunas) = janela.get_max_yx();
   let mut dir: Direcao;
   let mut visualiza_status = true;
   let mut ja_apagada = false;
   let mut metadados = Dados::gera(
      obj, obj_metas,
      (linhas as u16, colunas as u16)
   );
   let limite = (
      Ponto { y:1, x:1 },
      Ponto { 
         y:(linhas-1) as u8, 
         x:(colunas-1) as u8
      }
   );
   let mut controlador = Dilutor::instancia(limite);
   let __nao_debug__ = !cfg!(debug_assertions); 
   let mut exit_pressionado: bool = false;

   // apresentação ao iniciar o jogo.
   plota_cobrinha(janela, obj);
   introducao(janela);

   // enquanto todos alvos/bichos não se forem...
   while !obj_metas.sem_alvos() {
      // colhendo dados antes do próximo movimento.
      metadados.atualiza(obj, obj_metas);

      dir = controladores_da_cobrinha(
         janela, obj, &mut visualiza_status,
        &mut exit_pressionado
      ); 
      // move a cobrinha.
      obj.mover(dir);
      plota_cobrinha(janela, &obj);
      
      // se estiver no local de um "bicho", captura-lô.
      let devorou = obj_metas.captura_valido(obj.posicao());
      // se capturou algo, crescer a cobrinha em dois membros.
      if devorou { 
         // coloca na "fila de incremento".
         controlador += 5;
         // sinal de captura.
         beep();
      }
      /* Só aumenta o corpo da cobrinha após devoramento do 'bug' no
       * compilado final do programa. */
      if __nao_debug__ {
         // verifica se pode incrementar de um-em-um.
         if controlador.pode_aumentar(obj) 
            { *obj += 1 }; 
      }
      // mostra bichos/locais restantes.
      plota_metas_melhorado(janela, obj_metas);
      // barra de status com informações importantes.
      if visualiza_status 
         { barra_status_flutuante(janela, obj, obj_metas, &mut ja_apagada); }

      // atualização de frame do jogo.
      janela.refresh();
      napms(VELOCIDADE); // um décimo de segundo.
   }
   // útlimo registro.
   metadados.atualiza(obj, obj_metas);

   /* colisão, apenas abandona o jogo imediamente,
    * sem rodar a animação. */
   if colidiu(obj, linhas, colunas) 
      { return metadados; }

   // animação de termino.
   animacao_final(janela, obj, obj_metas, 60, &metadados);

   return metadados;
}

fn controladores_da_cobrinha(janela: &mut Window, obj: &mut Cobrinha,
  status: &mut bool, saida: &mut bool) -> Direcao 
{
   /* Baseado em pressões em determinada teclas, pode mover a 'cobrinha',
    * acionar ou não a 'barra de status' e sair do programa. No fim, 
    * retorna a 'direção' dada ao objeto. */
   let atual_direcao: Direcao;
   let (linhas, _colunas) = janela.get_max_yx();

   // sentido não oposto ao do atual.
   match janela.getch() {
      Some(KeyLeft) => {
         if obj.sentido() != Direcao::Oeste.oposto()
            { atual_direcao = Direcao::Oeste; }
         else
            { atual_direcao = obj.cabeca.sentido; }
      } Some(KeyRight) => {
         if obj.sentido() != Direcao::Leste.oposto()
            { atual_direcao = Direcao::Leste; }
         else
            { atual_direcao = obj.sentido(); }
      } Some(KeyUp) => {
         if obj.sentido() != Direcao::Norte.oposto()
            { atual_direcao = Direcao::Norte; }
         else
            { atual_direcao = obj.sentido(); }
      } Some(KeyDown) => {
         if obj.sentido() != Direcao::Sul.oposto()
            { atual_direcao = Direcao::Sul; }
         else
            { atual_direcao = obj.sentido(); }
      } Some(Character(tecla)) => {
         if tecla == 's' 
            // sai do programa quebrando o loop.
            { *saida = true; }
         else if tecla == 'b' {
            // ativa/desativa barra de status.
            *status = !*status; 
            // apaga uma última vez.
            janela.mv(linhas - 2, 1);
            janela.clrtoeol();
            janela.refresh();
         }
         atual_direcao = obj.sentido();
      } _ => 
         { atual_direcao = obj.sentido(); }
   };
   // retorna a direção selecionada.
   return atual_direcao;
}

use crate::teletransporta_cobrinha;
use std::time::{Duration};

static mut VOLTAS: u16 = 0;
static mut CURSOR: usize = 0;
static mut KILOMETRAGEM: u16 = 0;

fn alterna_direcao() -> Direcao {
   const ROTAS: [Direcao; 4] = [
      Direcao::Leste, Direcao::Norte,
      Direcao::Oeste, Direcao::Sul
   ];

   if unsafe { VOLTAS } == 3 
      // aumenta o cursor para próxima direção e reseta voltas contadas.
      { unsafe { CURSOR += 1; VOLTAS = 0; KILOMETRAGEM = 0; } }
   unsafe { ROTAS[CURSOR % 4] }
}

fn conta_voltas_da_cobrinha(limite_h: u16)
{
   /* Adicionando de vez em quando isso desviará a cobrinhar de sempre 
    * virar no mesmo ponto 'x' ou 'y'. */
   const DESALINHAMENTO: u16 = 13;

   unsafe {
      if KILOMETRAGEM % DESALINHAMENTO == 0
         { KILOMETRAGEM += DESALINHAMENTO; }

      // conta cada "tic" da cobrinha no jogo...
      KILOMETRAGEM += 1;
      VOLTAS = KILOMETRAGEM / limite_h as u16;
   }
}

pub fn roda_animacao_movimento_continuo<J>(board: &mut J, 
  snake:&mut Cobrinha) where J: AsMut<Window>
{
   let janela = board.as_mut();
   #[allow(non_snake_case)]
   let (Y, X) = (
      janela.get_max_y() as u8,
      janela.get_max_x() as u8
   );
   let dimensao = (Y, X);
   let timer = Instant::now();
   const LIMITE: u64 = 49;

   // apresentação ao iniciar o jogo.
   plota_cobrinha(janela, snake);
   introducao(janela);

   while timer.elapsed() < Duration::from_secs(LIMITE) {
      // move a cobrinha.
      conta_voltas_da_cobrinha(janela.get_max_x() as u16);
      snake.mover(alterna_direcao());
      teletransporta_cobrinha(dimensao, snake);
      plota_cobrinha(janela, &snake);
      
      // atualização de frame do jogo.
      remenda_borda_da_tela(janela);
      janela.refresh();
      napms(VELOCIDADE / 3); // um décimo de segundo.
   }
}

