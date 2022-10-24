use buscaminas::buscaminas::Buscaminas;
use buscaminas::lector::leer_archivo;

pub fn setup() -> Buscaminas {
    let archivo = "./buscaminas.txt".to_string();
    let tablero = leer_archivo(archivo).unwrap();
    let buscaminas = Buscaminas::new(tablero.as_bytes()).unwrap();
    buscaminas
}
