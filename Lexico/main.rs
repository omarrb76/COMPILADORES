use std::env;                                   // Para usar los argumentos del main
use std::process;                               // Para el mensaje de error
use std::fs::File;                              // Para leer el archivo
use std::io::{self, prelude::*, BufReader};     // Para leer el archivo
use std::fmt::{self, Debug, Display};           // Para poder pasar un enum a String
use std::collections::HashMap;                  // Para la tabla de simbolos

/************ PERTENECIENTE AL ANÁLISIS LÉXICO ******************/
// Aunque el TokenType también pertenece al análisis sintáctico
#[derive(Debug, Copy, Clone, PartialEq)]
enum TokenType {
    /* PALBRAS RESERVADAS */
    PROGRAM, IF, ELSE, FI, DO, UNTIL, WHILE, READ, WRITE, FLOAT, INT, BOOL, NOT, AND, OR, TRUE, FALSE, THEN,
    /* SIMBOLOS ESPECIALES */
    PLUS, MINUS, TIMES, DIVISION, POW, LT, LTE, GT, GTE, EQ, DIFF, ASSIGN, SEMI, COMA, LPAREN, RPAREN, LBRACKET, RBRACKET,   
    /* TOKENS DE VARIOS CARACTERES */
    ID, NUMINT, NUMFLOAT,        
    /* COMENTARIOS */
    BLOCKCOMM, LINECOMM,   
    /* BOOK KEEPING TOKENS */
    ENDFILE, ERROR,
    /* ERRORES */
    INT_FLOAT_BOOL_LPAREN, STATEMENT_INITIALIZER, CONST_ID_LPAREN
}
// Pasar de TokenType a String
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Estado en el que se encuentra actualmente (para el analizador léxico getToken)
#[derive(Debug)]
enum StateType {
    START, INASSIGN, INCOMMENT, INNUMINT, INNUMFLOAT, INID, INLTE, INEQ, INGTE, INDIFF, DONE
}

// Estructura que contiene la información de cada Token
#[derive(Clone)]
struct Token {
    lexema: String,
    token: TokenType,
    linea: i32
}

/************ PERTENECIENTE AL ANÁLISIS SINTÁCTICO ***************/
// De qué tipo de nodo se trata, uno de statement, expresion o un nodo vacio
// El nodo vacio son para los errores
#[derive(Copy, Clone, Debug)]
enum NodeKind { STMT, EXP, EMPTY }

// Que tipo de statement estamos haciendo
#[derive(Copy, Clone, Debug)]
enum StmtKind { PROGRAM, IF, WHILE, DO, READ, WRITE, ASSIGN, DECLARE, UNDEFINED }
// Pasar StmtKind a String
impl fmt::Display for StmtKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Tipo de expresion, operador, constante (numero) o identificador
#[derive(Copy, Clone, Debug, PartialEq)]
enum ExpKind  { OP, CONST, ID, UNDEFINED }

// Tipo de variable
#[derive(Copy, Clone, Debug, PartialEq)]
enum ExpType  { VOID, INT, BOOL, FLOAT }

// Este es el nodo del arbol, tiene muchisimos atributos, de los cuales no creo que sean
// necesarios todos forzosamente.
// El hijo1,2,3 son para los hijos, el hermano es un nodo al mismo nivel que el nodo
// lineano, nos indica en que linea esta ese token (nodo), si hay algun error el usuario
// puede ver en que linea se encontro el error. Lo demas son el contenido, varia mucho dependiendo
// del tipo de nodo que sea el arbol.
#[derive(Clone, Debug)]
struct TreeNode {
    hijo1: Option<Box<TreeNode>>,
    hijo2: Option<Box<TreeNode>>,
    hijo3: Option<Box<TreeNode>>,
    hermano: Option<Box<TreeNode>>,
    valor: String,
    token: TokenType,
    dtype: ExpType,
    lineano: i32,
    tipo_nodo: NodeKind,
    kind_stmt: StmtKind
}

/************ PERTENECIENTE AL ANÁLISIS SEMÁNTICO ***************/

// Variables globales
// Posicion en la que nos encontramos
static mut token_actual: usize = 0;

// Arreglo que contendra todos los tokens que vayamos recuperando
static mut token_array: Vec<Token> = Vec::new();

/* Esta estructura sera la que vaya dentro de la clase de Tabla de simbolos */
#[derive(Clone, Debug)]
struct Simbolo {
    variable: String,
    lineano: i32,
    token: TokenType,
    dtype: ExpType,
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
                println!("No existe el simbolo que deseas acceder, variable: {}", llave);
                let aux: Simbolo = Simbolo { 
                    variable: llave.clone(),
                    lineano: 0,
                    token: TokenType::ERROR,
                    dtype: ExpType::VOID,
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
        println!("Imprimiendo los valores de la tabla de simbolos:");
        for (key, value) in &self.contenido {
            println!("{:?} | {:?}", key, value);
        }
    }
}

