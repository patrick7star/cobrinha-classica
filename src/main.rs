// biblioteca externa:
extern crate ncurses;
use ncurses::*;
extern crate rand;
// biblioteca do Rust:
use std::time::Instant;
// importando da minha biblioteca:
use cobrinha_classica::*;


// desenha na tela os bichinhos a serem devorados.
fn plota_metas_melhorado(meta:&Alvos) {
   match meta.a_mostrar() {
      Some(ponto) => { 
         wmove(stdscr(), ponto.y as i32, ponto.x as i32); 
         attrset(A_BOLD());
         attrset(A_BLINK());
         addch(meta.forma as u32 | COLOR_PAIR(12));
         attroff(A_BLINK());
         attrset(A_BOLD());
      } None => ()
   };
}

// desenha na tela a cobrinha.
fn plota_cobrinha(obj:&Cobrinha) {
   // pinta a cabeça da cobrinha.
   wmove(stdscr(), 
      obj.cabeca.posicao.y as i32,
      obj.cabeca.posicao.x as i32
   );
   addch(obj.cabeca.forma as u32 | COLOR_PAIR(11));
   // limpa comida, deixa apenas farelo.
   wmove(stdscr(),
      obj.cabeca.antiga_posicao.y as i32,
      obj.cabeca.antiga_posicao.x as i32
   );
   addch(' ' as u32);
   // agora com os membros.
   let mut n = 0;
   while n < obj.membros.len() {
      wmove(stdscr(), 
         obj.membros[n].posicao.y as i32,
         obj.membros[n].posicao.x as i32
      );
      addch(
         (obj.membros[n].forma as u32) | 
         COLOR_PAIR(11)
      );
      // limpa comida, deixa apenas farelo.
      wmove(
         stdscr(),
         obj.membros[n].antiga_posicao.y as i32,
         obj.membros[n].antiga_posicao.x as i32
      );
      addch(' ' as u32);
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
      wmove(stdscr(), 
         obj.cabeca.posicao.y as i32,
         obj.cabeca.posicao.x as i32
         );
      addch(obj.cabeca.forma as u32 | COLOR_PAIR(3));
   }
}

/* desenha a cobrinha onde quer que ela vá. Com
 * a array de direções que são dado para ela "virar"
 * a cada novo passo. 
 */
fn roda_jogo(obj:&mut Cobrinha, obj_metas:&mut Alvos) -> Dados {
   // dimensão da tela.
   let (linhas, colunas) = (
      getmaxy(stdscr()), 
      getmaxx(stdscr())
   );
   let mut dir;
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

   // apresentação ao iniciar o jogo.
   plota_cobrinha(obj);
   introducao();
   
   // enquanto todos alvos/bichos não se forem...
   while !obj_metas.sem_alvos() && !colidiu(obj) {
      // colhendo dados antes do próximo movimento.
      metadados.atualiza(obj, obj_metas);

      // tecla preesionada:
      let pressionado = getch();
      
      // sentido não oposto ao do atual.
      if pressionado == KEY_LEFT {  
         if obj.sentido() != Direcao::Oeste.oposto()
            { dir = Direcao::Oeste; }
         else
            { dir = obj.cabeca.sentido; }
      } else if pressionado == KEY_RIGHT {
         if obj.sentido() != Direcao::Leste.oposto()
            { dir = Direcao::Leste; }
         else
            { dir = obj.sentido(); }
      } else if pressionado == KEY_UP {
         if obj.sentido() != Direcao::Norte.oposto()
            { dir = Direcao::Norte; }
         else
            { dir = obj.sentido(); }
      } else if pressionado == KEY_DOWN {
         if obj.sentido() != Direcao::Sul.oposto()
            { dir = Direcao::Sul; }
         else
            { dir = obj.sentido(); }
      } else {
         // sai do programa quebrando o loop.
         if pressionado == 's' as i32
            { break; }
         // ativa/desativa barra de status.
         else if pressionado == 'b' as i32 { 
            visualiza_status = !visualiza_status; 
            // apaga uma última vez.
            mv(linhas - 2, 1);
            clrtoeol();
            refresh();
         }
         // direção atual.
         dir = obj.sentido();
      }
      
      // move a cobrinha.
      obj.mover(dir);
      plota_cobrinha(&obj);
      
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
      plota_metas_melhorado(obj_metas);
      // barra de status com informações importantes.
      if visualiza_status 
         { barra_status_flutuante(obj, obj_metas, &mut ja_apagada); }

      refresh();
      napms(VELOCIDADE); // um décimo de segundo.
   }
   // útlimo registro.
   metadados.atualiza(obj, obj_metas);

   /* colisão, apenas abandona o jogo imediamente,
    * sem rodar a animação. */
   if colidiu(obj) { return metadados; }

   // animando por um tempo após ter ganho o jogo.
   // continuar com a cobrinha por 1 seg e meio.
   let tempo = Instant::now();
   /* última atualizada, pois o último bichinho 
    * não será pintado corretamente. */
   //atualiza(obj, &molde);
   while tempo.elapsed().as_secs() <= 60 { 
      // saída de emergência.
      if getch() as u32 == 's' as u32
         { break; }

      // obtem sentido do próximo passo.
      let nova_dir = cobrinha_piloto_automatico(obj, linhas, colunas);

      /* visualizando e movimento cobrinha. O
       * básico: apenas ela e a barra de status. */
      obj.mover(nova_dir);
      plota_cobrinha(&obj);
      plota_metas_melhorado(obj_metas);
      if visualiza_status 
         { barra_status_flutuante(obj, obj_metas, &mut ja_apagada); }

      refresh();
      napms(VELOCIDADE); 
   }

   return metadados;
}

