
/* funções bobas que são usadas como
 * auxiliares no projeto de serialização,
 * como lá, acabam transbordando o limite
 * de 300 linhas(mais ou menos este), então
 * são trazidas para cá, pelô motivo 
 * simples de refatoração. 
 */

use std::time::Duration;

/* pega o tempo em segundos e transforma
 * numa legitíma string, informando o 
 * tempo de forma legível. O range aqui
 * não é muito amplos, pois o jogo sempre
 * gera algo nestes intervalo(minutos e 
 * segundos). */
pub fn tempo_legivel(t:Duration) -> String {
   let tempo = t.as_secs_f32();
   if tempo > 60.0 
      { format!("{:0.1} min", tempo / 60.0) }
   else
      { format!("{} seg", tempo as u8) }
}

// pega o booleano e traduz em termos de vitória/derrota.
pub fn traduz(resultado:bool) -> &'static str {
   match resultado {
      true => "VENCEU",
      false => "PERDEU"
   }
}