fn main() -> io::Result<()> {

    // Obtenemos los parámetros del main
    let args: Vec<String> = env::args().collect();

    // Si no nos dieron 1 documento, saltará un error
    if args.len() != 2 {
        eprintln!("Error en los parámetros del main");
        process::exit(1);
    }

    // Abrimos y leemos el archivo
    let file = File::open(args[1].to_string())?;
    let reader = BufReader::new(file);

    // Variables de control
    let mut lineano = 1;                        // Para contar las líneas
    let mut palabra = String::from("");         // Palabra que separamos en la línea
    let mut com_linea : bool = false;           // Para saber si tomamos en cuenta los lexemas o no
    let mut com_bloque : bool = false;          // Son banderas para saber cuando inicia y acaba un comentario
    let mut com_flag_close : bool = false;      // Bandera de comentario cerrado, para saber si el anterior fue un "*"
    let mut com_flag_open : bool = false;       // Bandera que me indica si recibimos un "/" y saber si el siguiente es "/" hacer comentario
    let mut special_simbol : bool = false;      // Porque RUST no tiene muchas cosas utiles, necesitamos varias banderas
    let mut special_simbol_char : char = 'a';

    for linea in reader.lines() { // Leemos linea por linea

        for c in linea?.chars() { // Caracter por caracter de cada linea
            
            // Si esta comentado, no tiene sentido estar haciendo los tokens
            if com_linea || com_bloque {

                if c == '*' { com_flag_close = true; }                                              // Si el siguiente caracter es '/' se quita el flag de comentario
                else if c == '/' && com_flag_close { com_bloque = false; com_flag_close = false; }  // desactivamos las banderas de comentarios
                continue;                                                                           // Nos saltamos todo lo de abajo

            }

            if com_flag_open { // Teóricamente el caracter anterior fue un '/' y se vació la palabra

                if c == '/' { // Mandamos a procesar un comentario de linea y activamos la bandera de comentario de linea
                    get_token(&String::from("//"), lineano);
                    com_linea = true;
                } else if c == '*' { // Mandamos a procesar un comentario de bloque y activamos la bandera de comentario de bloque
                    get_token(&String::from("/*"), lineano);
                    com_bloque = true;
                } else { get_token(&String::from("/"), lineano); } // Mandamos a procesar una division
                com_flag_open = false; // Desactivamos la bandera de "/"
                continue; // Nos saltamos al siguiente ciclo

            }

            if special_simbol { // Teóricamente aqui tendrá uno de estos símbolos: < > ! = y se vació la palabra

                special_simbol = false;
                if c == '=' { 
                    get_token(&String::from(special_simbol_char.to_string() + "="), lineano);   // Mandamos a llamar a un <= >= != ==
                    continue;
                } 
                else { get_token(&special_simbol_char.to_string(), lineano); }  // Lo mandamos asi solito < > ! =

            }

            if c != ' ' && c != '\t' { // Si es diferente de espacio o tabulador
                if  c == '+' || c == '-' || c == '*' || c == '^' || c == ';' ||
                    c == ',' || c == '(' || c == ')' || c == '{' || c == '}' { // Estos caracteres son tokens por si solos y no necesitan nada mas, por eso se mandan a procesar al instante

                    if palabra != "" { get_token(&palabra, lineano); } // Si hay algo en la palabra
                    get_token(&c.to_string(), lineano); // Mandamos a procesar el caracter
                    palabra = String::from(""); // Reiniciamos la palabra a vacia porque ya se mando a procesar y se tiene que vaciar

                } else if c == '/' { // Si recibimos este simbolo encendemos la bandera de un posible comentario y hacemos lo mismo que arribita

                    com_flag_open = true;
                    if palabra != "" { get_token(&palabra, lineano); }
                    palabra = String::from("");

                } else if c == '<' || c == '>' || c == '=' || c == '!' {

                    special_simbol = true;
                    special_simbol_char = c;
                    if palabra != "" { get_token(&palabra, lineano); }
                    palabra = String::from("");

                } else { // Si no fue ninguno de los casos de arriba solo metemos el caracter a la palabra

                    palabra.push(c);

                }
                
            } else { // Word boundary

                if palabra != "" { get_token(&palabra, lineano); }
                palabra = String::from("");

            }
        }

        // Mandamos a procesar la ultima palabra
        if palabra != "" { get_token(&palabra, lineano); }
        palabra = String::from("");

        com_linea = false;   // Debido al salto de linea, el comentario de linea se acaba
        lineano += 1;       // Sumamos en uno el numero de línea

    }

    unsafe {

        // Agregamos el último token de fin de línea
        token_array.push(Token {
            lexema: String::from("EOF"),
            token: TokenType::ENDFILE,
            linea: lineano - 1
        });

        // Imprimimos toda la información
        for token in token_array.iter() { println!("Lexema: {} | Linea: {} | Token: {:?}", token.lexema, token.linea, token.token); }

    }

    // Imprimos lo que sigue de sintactico
    println!("------ SINTACTICO ERRORES ------");
    let mut t: TreeNode = programa();
    println!("------ SINTACTICO ARBOL   ------");
    imprimir_arbol(t.clone(), 0);

    // Empieza el análisis semántico
    // Creamos la tabla de símbolos
    let mut tabla_simbolos: TablaDeSimbolos = TablaDeSimbolos { contenido: HashMap::new() };
    evalType(&mut t, &mut tabla_simbolos);

    tabla_simbolos.imprimir();

    Ok(())

}