// aumenta a cobrinha dado a área de jogo.
fn cobrinha_proporcional( cobra:&mut Cobrinha, dimensao:(i32, i32))
{
   let lins = dimensao.0 - 2;
   let cols = dimensao.1 - 2;
   // complementando baseado na área.
   let area_janela = (lins * cols) as f32;
   let area_constante = 21_f32 * 15_f32;
   let qtd_membros_restantes = 3.0 * area_janela / area_constante;
   *cobra += qtd_membros_restantes as usize;
}

// encaracola cobrinha.
fn encaracola(cobra:&mut Cobrinha) {
   let t: f32 = cobra.tamanho() as f32;
   let dirs = [
      Direcao::Norte,
      Direcao::Leste,
      Direcao::Sul,
      Direcao::Oeste
   ];
   /* de onde vêm tal fórmula:
    * a cobrinha dá 1 passo ao 
    * norte; depois 2 passo ao Oeste;
    * então 3 passos ao Sul; 4 passos
    * ao Leste; 5 passos ao... Norte
    * outra vez; e etc. Isso até enrolá
    * todo seu corpo de comprimento 't'.
    * Portanto, os 'n' passos crescentes 
    * dados de modo circular têm que somar 
    * (1+2+3+...+n) menor que o comprimento
    * 't' dela. Assim, o melhor 'n' é o
    * valor da soma aritmética que  bate 
    * ou fica no limite de 't'. A  seguinte 
    * inequação: 1+2+3+...+n < t. Reduzindo
    * ela a um equivalente mais fácil de
    * resolução chegamos no seguinte:
    *       n^2 + n -2t < 0
    */
   let delta = 1.0 + 9.0 * t;
   let n = (-1.0 + delta.sqrt()) / 2.0;

   for k in 1..=(n as usize) {
      for _ in 1..=k 
         { cobra.mover(dirs[k % 4]); }
   }
}

