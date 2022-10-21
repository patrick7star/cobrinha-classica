
// biblioteca externa:
extern crate rand;
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
   introducao, piloto_automatico
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

/* desenha a cobrinha onde quer que ela vá. Com
 * a array de direções que são dado para ela "virar"
 * a cada novo passo. 
 */
pub fn roda_jogo(janela: &Window, obj:&mut Cobrinha, obj_metas:&mut Alvos) -> Dados {
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

   // apresentação ao iniciar o jogo.
   plota_cobrinha(janela, obj);
   introducao(janela);
   
   // enquanto todos alvos/bichos não se forem...
   while !obj_metas.sem_alvos() && !colidiu(obj, linhas, colunas) {
      // colhendo dados antes do próximo movimento.
      metadados.atualiza(obj, obj_metas);

      // sentido não oposto ao do atual.
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
fn barra_status_flutuante(
janela: &Window, cobra:&Cobrinha, 
bichos:&Alvos, ja_apagada:&mut bool) {
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
fn animacao_final(
   janela: &Window, cobra:&mut Cobrinha, 
   alvos:&mut Alvos, duracao: u64, dados: &Dados
) {
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
   let iterador = dados_str.lines().skip(2);

   for (k, linha) in iterador.enumerate() {
      janela.mv(y+(k as i32), x);
      let linha = linha.trim_start().trim_start_matches('\r');
      janela.addstr(linha);
   }

   janela.refresh();
}