// Funcion que procesa la palabra y nos regresa un Token
fn get_token(lexeme: &String, lineano: i32) -> () {

    // Estado inicial del token
    let mut token : TokenType = TokenType::ERROR;
    let mut state : StateType = StateType::START;

    // Si es alguna palabra reservada
    if      lexeme == "program" { token = TokenType::PROGRAM;   }
    else if lexeme == "true"    { token = TokenType::TRUE;      }
    else if lexeme == "false"   { token = TokenType::FALSE;     }
    else if lexeme == "if"      { token = TokenType::IF;        }
    else if lexeme == "then"    { token = TokenType::THEN;      }
    else if lexeme == "else"    { token = TokenType::ELSE;      }
    else if lexeme == "fi"      { token = TokenType::FI;        }
    else if lexeme == "do"      { token = TokenType::DO;        }
    else if lexeme == "until"   { token = TokenType::UNTIL;     }
    else if lexeme == "while"   { token = TokenType::WHILE;     }
    else if lexeme == "read"    { token = TokenType::READ;      }
    else if lexeme == "write"   { token = TokenType::WRITE;     }
    else if lexeme == "float"   { token = TokenType::FLOAT;     }
    else if lexeme == "int"     { token = TokenType::INT;       }
    else if lexeme == "bool"    { token = TokenType::BOOL;      }
    else if lexeme == "not"     { token = TokenType::NOT;       }
    else if lexeme == "and"     { token = TokenType::AND;       }
    else if lexeme == "or"      { token = TokenType::OR;        }
    else { // No fue una palabra reservada, veamos de que trata

        for c in lexeme.chars() {
            
            // Automata finito
            match state {
                StateType::START => {
                    if      c.is_digit(10)      { state = StateType::INNUMINT;      token = TokenType::NUMINT;   }
                    else if c == '_'            { state = StateType::INID;          token = TokenType::ID;       }
                    else if c.is_alphabetic()   { state = StateType::INID;          token = TokenType::ID;       }
                    else if c == '<'            { state = StateType::INLTE;         token = TokenType::LT;       }
                    else if c == '='            { state = StateType::INEQ;          token = TokenType::ASSIGN;   }
                    else if c == '>'            { state = StateType::INGTE;         token = TokenType::GT;       }
                    else if c == '!'            { state = StateType::INDIFF;        token = TokenType::DIFF;     }
                    else if c == '/'            { state = StateType::INCOMMENT;     token = TokenType::DIVISION; }
                    else {
                        match c {
                            '+' => token = TokenType::PLUS,
                            '-' => token = TokenType::MINUS,
                            '*' => token = TokenType::TIMES,
                            '^' => token = TokenType::POW,
                            '(' => token = TokenType::LPAREN,
                            ')' => token = TokenType::RPAREN,
                            ';' => token = TokenType::SEMI,
                            ',' => token = TokenType::COMA,
                            '{' => token = TokenType::LBRACKET,
                            '}' => token = TokenType::RBRACKET,
                            _   => {
                                token = TokenType::ERROR;
                                state = StateType::DONE;
                            }
                        }
                    }
                },
                StateType::INNUMINT => {
                    if !c.is_digit(10) { 
                        if c == '.' { state = StateType::INNUMFLOAT;    token = TokenType::NUMFLOAT; }
                        else        { state = StateType::DONE;          token = TokenType::ERROR;    } 
                    }
                    else { token = TokenType::NUMINT; }
                },
                StateType::INNUMFLOAT => {
                    if !c.is_digit(10)  { state = StateType::DONE; token = TokenType::ERROR; }
                    else                { token = TokenType::NUMFLOAT; }
                },
                StateType::INID => {
                    if !c.is_alphanumeric() 
                    && !(c == '_')          { state = StateType::DONE; token = TokenType::ERROR;    }
                    else                    { token = TokenType::ID;                                }
                },
                StateType::INLTE => {
                    if c == '='             { token = TokenType::LTE;       }
                    else                    { token = TokenType::ERROR;     }
                },
                StateType::INEQ => {
                    if c == '='             { token = TokenType::EQ;        }
                    else                    { token = TokenType::ERROR;     }
                },
                StateType::INGTE => {
                    if c == '='             { token = TokenType::GTE;       }
                    else                    { token = TokenType::ERROR;     }
                },
                StateType::INDIFF => {
                    if c == '='             { token = TokenType::DIFF;      }
                    else                    { token = TokenType::ERROR;     }
                },
                StateType::INCOMMENT => {
                    if c == '/'             { token = TokenType::LINECOMM;  }
                    else if c == '*'        { token = TokenType::BLOCKCOMM; }
                    else                    { token = TokenType::ERROR;     }
                },
                StateType::DONE =>          { break;                        },
                _ => {
                    println!("Scanner bug, estado: {:?}", state);
                    state = StateType::DONE;
                    token = TokenType::ERROR;
                }
            }
        }
    }

    // Metemos el Token en el arreglo de Tokens (este se usará después a la hora del análisis sintáctico)
    unsafe {
        token_array.push(Token {
            lexema: lexeme.to_string(),
            token: token,
            linea: lineano
        });
    }
    
}

