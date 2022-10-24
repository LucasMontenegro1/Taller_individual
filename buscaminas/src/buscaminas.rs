use std::io::{self, ErrorKind, Result};

///Representa el buscaminas y su tablero con ancho y largo
pub struct Buscaminas {
    ///el tablero se trata de una matriz representada como un vector de vectores u8
    tablero: Vec<Vec<u8>>,
    filas: usize,
    columnas: usize,
}

impl Buscaminas {
    ///Funcion que crea la matriz resultante del slice
    /// devolviendo un vector de vectores de u8 en caso de una correcta
    /// ejecucion o un error en el caso de fallar.
    fn crear_matriz(data: &[u8]) -> Result<Vec<Vec<u8>>> {
        let mut fila = vec![];
        let mut last = b'.';
        let mut matriz = vec![];
        for i in data {
            if *i == b'\n' && last != b'\n' {
                matriz.push(fila);
                fila = vec![];
            } else if *i == b'*' || *i == b'.' {
                fila.push(*i);
            } else if *i != b'*' && *i != b'.' && *i != b'\n' {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    "caracteres invalidos en la matriz",
                ));
            }
            last = *i;
        }
        if !fila.is_empty() {
            matriz.push(fila);
        }
        for i in 0..matriz.len() {
            if matriz[0].len() != matriz[i].len() {
                return Err(io::Error::new(ErrorKind::Other, "matriz no rectangular"));
            }
        }
        Ok(matriz)
    }
    /// Crea un nuevo buscaminas que contiene la matriz
    /// así como tambien la cantidad de filas y columnas
    /// de la misma.
    ///
    ///
    /// # Argumentos
    /// * `data` - Slice de u8 que contiene la matriz inicial como caracteres u8
    ///
    /// # Retorno
    /// La funcion devuelve un Result, si la creación fue exitosa se trata de un
    /// Buscaminas, caso contrario, un error.
    ///
    /// # Ejemplo:
    /// ```rust
    ///     # use buscaminas::buscaminas::Buscaminas;
    ///     let data = ".*.*.\n..*..\n..*..\n.....\n".to_string();
    ///     let buscaminas = match Buscaminas::new(data.as_bytes()){
    ///         Ok(buscaminas)=>buscaminas,
    ///         Err(_)=>panic!("error")
    /// };
    /// ```
    ///
    pub fn new(data: &[u8]) -> Result<Buscaminas> {
        let matriz = match Self::crear_matriz(data) {
            Ok(matriz) => matriz,
            Err(error) => return Err(error),
        };
        let filas = matriz.len();
        let columnas = matriz[0].len();
        Ok(Buscaminas {
            tablero: matriz,
            filas,
            columnas,
        })
    }
    ///Busca todas las minas cercanas dado el buscaminas
    ///recorre la matriz y segun sea el caracter de la misma busca las minas adyacentes
    /// y devuelve un string con la matriz ya completada.
    ///
    ///
    /// # Ejemplo
    /// ```
    ///     # use buscaminas::buscaminas::Buscaminas;
    ///     let data = ".*.*.\n..*..\n..*..\n.....\n".to_string();
    ///     let buscaminas = Buscaminas::new(data.as_bytes()).unwrap();
    ///     buscaminas.buscar();
    /// ```
    /// Dado un String con el siguiente formato:
    /// ```text
    /// .*.*.
    /// ..*..
    /// ..*..
    /// .....
    ///```
    ///
    /// Al aplicar la funcion, deberiamos de obtener un string
    /// con el siguiente formato:
    ///
    ///```text
    /// 1*3*1
    /// 13*31
    /// .2*2.
    /// .111.
    ///```
    ///
    pub fn buscar(&self) -> String {
        let mut out = String::new();
        for i in 0..self.filas {
            for j in 0..self.columnas {
                if self.tablero[i][j] == b'.' {
                    let resultado = self.get_cantidad_de_minas(i, j);
                    if resultado == 0 {
                        out.push('.');
                    } else {
                        out.push((self.get_cantidad_de_minas(i, j) + b'0') as char);
                    }
                } else {
                    out.push('*');
                }
            }
            out.push('\n');
        }
        out
    }

    /// Cuenta la cantidad de minas adyacentes dada una posicion de la matriz del buscaminas
    /// y las devuelve como un u8, en el caso de no tener minas adyacentes retorna '0'
    ///
    ///
    /// # Argumentos
    /// * `i` - usize que indica la fila de la posicion a analizar
    /// * `j` - usize que indica la columna de la posicion a analizar
    ///
    ///
    pub fn get_cantidad_de_minas(&self, i: usize, j: usize) -> u8 {
        let mut cantidad = 0;
        let mut inicio_x = 0;
        if i != 0 {
            inicio_x = i - 1;
        }
        let mut inicio_y = 0;
        if j != 0 {
            inicio_y = j - 1;
        }
        for x in inicio_x..(i + 2) {
            for y in inicio_y..(j + 2) {
                if x < self.filas && y < self.columnas && self.tablero[x][y] == b'*' {
                    cantidad += 1;
                }
            }
        }

        cantidad
    }
}

