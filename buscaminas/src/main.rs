use std::io::{ErrorKind, Result};
use std::{env, io};

mod buscaminas;
mod lector;
use crate::buscaminas::Buscaminas;
use crate::lector::leer_archivo;

/// Imprime, a traves de la linea de comandos, la matriz del buscaminas contando
/// las minas adyacentes de las distintas posiciones vacias
///
/// # Argumento
///  recibe por linea de comandos el path de un archivo con la matriz del
/// buscaminas
///  
/// # Error
///  se retorna error al haber algun problema en la apertura del archivo.
///
/// # Ejemplo
/// ```
/// cargo run "./buscaminas.txt"
/// ```
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(io::Error::new(
            ErrorKind::Other,
            "cantidad incorrecta de argumentos",
        ));
    }
    let data = match leer_archivo(args[1].to_string()) {
        Ok(data) => data,
        Err(error) => return Err(error),
    };

    let buscaminas = match Buscaminas::new(data.as_bytes()) {
        Ok(buscaminas) => buscaminas,
        Err(error) => return Err(error),
    };
    println!("{}", buscaminas.buscar());
    Ok(())
}