// Funciones del análisis sintáctico, estas son potencialmente recursivas y dificil de seguir
// en resumen, crean un nodo, a ese nodo le agregan hijos y despues lo devuelven a alguna otra funcion que los haya llamado
// Seguir el procedimiento de estas funciones es muy dificil, si se quiere probar que esa creando el arbol
// de manera correcta, mejor usa la funcion de imprimir arbol.

// programa ::= program ”{” lista-declaración lista-sentencias ”}”
fn programa() -> TreeNode {
    //unsafe { println!("ENTRE PROGRAMA --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newStmtNode(StmtKind::PROGRAM);
    coincide(TokenType::PROGRAM);
    coincide(TokenType::LBRACKET);
    unsafe {
        if token_array[token_actual].token != TokenType::RBRACKET {
            t.hijo1 = Some(Box::new(lista_declaracion()));
            t.hijo2 = Some(Box::new(lista_sentencia()));
        }
    }
    coincide(TokenType::RBRACKET);
    return t;
}

// lista-declaración ::= { declaración }
fn lista_declaracion() -> TreeNode {
    //unsafe { println!("ENTRE LISTA_DECLARACION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = declaracion();
    unsafe {
        if 
        token_array[token_actual].token == TokenType::INT || token_array[token_actual].token == TokenType::FLOAT ||
        token_array[token_actual].token == TokenType::BOOL {

            t.hermano = Some(Box::new(lista_declaracion()));

        }
    }
    return t;
}

// declaración ::= tipo lista-id ”;”
// tipo ::= int | float | bool
fn declaracion() -> TreeNode {
    //unsafe { println!("ENTRE DECLARACION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newStmtNode(StmtKind::DECLARE);
    unsafe {
        match token_array[token_actual].token {
            TokenType::INT => {
                t.dtype = ExpType::INT;
                coincide(TokenType::INT);
                t.hijo1 = Some(Box::new(lista_id()));
            },
            TokenType::FLOAT => {
                t.dtype = ExpType::FLOAT;
                coincide(TokenType::FLOAT);
                t.hijo1 = Some(Box::new(lista_id()));
            },
            TokenType::BOOL => {
                t.dtype = ExpType::BOOL;
                coincide(TokenType::BOOL);
                t.hijo1 = Some(Box::new(lista_id()));
            },
            _ => {
                t = newErrorNode();
                error(TokenType::INT_FLOAT_BOOL_LPAREN);
            }
        }
    }
    coincide(TokenType::SEMI);
    return t;
}

// lista-id ::= identificador { ”,” identificador }
fn lista_id() -> TreeNode {
    //unsafe { println!("ENTRE LISTA_ID --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newExpNode();
    coincide(TokenType::ID);
    unsafe {
        if token_array[token_actual].token == TokenType::COMA {
            coincide(TokenType::COMA);
            t.hermano = Some(Box::new(lista_id()));
        }
    }
    return t;
}

// lista-sentencias ::= { sentencias }
fn lista_sentencia() -> TreeNode {
    //unsafe { println!("ENTRE LISTA_SENTENCIA --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = sentencia();
    unsafe {
        if 
        token_array[token_actual].token == TokenType::IF       || token_array[token_actual].token == TokenType::WHILE    ||
        token_array[token_actual].token == TokenType::DO       || token_array[token_actual].token == TokenType::READ     ||
        token_array[token_actual].token == TokenType::WRITE    || token_array[token_actual].token == TokenType::LBRACKET || 
        token_array[token_actual].token == TokenType::ID {

            t.hermano = Some(Box::new(lista_sentencia()));

        }
    }
    return t;
}

// sentencia ::= selección | iteración | repetición | sent-read | sent-write | bloque | asignación
fn sentencia() -> TreeNode {
    //unsafe { println!("ENTRE SENTENCIA --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    unsafe {
        match token_array[token_actual].token {
            TokenType::IF =>        { t = seleccion();  },
            TokenType::WHILE =>     { t = iteracion();  },
            TokenType::DO =>        { t = repeticion(); },
            TokenType::READ =>      { t = sent_read();  },
            TokenType::WRITE =>     { t = sent_write(); },
            TokenType::LBRACKET =>  { t = bloque();     },
            TokenType::ID =>        { t = asignacion(); },
            _ => {
                t = newErrorNode();
                error(TokenType::STATEMENT_INITIALIZER);
            }
        }
    }
    return t;
}