//test unitarios
#[test]
fn buscaminas_se_crea_exitosamente() {
    let data: &[u8] = &[
        46, 42, 46, 42, 46, 46, 46, 46, 10, 46, 46, 42, 46, 46, 46, 46, 46, 10,
    ];
    let primer_fila: Vec<u8> = vec![46, 42, 46, 42, 46, 46, 46, 46];
    let segunda_fila: Vec<u8> = vec![46, 46, 42, 46, 46, 46, 46, 46];

    let mut matriz = vec![];
    matriz.push(primer_fila);
    matriz.push(segunda_fila);
    let buscaminas = Buscaminas::new(data);
    match buscaminas {
        Ok(buscaminas) => assert_eq!(matriz, buscaminas.tablero),
        Err(_) => assert!(false),
    }
}

#[test]
fn el_numero_de_filas_es_el_correcto() {
    let data: &[u8] = &[
        46, 42, 46, 42, 46, 46, 46, 46, 10, 46, 46, 42, 46, 46, 46, 46, 46, 10,
    ];
    let buscaminas = Buscaminas::new(data);
    match buscaminas {
        Ok(buscaminas) => assert_eq!(2, buscaminas.filas),
        Err(_) => assert!(false),
    }
}

#[test]
fn el_numero_de_columnas_es_el_correcto() {
    let data: &[u8] = &[
        46, 42, 46, 42, 46, 46, 46, 46, 10, 46, 46, 42, 46, 46, 46, 46, 46, 10,
    ];
    let buscaminas = Buscaminas::new(data);
    match buscaminas {
        Ok(buscaminas) => assert_eq!(8, buscaminas.columnas),
        Err(_) => assert!(false),
    }
}

#[test]
fn numero_de_minas_adyacentes_correcto() {
    let data = ".*\n".to_string();
    let resultado = '1';

    let buscaminas = Buscaminas::new(data.as_bytes());
    match buscaminas {
        Ok(buscaminas) => assert_eq!(
            resultado,
            (buscaminas.get_cantidad_de_minas(0, 0) + b'0') as char
        ),
        Err(_) => assert!(false),
    }
}

#[test]
fn una_mina_cercana() {
    let data = ".*\n".to_string();
    let resultado = "1*\n".to_string();

    let buscaminas = Buscaminas::new(data.as_bytes());
    match buscaminas {
        Ok(buscaminas) => assert_eq!(resultado, buscaminas.buscar()),
        Err(_) => assert!(false),
    }
}

#[test]
fn buscaminas_funciona_correctamente() {
    let data = ".*.*.\n..*..\n..*..\n.....\n".to_string();
    let resultado = "1*3*1\n13*31\n.2*2.\n.111.\n".to_string();

    let buscaminas = Buscaminas::new(data.as_bytes());
    match buscaminas {
        Ok(buscaminas) => assert_eq!(resultado, buscaminas.buscar()),
        Err(_) => assert!(false),
    };
}
