use std::env;                                   // Para usar los argumentos del main
use std::process;                               // Para el mensaje de error
use std::fs::File;                              // Para leer el archivo
use std::io::{self, prelude::*, BufReader};     // Para leer el archivo

// ENUMS Y STRUCTS PERTENECIENTES AL ANÁLISIS LÉXICO
#[derive(Debug, Copy, Clone, PartialEq)]
enum TokenType {
    /* PALBRAS RESERVADAS */
    PROGRAM, IF, ELSE, FI, DO, UNTIL, WHILE, READ, WRITE, FLOAT, INT, BOOL, NOT, AND, OR, TRUE, FALSE, THEN,
    /* SIMBOLOS ESPECIALES */
    PLUS, MINUS, TIMES, DIVISION, POW, LT, LTE, GT, GTE, EQ, DIFF, ASSIGN, SEMI, COMA, LPAREN, RPAREN, LBRACKET, RBRACKET,   
    /* TOKENS DE VARIOS CARACTERES */
    ID, NUM,        
    /* COMENTARIOS */
    BLOCKCOMM, LINECOMM,   
    /* BOOK KEEPING TOKENS */
    ENDFILE, ERROR       
}

#[derive(Debug)]
enum StateType {
    START, INASSIGN, INCOMMENT, INNUM, INID, INLTE, INEQ, INGTE, INDIFF, DONE
}

// Estructura que contiene la información de cada Token
#[derive(Clone)]
struct Token {
    lexema: String,
    token: TokenType,
    linea: i32
}

/************ PERTENECIENTE AL ANÁLISIS SINTÁCTICO ***************/
#[derive(Copy, Clone)]
enum NodeKind { STMT, EXP }

#[derive(Copy, Clone)]
enum StmtKind { IF, REPEAT,  ASSIGN, READ, WRITE }

#[derive(Copy, Clone)]
enum ExpKind  { OP, CONST, ID }

#[derive(Copy, Clone)]
enum ExpType  { VOID, INT, BOOL }

#[derive(Clone)]
struct TreeNode {
    hijo1: Option<Box<TreeNode>>,
    hijo2: Option<Box<TreeNode>>,
    hijo3: Option<Box<TreeNode>>,
    hermano: Option<Box<TreeNode>>,
    lineano: i32,
    tipo_nodo: NodeKind,
    kind_stmt: Option<StmtKind>,
    kind_exp: Option<ExpKind>,
    attr_op: Option<TokenType>,
    attr_val: Option<i32>,
    attr_name: Option<String>,
    exp_type: ExpType
}

// Variables globales
// Posicion en la que nos encontramos
static mut token_actual: usize = 0;

// Arreglo que contendra todos los tokens que vayamos recuperando
static mut token_array: Vec<Token> = Vec::new();

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
    let mut lineano = 1;                    // Para contar las líneas
    let mut palabra = String::from("");     // Palabra que separamos en la línea
    let mut com_linea : bool = false;        // Para saber si tomamos en cuenta los lexemas o no
    let mut com_bloque : bool = false;       // Son banderas para saber cuando inicia y acaba un comentario
    let mut com_flag_close : bool = false;
    let mut com_flag_open : bool = false;
    let mut special_simbol : bool = false;   // Porque RUST no tiene muchas cosas utiles, necesitamos varias banderas
    let mut special_simbol_char : char = 'a';

    for linea in reader.lines() {           // Leemos linea por linea

        for c in linea?.chars() {           // Caracter por caracter de cada linea
            
            // Si esta comentado, no tiene sentido estar haciendo los tokens
            if com_linea || com_bloque {

                if c == '*' { com_flag_close = true; }// Si el siguiente caracter es '/' se quita el flag
                else if c == '/' && com_flag_close { com_bloque = false; com_flag_close = false; }
                continue;

            }

            if com_flag_open { // Teóricamente el caracter anterior fue un '/' y se vació la palabra

                if c == '/' {
                    get_token(&String::from("//"), lineano);
                    com_linea = true;
                } else if c == '*' {
                    get_token(&String::from("/*"), lineano);
                    com_bloque = true;
                } else { get_token(&String::from("/"), lineano); }
                com_flag_open = false;
                continue;

            }

            if special_simbol { // Teóricamente aqui tendrá uno de estos símbolos: < > ! = y se vació la palabra

                if c == '=' { get_token(&String::from(special_simbol_char.to_string() + "="), lineano); }
                else { get_token(&special_simbol_char.to_string(), lineano); }
                special_simbol = false;
                continue;

            }

            if c != ' ' {
                if  c == '+' || c == '-' || c == '*' || c == '^' || c == ';' ||
                    c == ',' || c == '(' || c == ')' || c == '{' || c == '}' {

                    if palabra != "" { get_token(&palabra, lineano); }
                    get_token(&c.to_string(), lineano);
                    palabra = String::from("");

                } else if c == '/' {

                    com_flag_open = true;
                    if palabra != "" { get_token(&palabra, lineano); }
                    palabra = String::from("");

                } else if c == '<' || c == '>' || c == '=' || c == '!' {

                    special_simbol = true;
                    special_simbol_char = c;
                    if palabra != "" { get_token(&palabra, lineano); }
                    palabra = String::from("");

                } else {

                    palabra.push(c);

                }
                
            } else { // Word boundary

                if palabra != "" { get_token(&palabra, lineano); }
                palabra = String::from("");

            }
        }

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

    //b_expresion();
    asignacion();

    Ok(())

}