// execução de testes...
fn main() {
   // ativando unicode characteres...
   let local = LcCategory::all;
   setlocale(local, "pt.UTF-8");

   // iniciando terminal...
   let _tabuleiro = initscr();
   start_color();
   nodelay(stdscr(),true);
   keypad(stdscr(), true);
   curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

   /* paleta de cores(
    * background: #9BBA5A
    * borda: #272F17
    * cobrinha: #2B331A
    */
   // criando novas cores.
   init_color(99, 204, 255, 158);
   init_color(98, 0, 0, 0);
   init_color(97, 0, 0, 0); 
   const FUNDO:i16 = 99;
   const BORDA:i16 = 98;
   const CORPO:i16 = 97;
   init_pair(3, COLOR_GREEN, FUNDO);
   init_pair(11, CORPO, FUNDO);
   init_pair(12, COLOR_YELLOW, FUNDO);
   init_pair(13, COLOR_WHITE, FUNDO);
   init_pair(14, BORDA, FUNDO);
   bkgd(' ' as u32 | COLOR_PAIR(11));
   border(0, 0, 0, 0, 0, 0, 0, 0 | COLOR_PAIR(14));

   // obtendo dimensão do terminal...
   let mut linhas = 0;
   let mut colunas = 0;
   getmaxyx(
      stdscr(), 
      &mut linhas, 
      &mut colunas
   );

   // obtendo a posição do meio da tela.
   let meio = Ponto{ 
      y:(linhas / 2) as u8, 
      x:(colunas / 2) as u8 
   };

   // instânciando cobrinha e os bichinhos/alvos.
   let mut cobra = Cobrinha::criar(meio);
   cobrinha_proporcional(&mut cobra, (linhas, colunas));
   // enrrola ela antes de aparecer em tela.
   encaracola(&mut cobra);
   let mut metas = Alvos::cria(
      (linhas-1) as u8, 
      (colunas-1) as u8, 
      rand::random::<u16>() % 100 
   );
   // rodando o jogo, e colhendo dados.
   let dados_do_jogo = roda_jogo(&mut cobra, &mut metas); 

   // finalizando terminal...
   napms(700);
   endwin();

   // visualizando informação...
   println!("{}", dados_do_jogo);

   // salvando o resultado ...
   match salva_no_bd(dados_do_jogo.serializa()) {
      Ok(_) => { println!("partida registrada com sucesso."); }
      Err(erro) => { println!("ERRO:[{}]", erro); }
   };
}

/* barra de status flutuante: o mesmo que 
 * a outra, porém está interage com a cobrinha
 * e os alvos(bichinhos) quando têm espaço
 * o espaços conflitados entre sí.
 */
fn barra_status_flutuante(cobra:&Cobrinha, 
bichos:&Alvos, ja_apagada:&mut bool) {
   // dimensão da tela.
   let mut linhas:i32 = -1;
   let mut colunas:i32 = -1;
   let tela = stdscr();
   getmaxyx(tela, &mut linhas, &mut colunas);

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
      wmove(tela, linhas-2, posicao_coluna);
      color_set(13);
      addstr(barra.as_str());
      *ja_apagada = false;
   }
   else {
      if !(*ja_apagada) {
         // move para a linha e apaga-a.
         mv(linhas-2, 1);
         clrtoeol();
         *ja_apagada = true;
      }
   }
}

/* após o termino da partida, toma a cobrinha e 
 * a dirige até a borda, e faz dá várias voltas,
 * na borda, no sentido-horário.
 */