// selección ::= if ”(” b-expresión ”)” then bloque [ else bloque ] fi
fn seleccion() -> TreeNode {
    //unsafe { println!("ENTRE SELECCION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newStmtNode(StmtKind::IF);
    coincide(TokenType::IF);
    coincide(TokenType::LPAREN);
    t.hijo1 = Some(Box::new(b_expresion()));
    coincide(TokenType::RPAREN);
    coincide(TokenType::THEN);
    t.hijo2 = Some(Box::new(bloque()));
    unsafe {
        if token_array[token_actual].token == TokenType::ELSE {
            coincide(TokenType::ELSE);
            t.hijo3 = Some(Box::new(bloque()));
        }
    }
    coincide(TokenType::FI);
    return t;
}

// iteración ::= while ”(” b-expresión ”)” bloque
fn iteracion() -> TreeNode {
    //unsafe { println!("ENTRE ITERACION (WHILE) --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newStmtNode(StmtKind::WHILE);
    coincide(TokenType::WHILE);
    coincide(TokenType::LPAREN);
    t.hijo1 = Some(Box::new(b_expresion()));
    coincide(TokenType::RPAREN);
    t.hijo2 = Some(Box::new(bloque()));
    return t;
}

// repetición ::= do bloque until "(" b-expresión ")" ";"
fn repeticion() -> TreeNode {
    //unsafe { println!("ENTRE REPETICION (DO) --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newStmtNode(StmtKind::DO);
    coincide(TokenType::DO);
    t.hijo1 = Some(Box::new(bloque()));
    coincide(TokenType::UNTIL);
    coincide(TokenType::LPAREN);
    t.hijo2 = Some(Box::new(b_expresion()));
    coincide(TokenType::RPAREN);
    coincide(TokenType::SEMI);
    return t;
}

// sent-read ::= read identificador ";"
fn sent_read() -> TreeNode {
    //unsafe { println!("ENTRE SENT_READ --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newStmtNode(StmtKind::READ);
    coincide(TokenType::READ);
    let mut p: TreeNode = newExpNode();
    coincide(TokenType::ID);
    t.hijo1 = Some(Box::new(p.clone()));
    coincide(TokenType::SEMI);
    return t;
}

// sent-write ::= write b-expresión ";"
fn sent_write() -> TreeNode {
    //unsafe { println!("ENTRE SENT_WRITE --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newStmtNode(StmtKind::WRITE);
    coincide(TokenType::WRITE);
    t.hijo1 = Some(Box::new(b_expresion()));
    coincide(TokenType::SEMI);
    return t;
}

// bloque ::= ”{” lista-sentencia ”}”
fn bloque() -> TreeNode {
    //unsafe { println!("ENTRE BLOQUE --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    coincide(TokenType::LBRACKET);
    unsafe {
        if token_array[token_actual].token != TokenType::RBRACKET {
            t = lista_sentencia();
        } else {
            t = newEmptyNode();
        }
    }
    coincide(TokenType::RBRACKET);
    return t;
}

// asignacion ::= identificador "=" b-expresion ";"
fn asignacion() -> TreeNode {
    //unsafe { println!("ENTRE ASIGNACION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newStmtNode(StmtKind::ASSIGN);
    unsafe {
        if token_array[token_actual].token == TokenType::ID {
            coincide(TokenType::ID);
            coincide(TokenType::ASSIGN);
            t.hijo1 = Some(Box::new(b_expresion()));
            coincide(TokenType::SEMI);
        }
    }
    return t;
}

// b-expresión ::= b-term { OR b-term }
fn b_expresion() -> TreeNode {
    //unsafe { println!("ENTRE B_EXPRESION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = b_term();
    unsafe {
        while token_array[token_actual].token == TokenType::OR {
            let mut p: TreeNode = newExpNode();
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(TokenType::OR);
            t.hijo2 = Some(Box::new(b_term()));
        }
    }
    return t;
}

// b-term ::= not-factor { AND not-factor }
fn b_term() -> TreeNode {
    //unsafe { println!("ENTRE B_TERM --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = not_factor();
    unsafe {
        while token_array[token_actual].token == TokenType::AND {
            let mut p: TreeNode = newExpNode();
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(TokenType::AND);
            t.hijo2 = Some(Box::new(not_factor()));
        }
    }
    return t;
}

// not-factor ::= [NOT] b-factor
fn not_factor() -> TreeNode {
    //unsafe { println!("ENTRE NOT_FACTOR --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    unsafe {
        if token_array[token_actual].token == TokenType::NOT {
            t = newExpNode();
            coincide(TokenType::NOT);
            t.hijo1 = Some(Box::new(b_factor()));
        } else {
            t = b_factor();
        }
    }
    return t;
}

