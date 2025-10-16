/* Cria link simbólico tanto para a versão em debug, quanto para o binário 
 * final. */
#[cfg(target_os="unix")]
use std::os::unix::fs::symlink;
use std::env::current_exe;
use std::path::{PathBuf};

pub fn computa_caminho(caminho_str:&str) -> PathBuf {
// Complementa link ao executável à partir do caminho do executável ...
   match current_exe() {
      Ok(mut base) => {
         // remove executável do caminho.
         base.pop(); 
         // sai do subdiretório 'release'.
         base.pop(); 
         // sai do subdiretório 'target'.
         base.pop();
         // complementa com o caminho passado.
         base.push(caminho_str);

         base
      } Err(_) =>
         { panic!("não foi possível obter o caminho do executável!"); }
   }
}

#[cfg(target_os="unix")]
pub fn linka_executaveis(nome: &str) {
   // caminho aos executáveis.
   let caminho_str = "target/release/cobrinha_classica";
   let executavel = computa_caminho(caminho_str);
   let caminho_str = "target/debug/cobrinha_classica";
   let executavel_debug: PathBuf = computa_caminho(caminho_str);

   // seus links simbólicos:
   let ld_link = computa_caminho(nome);
   let mut nome_debug = nome.to_string();
   nome_debug.push_str("_debug");
   let ld_debug_link = computa_caminho(nome_debug.as_str());

   if ld_link.as_path().exists() && 
   ld_link.as_path().is_symlink() {
      if executavel.as_path().exists() 
         { println!("binário do executável existe."); }
   } else {
      print!("criando '{}' ... ", nome);
      match symlink(executavel.as_path(), ld_link.as_path()) {
         Ok(_) => {
            println!("com sucesso.");
         } Err(_) => 
            { println!("executável não existe!"); }
      };
   }

   if ld_debug_link.as_path().exists() && 
   ld_link.as_path().is_symlink() { 
      if executavel_debug.exists() 
         { println!("binário do executável(DEBUG) existe."); }
   } else {
      print!("criando '{}'(debug) ... ", nome_debug);
      match symlink(executavel_debug.as_path(), ld_debug_link.as_path()) {
         Ok(_) => {
            println!("com sucesso.");
         } Err(_) => 
            { println!("executável não existe!"); }
      };
   }
}

#[cfg(target_os="unix")]
pub fn linca_executaveis_externamente(nome: &str) -> 
  Result<PathBuf, std::io::ErrorKind> 
{
   // caminho aos executáveis.
   let executavel = current_exe().unwrap();
   // destino do linque agora é no global, se houver um é claro.
   let destino: &'static str = env!("LINKS");
   let linque = Path::new(destino).join(nome);
   let ja_existe_um_linque_simbolico = {
      linque.exists() && 
      linque.is_symlink()
   };

   if cfg!(debug_assertions) { 
      println!("resultado do link='{}'", linque.display()); 
      println!("existência? {}", ja_existe_um_linque_simbolico);
   }
      
   if ja_existe_um_linque_simbolico {
      println!("binário do executável já existe em {}.", destino); 
      // apenas retorna o linque que já existe!
      return Ok (linque);
   } 

   print!("criando '{}' ... ", nome);
   match symlink(executavel.as_path(), &linque) {
      Ok(_) => {
         println!("com sucesso.");
         // apenas retorna caminho do linque criado.
         Ok (linque)
      } Err(_) => { 
         println!("executável não existe!"); 
         // erro informando que tal executável não existe.
         Err (std::io::ErrorKind::NotFound)
      }
   }
}

#[cfg(target_os="windows")]
pub fn linka_executaveis(_nome: &str) 
   { panic!("{}", std::io::ErrorKind::Unsupported); }

#[cfg(target_os="windows")]
pub fn linca_executaveis_externamente(_nome: &str) -> 
  Result<PathBuf, std::io::ErrorKind> 
   { Err(std::io::ErrorKind::Unsupported) }

#[cfg(test)]
mod tests {
   use std::fs::remove_file;

   #[test]
   fn criando_simples_linque_externo_ao_caixote() {
      let _path = {
         super::linca_executaveis_externamente
         ("cobrinha-link").unwrap()
      };
      assert!(&_path.exists());
      remove_file(&_path).unwrap();
      assert!(!_path.exists());
   }
}