fn get_token(lexeme: &String, lineano: i32) -> () {

    let mut token : TokenType = TokenType::ERROR;
    let mut state : StateType = StateType::START;

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
    else {

        for c in lexeme.chars() {
            
            match state {
                StateType::START => {
                    if      c.is_digit(10)      { state = StateType::INNUM;     token = TokenType::NUM;         }
                    else if c == '_'            { state = StateType::INID;      token = TokenType::ID;          }
                    else if c.is_alphabetic()   { state = StateType::INID;      token = TokenType::ID;          }
                    else if c == '<'            { state = StateType::INLTE;     token = TokenType::LT;          }
                    else if c == '='            { state = StateType::INEQ;      token = TokenType::ASSIGN;      }
                    else if c == '>'            { state = StateType::INGTE;     token = TokenType::GT;          }
                    else if c == '!'            { state = StateType::INDIFF;    token = TokenType::DIFF;        }
                    else if c == '/'            { state = StateType::INCOMMENT; token = TokenType::DIVISION;    }
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
                StateType::INNUM => {
                    if !c.is_digit(10)      { state = StateType::DONE; token = TokenType::ERROR;    } 
                    else                    { token = TokenType::NUM;                               }
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

// Funciones del análisis sintáctico

// lista-sentencias ::= { sentencias }
fn lista_sentencia() -> TreeNode {
    unsafe { println!("ENTRE LISTA_SENTENCIA --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
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
    unsafe { println!("ENTRE SENTENCIA --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
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
                t = newErrorNode(ExpKind::OP);
                error(TokenType::ENDFILE);
                token_actual += 1;
            }
        }
    }
    return t;
}

fn seleccion() -> TreeNode {
    unsafe { println!("ENTRE SENTENCIA --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    coincide(TokenType::IF);
    coincide(TokenType::LPAREN);
    t = b_expresion();
    coincide(TokenType::RPAREN);
    coincide(TokenType::TH)
}

// bloque ::= ”{” lista-sentencia ”}”
fn bloque() -> TreeNode {
    unsafe { println!("ENTRE BLOQUE --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    coincide(TokenType::LBRACKET);
    t = lista_sentencia();
    coincide(TokenType::RBRACKET);
    return t;
}

// asignacion ::= identificador "=" b-expresion ";"
fn asignacion() -> TreeNode {
    unsafe { println!("ENTRE ASIGNACION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = newStmtNode(StmtKind::ASSIGN);
    unsafe {
        if token_array[token_actual].token == TokenType::ID {
            t.attr_name = Some(token_array[token_actual].lexema.clone());
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
    unsafe { println!("ENTRE B_EXPRESION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = b_term();
    unsafe {
        while token_array[token_actual].token == TokenType::OR {
            let mut p: TreeNode = newExpNode(ExpKind::OP);
            p.attr_op = Some(token_array[token_actual].token);
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(TokenType::OR);
            p.hijo2 = Some(Box::new(b_term()));
        }
    }
    return t;
}

// b-term ::= not-factor { AND not-factor }
fn b_term() -> TreeNode {
    unsafe { println!("ENTRE B_TERM --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = not_factor();
    unsafe {
        while token_array[token_actual].token == TokenType::AND {
            let mut p: TreeNode = newExpNode(ExpKind::OP);
            p.attr_op = Some(token_array[token_actual].token);
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(TokenType::AND);
            p.hijo2 = Some(Box::new(not_factor()));
        }
    }
    return t;
}

// not-factor ::= [NOT] b-factor
fn not_factor() -> TreeNode {
    unsafe { println!("ENTRE NOT_FACTOR --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    unsafe {
        if token_array[token_actual].token == TokenType::NOT {
            t = newExpNode(ExpKind::OP);
            t.attr_op = Some(token_array[token_actual].token);
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
    unsafe { println!("ENTRE B_FACTOR --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    unsafe {
        match token_array[token_actual].token {
            TokenType::TRUE | TokenType::FALSE => {
                t = newExpNode(ExpKind::OP);
                t.attr_op = Some(token_array[token_actual].token);
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
    unsafe { println!("ENTRE RELACION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = expresion();
    unsafe {
        if token_array[token_actual].token == TokenType::LT || token_array[token_actual].token == TokenType::LTE  ||
           token_array[token_actual].token == TokenType::GT || token_array[token_actual].token == TokenType::GTE  ||
           token_array[token_actual].token == TokenType::EQ || token_array[token_actual].token == TokenType::DIFF {
            let mut p: TreeNode = newExpNode(ExpKind::OP);
            p.attr_op = Some(token_array[token_actual].token);
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(token_array[token_actual].token);
            p.hijo2 = Some(Box::new(expresion()));
           }
    }
    return t;
}

// expresion -> termino { sumaOp termino }
// sumaOp -> + | -
fn expresion() -> TreeNode {
    unsafe { println!("ENTRE EXPRESION --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = termino();
    unsafe {
        while token_array[token_actual].token == TokenType::PLUS || token_array[token_actual].token == TokenType::MINUS {
            let mut p: TreeNode = newExpNode(ExpKind::OP);
            p.attr_op = Some(token_array[token_actual].token);
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(token_array[token_actual].token);
            p.hijo2 = Some(Box::new(termino()));
        }
    }
    return t;
}

// termino -> signoFactor { multOp signoFactor }
// multOp -> * | /
fn termino() -> TreeNode {
    unsafe { println!("ENTRE TERMINO --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode = signoFactor();
    unsafe {
        while token_array[token_actual].token == TokenType::TIMES || token_array[token_actual].token == TokenType::DIVISION {
            let mut p: TreeNode = newExpNode(ExpKind::OP);
            p.attr_op = Some(token_array[token_actual].token);
            p.hijo1 = Some(Box::new(t.clone()));
            t = p.clone();
            coincide(token_array[token_actual].token);
            p.hijo2 = Some(Box::new(signoFactor()));
        }
    }
    return t;
}

// signoFactor -> [sumaOp] factor
fn signoFactor() -> TreeNode {
    unsafe { println!("ENTRE SIGNO_FACTOR --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    unsafe {
        if token_array[token_actual].token == TokenType::PLUS || token_array[token_actual].token == TokenType::MINUS {
            t = newExpNode(ExpKind::OP);
            t.attr_op = Some(token_array[token_actual].token);
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
    unsafe { println!("ENTRE FACTOR --- TOKEN ACTUAL: {:?}", token_array[token_actual].token); }
    let mut t: TreeNode;
    unsafe {
        match token_array[token_actual].token {
            TokenType::NUM => {
                t = newExpNode(ExpKind::CONST);
                t.attr_val = Some(token_array[token_actual].lexema.parse::<i32>().unwrap());
                coincide(TokenType::NUM);
            },
            TokenType::ID => {
                t = newExpNode(ExpKind::ID);
                t.attr_name = Some(token_array[token_actual].lexema.clone());
                coincide(TokenType::ID);
            },
            TokenType::LPAREN => {
                coincide(TokenType::LPAREN);
                t = b_expresion();
                coincide(TokenType::RPAREN);
            },
            _ => {
                t = newErrorNode(ExpKind::OP);
                error(TokenType::ENDFILE);
                token_actual += 1;
            }
        }
    }
    return t;
}

// Nuevo nodo de expresion, estos sirven para los operadores, identificadores y numeros
fn newExpNode(kind: ExpKind) -> TreeNode {

    let mut t: TreeNode = TreeNode {
        hijo1 : None,
        hijo2 : None,
        hijo3 : None,
        hermano : None,
        attr_op : None,
        kind_stmt: None,
        kind_exp: Some(kind),
        tipo_nodo: NodeKind::EXP,
        attr_val : None,
        attr_name : None,
        lineano : unsafe { token_array[token_actual].linea },
        exp_type : ExpType::VOID
    };
    return t;
}

// Nuevo nodo del tipo statement, este se aplica a los siguientes:
// asignacion, bloque, sent-write, sent-read, repeticion, iteracion, seleccion
fn newStmtNode(kind: StmtKind) -> TreeNode {

    let mut t: TreeNode = TreeNode {
        hijo1 : None,
        hijo2 : None,
        hijo3 : None,
        hermano : None,
        attr_op : None,
        kind_stmt: Some(kind),
        kind_exp: None,
        tipo_nodo: NodeKind::STMT,
        attr_val : None,
        attr_name : None,
        lineano : unsafe { token_array[token_actual].linea },
        exp_type : ExpType::VOID
    };
    return t;
}

// Funcion que crea un nodo de error (un nodo vacío, ya que no existe el null en RUST)
fn newErrorNode(kind: ExpKind) -> TreeNode {

    let mut t: TreeNode = TreeNode {
        hijo1 : None,
        hijo2 : None,
        hijo3 : None,
        hermano : None,
        attr_op : Some(TokenType::ERROR),
        kind_stmt: None,
        kind_exp: Some(kind),
        tipo_nodo: NodeKind::EXP,
        attr_val : None,
        attr_name : None,
        lineano : unsafe { token_array[token_actual].linea },
        exp_type : ExpType::VOID
    };
    return t;
}

// Si es el token correcto, entonces avanzamos en la lectura de tokens
fn coincide(expected: TokenType) {
    unsafe {
        if (token_array[token_actual].token == expected) {
            token_actual += 1;
        } else {
            error(expected);
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