// b-factor ::= true | false | relación
fn b_factor() -> TreeNode {
    //unsafe { println!("ENTRE B_FACTOR --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    unsafe {
        match token_array[token_actual].token {
            TokenType::TRUE | TokenType::FALSE => {
                t = newExpNode();
                t.dtype = ExpType::BOOL;
                coincide(token_array[token_actual].token);
            },
            _ => { t = relacion(); }
        }
    }
    return t;
}

// relacion ::= expresion [ relOp expresion ]
// relOp ::= <= | < | > | >= | == | !=
fn relacion() -> TreeNode {
    //unsafe { println!("ENTRE RELACION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = expresion();
    unsafe {
        if token_array[token_actual].token == TokenType::LT || token_array[token_actual].token == TokenType::LTE  ||
           token_array[token_actual].token == TokenType::GT || token_array[token_actual].token == TokenType::GTE  ||
           token_array[token_actual].token == TokenType::EQ || token_array[token_actual].token == TokenType::DIFF {
            let mut p: TreeNode = newExpNode();
            p.dtype = ExpType::BOOL;
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(token_array[token_actual].token);
            t.hijo2 = Some(Box::new(expresion()));
           }
    }
    return t;
}

// expresion -> termino { sumaOp termino }
// sumaOp -> + | -
fn expresion() -> TreeNode {
    //unsafe { println!("ENTRE EXPRESION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = termino();
    unsafe {
        while token_array[token_actual].token == TokenType::PLUS || token_array[token_actual].token == TokenType::MINUS {
            let mut p: TreeNode = newExpNode();
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(token_array[token_actual].token);
            t.hijo2 = Some(Box::new(termino()));
        }
    }
    return t;
}

// termino -> signoFactor { multOp signoFactor }
// multOp -> * | /
fn termino() -> TreeNode {
    //unsafe { println!("ENTRE TERMINO --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = signoFactor();
    unsafe {
        while token_array[token_actual].token == TokenType::TIMES || token_array[token_actual].token == TokenType::DIVISION {
            let mut p: TreeNode = newExpNode();
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(token_array[token_actual].token);
            t.hijo2 = Some(Box::new(signoFactor()));
        }
    }
    return t;
}

// signoFactor -> [sumaOp] factor
fn signoFactor() -> TreeNode {
    //unsafe { println!("ENTRE SIGNO_FACTOR --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    unsafe {
        if token_array[token_actual].token == TokenType::PLUS || token_array[token_actual].token == TokenType::MINUS {
            t = newExpNode();
            coincide(token_array[token_actual].token);
            t.hijo1 = Some(Box::new(factor()));
        } else {
            t = factor();
        }
    }
    return t;
}

// EL ORIGEN, AQUI SE DETIENE LA LOCURA
// factor -> "(" b-expresion ")" | numero | identificador
fn factor() -> TreeNode {
    //unsafe { println!("ENTRE FACTOR --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    unsafe {
        match token_array[token_actual].token {
            TokenType::NUMINT => {
                t = newExpNode();
                t.dtype = ExpType::INT;
                coincide(TokenType::NUMINT);
            },
            TokenType::NUMFLOAT => {
                t = newExpNode();
                t.dtype = ExpType::FLOAT;
                coincide(TokenType::NUMFLOAT);
            },
            TokenType::ID => {
                t = newExpNode();
                coincide(TokenType::ID);
            },
            TokenType::LPAREN => {
                coincide(TokenType::LPAREN);
                t = b_expresion();
                coincide(TokenType::RPAREN);
            },
            _ => {
                t = newErrorNode();
                error(TokenType::CONST_ID_LPAREN);
            }
        }
    }
    return t;
}

// Nuevo nodo de expresion, estos sirven para los operadores, identificadores y numeros
fn newExpNode() -> TreeNode {

    let mut t: TreeNode = TreeNode {
        hijo1: None,
        hijo2: None,
        hijo3: None,
        hermano: None,
        valor: unsafe { token_array[token_actual].lexema.clone() },
        token: unsafe { token_array[token_actual].token },
        dtype: ExpType::VOID,
        lineano: unsafe { token_array[token_actual].linea },
        tipo_nodo: NodeKind::EXP,
        kind_stmt: StmtKind::UNDEFINED
    };
    return t;
}

// Nuevo nodo del tipo statement, este se aplica a los siguientes:
// asignacion, bloque, sent-write, sent-read, repeticion, iteracion, seleccion
fn newStmtNode(kind: StmtKind) -> TreeNode {

    let mut t: TreeNode = TreeNode {
        hijo1: None,
        hijo2: None,
        hijo3: None,
        hermano: None,
        valor: unsafe { token_array[token_actual].lexema.clone() },
        token: unsafe { token_array[token_actual].token },
        dtype: ExpType::VOID,
        lineano: unsafe { token_array[token_actual].linea },
        tipo_nodo: NodeKind::STMT,
        kind_stmt: kind
    };
    return t;

}

