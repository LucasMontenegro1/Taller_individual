use std::fs::File;
use std::io::Read;
use std::io::Result;

///
/// Lee un archivo dado un path, si el archivo es valido, devuelve su
/// contenido como string, caso contrario devuelve un error.
/// # Argumentos
/// * `path` - String que contiene el path del archivo a leer.
///
///
/// # Error:
/// En el caso de que el path del archivo sea inexistente o no se
/// pueda leer el contenido del mismo retorna error.
///
/// # Ejemplo:
///
/// ```rust
///     # use buscaminas::lector::leer_archivo;
///     leer_archivo("./buscaminas.txt".to_string());
/// ```
pub fn leer_archivo(path: String) -> Result<String> {
    let mut data = String::new();
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    match file.read_to_string(&mut data) {
        Ok(data) => data,
        Err(error) => return Err(error),
    };
    Ok(data)
}

#[test]
fn lector_devuelve_archivo_como_string() {
    let archivo = "./buscaminas.txt".to_string();
    let resultado = ".*.*.\n..*..\n..*..\n.....\n".to_string();
    match leer_archivo(archivo) {
        Ok(data) => assert_eq!(resultado, data),
        Err(_) => assert!(false),
    }
}

#[test]
fn lector_devuelve_error_con_archivo_inexistente() {
    let archivo = "./no_existe.txt".to_string();
    match leer_archivo(archivo) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}
