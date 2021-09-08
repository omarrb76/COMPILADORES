use std::collections::HashMap;

/* Esta estructura sera la que vaya dentro de la clase de Tabla de simbolos */
#[derive(Clone, Debug)]
struct Simbolo {
    variable: String,
    lineano: i32,
    //token: TokenType,
    //dtype: ExpType,
    valor: String
}

/* Clase pirata para la tabla de simbolos */
#[derive(Clone, Debug)]
struct TablaDeSimbolos {
    contenido: HashMap<String, Simbolo>
}

/* Implementacion de la clase */
impl TablaDeSimbolos {
    /* Comprobamos siempre si ya existe el simbolo, para no meterlo de nuevo */
    fn insertar(&mut self, llave: String, nuevo: Simbolo) {
        if !self.buscar(llave.clone()) { self.contenido.insert(llave, nuevo); }
        else { println!("Ya existe este simbolo! {:?}", nuevo); }
    }

    /* Actualizar el contenido de un simbolo */
    fn set(&mut self, llave: String, nuevo: Simbolo) {
        if !self.buscar(llave.clone()) { println!("No existe el elemento que deseas modificar"); }
        else { self.contenido.insert(llave, nuevo); }
    }

    /* Obtenemos el simbolo del contenido */
    fn get(&mut self, llave: String) -> Simbolo {
        match self.contenido.get(&llave) {
            Some(res) => return res.clone(),
            _ => {
                println!("No existe el simbolo que deseas acceder");
                let aux: Simbolo = Simbolo { 
                    variable: llave.clone(),
                    lineano: 0,
                    //token: TokenType::ERROR,
                    //dtype: ExpType::VOID,
                    valor: String::from("0")
                };
                return aux;
            }
        }
    }

    /* Remover de la tabla de Símbolos */
    fn remove(&mut self, llave: String) {
        self.contenido.remove(&llave);
    }

    /* Funcion que busca la llave dentro del hash */
    fn buscar(&mut self, llave: String) -> bool {
        let mut res = self.contenido.contains_key(&llave);
        if !res { return false; }
        else { return true; }
    }

    /* Funcion para imprimir */
    fn imprimir(&mut self) {
        println!("Imprimiendo los valores de la tabla de simbolos: {:?}", self);
    }
}

/* Probando la clase */
fn main() {
    let mut tabla: TablaDeSimbolos = TablaDeSimbolos { contenido: HashMap::new() };
    tabla.insertar(String::from("x"), Simbolo { variable: String::from("x"), lineano: 15, valor: String::from("hola")});
    tabla.insertar(String::from("x"), Simbolo { variable: String::from("x"), lineano: 15, valor: String::from("hola")});
    let mut res = tabla.get(String::from("x"));
    println!("Repsuesta: {:?}", res);
    tabla.set(String::from("x"), Simbolo { variable: String::from("x"), lineano: 15, valor: String::from("adios")});
    tabla.set(String::from("y"), Simbolo { variable: String::from("x"), lineano: 15, valor: String::from("adios")});
    res = tabla.get(String::from("x"));
    println!("Repsuesta: {:?}", res);
    tabla.get(String::from("f"));
    println!("{:?}", tabla);
}