// Funcion que crea un nodo de error (un nodo vacío, ya que no existe el null en RUST)
fn newErrorNode() -> TreeNode {

    let mut t: TreeNode = TreeNode {
        hijo1: None,
        hijo2: None,
        hijo3: None,
        hermano: None,
        valor: unsafe { token_array[token_actual].lexema.clone() },
        token: TokenType::ERROR,
        dtype: ExpType::VOID,
        lineano: unsafe { token_array[token_actual].linea },
        tipo_nodo: NodeKind::EMPTY,
        kind_stmt: StmtKind::UNDEFINED
    };
    return t;

}

// Nos devuelve un arbol vacio, para cuando no ponemos nada en el bloque (como cuando)
// dejamos vacio un else
fn newEmptyNode() -> TreeNode {

    let mut t: TreeNode = TreeNode {
        hijo1: None,
        hijo2: None,
        hijo3: None,
        hermano: None,
        valor: String::from(""),
        token: TokenType::ERROR,
        dtype: ExpType::VOID,
        lineano: 0,
        tipo_nodo: NodeKind::EMPTY,
        kind_stmt: StmtKind::UNDEFINED
    };
    return t;

}

// Si es el token correcto, entonces avanzamos en la lectura de tokens
fn coincide(expected: TokenType) {
    unsafe {
        if (token_array[token_actual].token == expected) {
            saltar();
        } else {
            error(expected);
        }
    }
}

// Funcion que salta al siguiente token, se salta el token si es un comentario (ya que eso no nos importa)
fn saltar() {
    unsafe {
        if token_actual < token_array.len() - 1 { token_actual += 1; }
        if token_array[token_actual].token == TokenType::LINECOMM || token_array[token_actual].token == TokenType::BLOCKCOMM {
            saltar();
        }
    }
}

// Mostramos al usuario cual fue el error
fn error(expected: TokenType) {
    unsafe {
        println!(
            "TOKEN INESPERADO => Lexema: {} | Linea: {} | Token: {:?} --- TOKEN ESPERADO: {:?}", 
            token_array[token_actual].lexema,
            token_array[token_actual].linea,
            token_array[token_actual].token,
            expected
        );
    }
}

// Funcion que imprime el Arbol Sintáctico
// NOTA MUY IMPORTANTE: pinta bien el arbol, solo que no supe como corregir las ramitas que a veces se siguen imprimiendo debajo de un hijo
fn imprimir_arbol(nodo: TreeNode, identacion: i32) { // Esta funcion es recursiva y la identacion va aumentando dependiendo de en que hijo vaya
    unsafe {
        let mut token_string = String::from(""); // Cadena vacia
        for i in 0..identacion { 
            if i == identacion - 2 { token_string = token_string + "├"; } // Si la posicion es justo 2 posiciones menor a la identacion
            else {  // Si debe de ir un espacio o una ramita
                if i % 2 == 0 { token_string = token_string + "│";  } 
                else { token_string = token_string + " "; }
            }
        }
        // En esta seccion se agrega el texto corresponiente (el token que es)
        match nodo.tipo_nodo {
            NodeKind::EXP => {
                token_string = format!("{}{}", token_string, nodo.valor);
            },
            NodeKind::STMT => {
                match nodo.kind_stmt {
                    StmtKind::ASSIGN    |
                    StmtKind::DO        | 
                    StmtKind::IF        | 
                    StmtKind::PROGRAM   | 
                    StmtKind::READ      | 
                    StmtKind::WHILE     | 
                    StmtKind::WRITE   => { token_string = format!("{}{}", token_string, nodo.valor); }
                    StmtKind::DECLARE => { token_string = format!("{}{} {:?}", token_string, nodo.valor, nodo.dtype); },
                    _ => {}
                }
            },
            NodeKind::EMPTY => {}
        }
        // Se imprime el token con su identacion y lineas y todo
        println!("{}", token_string);
        // Primero se manda a llamar al hijo1, luego el 2, 3 y por ultimo el hermano, observese que se aumenta la identacion
        // Solo se manda a llamar si existe el hijo o hermano
        match nodo.hijo1 {
            Some(hijo_1)  => { imprimir_arbol(*hijo_1, identacion + 2); },
            None => {}
        }
        match nodo.hijo2 {
            Some(hijo_2)  => { imprimir_arbol(*hijo_2, identacion + 2); },
            None => {}
        }
        match nodo.hijo3 {
            Some(hijo_3)  => { imprimir_arbol(*hijo_3, identacion + 2); },
            None => {}
        }
        match nodo.hermano {
            Some(brother) => { imprimir_arbol(*brother, identacion); },
            None => {}
        }
    }
}

/* FUNCIONES DEL ANALISIS SEMANTICO, AQUI VAMOS A RECORRER EL ARBOL GENERADO, VAMOS A VALIDAR LOS TIPOS
REDUCIR UN POCO EL ARBOL HACIENDO LAS OPERACIONES Y MOSTRAR LOS ERRORES QUE PUEDA LLEGAR A TENER EL USUARIO */

