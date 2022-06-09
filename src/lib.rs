
/**!
Importação de todas bibliotecas que foram 
construídas. 
Colocando aqui agora para que possa acessar e 
realizar testes, já que é preciso acessar o 
"caixote" para que utilize suas ferramentas.
*/

mod alvos;
mod modelos;
mod computacoes;
mod estatisticas;
mod serializacao;
mod banco_de_dados;
mod correcoes;

// importando todas funções úteis.
pub use modelos::*;
pub use computacoes::*;
pub use alvos::*;
pub use estatisticas::*;
pub use serializacao::*;
pub use banco_de_dados::*;
pub use correcoes::*;

// velocidade(tempo em miliseg de cada novo quadro).
pub const VELOCIDADE:i32 = 100;
