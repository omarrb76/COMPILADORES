use std::env;                                   // Para usar los argumentos del main
use std::process;                               // Para el mensaje de error
use std::fs::File;                              // Para leer el archivo
use std::io::{self, prelude::*, BufReader};     // Para leer el archivo

#[derive(Debug, Copy, Clone)]
enum TokenType {
    /* PALBRAS RESERVADAS */
    PROGRAM, IF, ELSE, FI, DO, UNTIL, WHILE, READ, WRITE, FLOAT, INT, BOOL, NOT, AND, OR,         
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

// Variables globales
// Token actual, lo lleno con lo primero que se me ocurrio
// Al final cambiara su valor
static mut token_actual: Token = Token {
    lexema: String::new(),
    token: TokenType::PROGRAM,
    linea: 0
};

// Arreglo que contendra todos los tokens que vayamos recuperando
static mut token_array: Vec<Token> = Vec::new();

fn main () -> io::Result<()> {

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
            lexema: String::new(),
            token: TokenType::ENDFILE,
            linea: lineano
        });

        // Imprimimos toda la información
        for token in token_array.iter() {
            println!("Lexema: {} | Linea: {} | Token: {:?}", token.lexema, token.linea, token.token);
        }
    }

    Ok(())

}

fn get_token(lexeme: &String, lineano: i32) -> () {

    let mut token : TokenType = TokenType::ERROR;
    let mut state : StateType = StateType::START;

    if      lexeme == "program" { token = TokenType::PROGRAM;   }
    else if lexeme == "if"      { token = TokenType::IF;        }
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