fn evalType(nodo: &mut TreeNode, tabla_simbolos: &mut TablaDeSimbolos) {
    match nodo.tipo_nodo {
        NodeKind::EXP => {
            /* En caso de que sea una expresion */
            println!("NodeKind del tipo EXP");
            match nodo.token {
                TokenType::ID => {
                    
                },
                __ => {}
            }
        },
        NodeKind::STMT => {
            /* En caso de que sea un statement */
            println!("NodeKind del tipo STMT: {:?}", nodo.kind_stmt);
            match nodo.kind_stmt {
                /* Esta es la seccion de declaraciones */
                StmtKind::DECLARE => {
                    match nodo.dtype {
                        ExpType::INT | ExpType::FLOAT | ExpType::BOOL => {
                            if nodo.hijo1.is_some() { nodo.hijo1.as_deref_mut().unwrap().dtype = nodo.dtype; }
                            evalDecl(&mut nodo.hijo1.as_deref_mut().unwrap(), tabla_simbolos);
                            if nodo.hermano.is_some() { evalType(&mut nodo.hermano.as_deref_mut().unwrap(), tabla_simbolos); }
                        }
                        _ => { println!("Es del tipo void o ninguno"); }
                    }
                },
                StmtKind::PROGRAM => {
                    /* Evaluamos las declaraciones */
                    if nodo.hijo1.is_some() { 
                        evalType(&mut nodo.hijo1.as_deref_mut().unwrap(), tabla_simbolos); }
                    /* Evaluamos el programa en si */
                    if nodo.hijo2.is_some() { 
                        evalType(&mut nodo.hijo2.as_deref_mut().unwrap(), tabla_simbolos); }
                },
                StmtKind::ASSIGN => {
                    if nodo.hijo1.is_some() {

                        /* Evaluamos la asignacion */
                        evalAssg(&mut nodo.hijo1.as_deref_mut().unwrap(), tabla_simbolos);
                        
                        /* Obtenemos el id si es que existe */
                        let mut temp = tabla_simbolos.get(nodo.valor.clone());
                        if temp.token == TokenType::ERROR { process::exit(0x0100); }
                        nodo.dtype = temp.dtype;

                        /* Comprobamos que no hay conflictos con los tipos de valores */

                    }
                },
                _ => {
                    println!("Default para el switch kind_stmt");
                }
            }
        },
        _ => {
            /* Aun no se como se deberia de llegar aqui */
            println!("NodeKind del tipo NINGUNO");
        }
    }
}

/* Evaluar las declaraciones */
fn evalDecl(nodo: &mut TreeNode, tabla_simbolos: &mut TablaDeSimbolos) {

    /* Añadimos la variable a la tabla de simbolos */
    let mut nuevo_simbolo = Simbolo {
        variable: nodo.valor.clone(),
        lineano: nodo.lineano,
        token: TokenType::ID,
        dtype: nodo.dtype,
        valor: nodo.valor.clone()
    };
    println!("Agregando nuevi simbolo: {:?}", nuevo_simbolo);
    tabla_simbolos.insertar(nodo.valor.clone(), nuevo_simbolo);

    /* Si tiene hermanos en la declaracion, pues tambien les daremos el mismo tipo de dato */
    if nodo.hermano.is_some() {
        nodo.hermano.as_deref_mut().unwrap().dtype = nodo.dtype;
        evalDecl(&mut nodo.hermano.as_deref_mut().unwrap(), tabla_simbolos);
    }
}

/* Evalua la asignacion, asegurandose de que el tipo final de estas expresiones sean
del mismo tipo de dato, ademas hace las operaciones necesarias */
fn evalAssg(nodo: &mut TreeNode, tabla_simbolos: &mut TablaDeSimbolos) {

    match nodo.token {
        /* Si es cualquier operador, tienen que existir dos hijos */
        TokenType::PLUS | TokenType::MINUS | TokenType::TIMES | TokenType::DIVISION => {

            /* Si tiene hijos */
            if nodo.hijo1.is_some() && nodo.hijo2.is_some() {

                /* Evaluamos a los hijos */
                evalAssg(&mut nodo.hijo1.as_deref_mut().unwrap(), tabla_simbolos);
                evalAssg(&mut nodo.hijo2.as_deref_mut().unwrap(), tabla_simbolos);

                /* Preguntamos si los dos dTypes son iguales o entra en conflicto */
                if nodo.hijo1.as_deref().unwrap().dtype != nodo.hijo1.as_deref().unwrap().dtype {
                    error_semantico(nodo.clone(), String::from("los tipos de datos no coinciden evalAssg()"));
                }

            }

        },
        _ => {}
    }

}

/* Imprimir los errores del analisis semantico */
fn error_semantico(nodo: TreeNode, motivo: String) {
    println!("Hay un error en el nodo: \n{:?}\nMotivo: {}", nodo, motivo);
    process::exit(0x0100);
}
