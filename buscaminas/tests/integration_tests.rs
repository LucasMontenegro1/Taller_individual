use buscaminas::{buscaminas::Buscaminas, lector::leer_archivo};

extern crate buscaminas;

// importing common module.
mod common;

#[test]
fn buscaminas_cuenta_correctamente() {
    let buscaminas = common::setup();
    let resultado = "1*3*1\n13*31\n.2*2.\n.111.\n".to_string();
    assert_eq!(resultado, buscaminas.buscar());
}

#[test]
fn buscaminas_falla_con_formato_incorrecto() {
    let buscaminas = Buscaminas::new("..7..".as_bytes());
    match buscaminas {
        Ok(_) => assert!(false, "deberia devolver error"),
        Err(_) => assert!(true),
    }
}

#[test]
fn buscaminas_falla_con_matriz_no_rectangular() {
    let buscaminas = Buscaminas::new("....\n...".as_bytes());
    match buscaminas {
        Ok(_) => assert!(false, "deberia devolver error"),
        Err(_) => assert!(true),
    }
}

#[test]
fn buscaminas_no_encuentra_el_archivo() {
    let path = "./archivo_inexistente".to_string();
    assert!(matches!(leer_archivo(path), Err(_)));
}
