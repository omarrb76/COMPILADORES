use std::env;                                   // Para usar los argumentos del main
use std::process;                               // Para el mensaje de error
use std::fs::File;                              // Para leer el archivo
use std::io::{self, prelude::*, BufReader};     // Para leer el archivo

#[derive(Debug)]
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
    let mut comLinea : bool = false;        // Para saber si tomamos en cuenta los lexemas o no
    let mut comBloque : bool = false;       // Son banderas para saber cuando inicia y acaba un comentario
    let mut comFlagClose : bool = false;
    let mut comFlagOpen : bool = false;
    let mut specialSimbol : bool = false;   // Porque RUST no tiene muchas cosas utiles, necesitamos varias banderas
    let mut specialSimbolChar : char = 'a';

    for linea in reader.lines() {           // Leemos linea por linea

        for c in linea?.chars() {           // Caracter por caracter de cada linea
            
            // Si esta comentado, no tiene sentido estar haciendo los tokens
            if comLinea || comBloque {

                if c == '*' { comFlagClose = true; }// Si el siguiente caracter es '/' se quita el flag
                else if c == '/' && comFlagClose { comBloque = false; comFlagClose = false; }
                continue;

            }

            if comFlagOpen { // Teóricamente el caracter anterior fue un '/' y se vació la palabra

                if c == '/' {
                    getToken(&String::from("//"), lineano);
                    comLinea = true;
                } else if c == '*' {
                    getToken(&String::from("/*"), lineano);
                    comBloque = true;
                } else { getToken(&String::from("/"), lineano); }
                comFlagOpen = false;
                continue;

            }

            if specialSimbol { // Teóricamente aqui tendrá uno de estos símbolos: < > ! = y se vació la palabra

                if c == '=' { getToken(&String::from(specialSimbolChar.to_string() + "="), lineano); }
                else { getToken(&specialSimbolChar.to_string(), lineano); }
                specialSimbol = false;
                continue;

            }

            if c != ' ' {
                if  c == '+' || c == '-' || c == '*' || c == '^' || c == ';' ||
                    c == ',' || c == '(' || c == ')' || c == '{' || c == '}' {

                    if palabra != "" { getToken(&palabra, lineano); }
                    getToken(&c.to_string(), lineano);
                    palabra = String::from("");

                } else if c == '/' {

                    comFlagOpen = true;
                    if palabra != "" { getToken(&palabra, lineano); }
                    palabra = String::from("");

                } else if c == '<' || c == '>' || c == '=' || c == '!' {

                    specialSimbol = true;
                    specialSimbolChar = c;
                    if palabra != "" { getToken(&palabra, lineano); }
                    palabra = String::from("");

                } else {

                    palabra.push(c);

                }
                
            } else { // Word boundary

                if palabra != "" { getToken(&palabra, lineano); }
                palabra = String::from("");

            }
        }

        if palabra != "" { getToken(&palabra, lineano); }
        palabra = String::from("");

        comLinea = false;   // Debido al salto de linea, el comentario de linea se acaba
        lineano += 1;       // Sumamos en uno el numero de línea

    }

    Ok(())

}

fn getToken(lexeme: &String, lineano: i32) -> TokenType {

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

    println!("Lexema: {} | Linea: {} | Token: {:?}", lexeme, lineano, token);

    return token;
}