fn cobrinha_piloto_automatico<'b>(cobra:&'b Cobrinha, 
lin:i32, col:i32) -> Direcao {
   // encurtando variáveis com alias.
   let x = cobra.cabeca.posicao.x as i32;
   let y = cobra.cabeca.posicao.y as i32;
   let s = cobra.cabeca.sentido;

   // verifica se está na borda da tela.
   let na_borda:bool = {
      (x == 1 && (y >= 1 && y <= lin-2)) ||
      (x == col-2 && (y >= 1 && y <= lin-2)) ||
      (y == 1 && (x >= 1 && x <= col-2)) ||
      (y == col-2 && (x >= 1 && x <= col-2))
   };
   
   /* closure que computa se tal ponto está fora 
    * da borda. Basicamente, a negação do anterior
    * para um ponto dado. */
   let fora_da_borda = { 
      |p:Ponto| 
         (p.x > 1 && (p.x as i32) < col-2) &&
         (p.y > 1 && (p.y as i32) < lin-2)
   };
   
   /* neste caso, na borda; supondo que já
    * começou no sentido-horário, que é o 
    * planejado. */
   if na_borda {
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
         /* verifica se a posição anterior estava
          * fora da borda. */
         if fora_da_borda(cobra.cabeca.antiga_posicao) {
            // caso esteja, se vinher da direita, então vai para cima.
            if s == Direcao::Oeste
               { Direcao::Norte }
            // se está vindo de baixo, vai para direita.
            else if s == Direcao::Norte
               { Direcao::Leste }
            // se está vindo de cima, vai para esquerda.
            else if s == Direcao::Sul
               { Direcao::Oeste }
            // o último caso é  vindo da esquerda, então vai para baixo.
            else 
               { Direcao::Sul }
         }
         /* nenhum acima, possívelmente um bug em execução,
          * não se pode fazer nada ainda no momento. Retorna
          * rota original.  */
         else { s }
      }
   }
   // continue indo no mesmo sentido.
   else { 
      /* como não cai em nenhum caso específico anterior, então
       * vamos detalhar ainda mais, e aplicar uma 
       * solução simples. Como não funciona apenas para 
       * dobrar a parede vindo do Norte, vamos dá uma 
       * virada para este caso, para qualquer lado mais
       * próximo, assim cai nos casos que funcionam. */
      if s == Direcao::Sul && 
      (y as i32) < lin-2 &&
      x > 1 && (x as i32) < col-2
         { Direcao::Leste }
      else { s }
   }
}

/* retorna se a cobrinha ultrapassou as paredes
 * da tela gerada pelo ncurses.
 */
fn colidiu<'b>(cobra:&'b Cobrinha) -> bool {
   // dimensão da tela:
   let lin = getmaxy(stdscr());
   let col = getmaxx(stdscr());
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
   if x == 0 || y == 0 
      { return true; }
   // intervalo permitido nas duas dimensões.
   else 
      { !((x >= 1 && x <= col-2) && ( y >= 1 && y <= lin-2)) }
}

/* animação que pauso por um instante, para 
 * que se possa situar-se no jogo. A cobrinha
 * arranjada é mostrada. */
fn introducao() {
   // dimensão da tela.
   let colunas = getmaxx(stdscr());
   // núcleo da mensagem.
   let mensagem = String::from("O jogo inicia em ...");

   // computa o ponto do meio.
   let (recuo, lin, col):(i32, i32, i32) = (
      mensagem.len() as i32 / 2 + 1, 
               3,
      (colunas / 2) as i32
   );

   // escrevendo ...
   mv(lin, col - (recuo + 5));
   color_set(13);
   attrset(A_REVERSE());
   addstr(mensagem.as_str());
   addch(' ' as u32);
   // escrevendo contagem...
   for k in 0..=5 {
      // contagem está em ...
      let t = (5 - k).to_string();
      addstr(t.as_str());
      addch(' ' as u32);
      // tempo para próxima contagem..
      napms(700);
      refresh();
   }
   attroff(A_REVERSE());
   // limpa linha após mensagem colocada.
   mv(lin, 1);
   clrtoeol();
}

/* se alguma parte da borda estiver faltando,
 * então reconstruíla inteira.  */
#[allow(dead_code)]
fn conserta_borda() {
   // dimensão da tela.
   let (linhas, colunas) = (
      getmaxy(stdscr()), 
      getmaxx(stdscr())
   );
   let mut confirma = false;
   let espaco_branco = ' ' as u32;

   // varredura ...
   for col in 0..=colunas-1 {
      if mvinch(0, col) == espaco_branco 
         { confirma = true; }
      if mvinch(linhas-1, col) == espaco_branco 
         { confirma = true; }
   }

   for lin in 0..=linhas-1 {
      if mvinch(lin, 0) == espaco_branco
         { confirma = true; }
      if mvinch(lin, colunas-1) == espaco_branco
         { confirma = true; }
   }
   if confirma { 
      border(0, 0, 0, 0, 0, 0, 0, 0); 
      refresh();
   }
}

