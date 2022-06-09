
/* Alguns traits para serialização de 
 * objetos específicos, que são muito
 * comuns nos programas aqui codificados.
 */


// para o único tipo primitivo que não têm 'to_be_bytes'.
pub trait UnicoByte {
   // pode ser codificado num único byte.
   fn serializa(&self) -> u8;
}

/** Serialização de um tipo de dado dinâmico, que
 * pode aumentar ou diminuir ao longo da execução.
 */
pub trait Serializacao {
   /* serão gravados na ordem que foram
    * codificados, e, por consequência,
    * a deserialização segue a mesma 
    * regra. 
    * numa array-dinâmica, pois pode variar sempre
    * para o mesmo tipo de dado. */
   fn serializa(&self) -> Vec<u8>;
}
