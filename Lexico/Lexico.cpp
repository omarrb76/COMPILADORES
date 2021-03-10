/* OMAR ARTURO RUIZ BERNAL ISC 8A 09/03/2021 */
/* SCANNER PARA TINYEXTENDED */

#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <fstream>
#include <ctype.h>

using namespace std;

enum TokenType {
    /* PALBRAS RESERVADAS */
    PROGRAM,    // 0
    IF,         // 1
    ELSE,       // 2
    FI,         // 3
    DO,         // 4
    UNTIL,      // 5
    WHILE,      // 6
    READ,       // 7
    WRITE,      // 8
    FLOAT,      // 9
    INT,        // 10
    BOOL,       // 11
    NOT,        // 12
    AND,        // 13
    OR,         // 14
    /* SIMBOLOS ESPECIALES */
    PLUS,       // 15
    MINUS,      // 16
    TIMES,      // 17
    DIVISION,   // 18
    POW,        // 19
    LT,         // 20
    LTE,        // 21
    GT,         // 22
    GTE,        // 23
    EQ,         // 24
    DIFF,       // 25
    ASSIGN,     // 26
    SEMI,       // 27
    COMA,       // 28
    LPAREN,     // 29
    RPAREN,     // 30
    LBRACKET,   // 31
    RBRACKET,   // 32
    /* TOKENS DE VARIOS CARACTERES */
    ID,         // 33
    NUM,        // 34
    /* COMENTARIOS */
    BLOCKCOMM,  // 35
    LINECOMM,   // 36
    /* BOOK KEEPING TOKENS */
    ENDFILE,    // 37
    ERROR       // 38
};

enum StateType {
    START,      // 0
    INASSIGN,   // 1
    INCOMMENT,  // 2
    INNUM,      // 3
    INID,       // 4
    INLTE,      // 5
    INEQ,       // 6
    INGTE,      // 7
    INDIFF,     // 8
    DONE        // 9
};

// FUNCIONES
TokenType getToken(string lexeme, int lineano);

int main(int argc, char * argv[]) {

    // Abrimos y leemos el archivo
    string linea;               // linea que extreamos del archivo
    ifstream archivo(argv[1]);
    int lineano = 1;            // Linea actual
    char c;                     // Para recorrer la linea
    string palabra = "";        // Palabra que separamos en la linea
    bool comLinea = false;      // Para saber si estamos tomando en cuenta los lexemas o no
    bool comBloque = false;

    // Leemos el archivo
    while (getline(archivo, linea)) {

        for (int i = 0; i < linea.size(); i++) {

            c = linea[i];

            if (comLinea || comBloque) { // Si esta comentado, no tiene sentido estar haciendo los tokens

                if (c == '*' && linea[i + 1] == '/') {
                    comBloque = false;
                    i++;
                }

            } else { // Solo checaremos arriba si existe una forma de quitar el comBloque

                if (c != ' ') { // Sigue siendo la misma palabra o algun símbolo especial

                    if (c == '+' || c == '-' || c == '*' || c == '^' || c == ';' ||
                        c == ',' || c == '(' || c == ')' || c == '{' || c == '}') {
                        if (palabra != "") getToken(palabra, lineano);
                        getToken(string(1, c), lineano);
                        palabra = "";
                    } else if (c == '/') {
                        if (linea[i + 1] == '/') {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken("//", lineano);
                            comLinea = true;
                            palabra = "";
                            i++;
                        } else if (linea[i + 1] == '*') {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken("/*", lineano);
                            comBloque = true;
                            palabra = "";
                            i++;
                        } else {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken("/", lineano);
                            palabra = "";
                        }
                    } else if (c == '<') {
                        if (linea[i + 1] == '=') {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken("<=", lineano);
                            palabra = "";
                            i++;
                        } else {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken("<", lineano);
                            palabra = "";
                        }
                    } else if (c == '>') {
                        if (linea[i + 1] == '=') {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken(">=", lineano);
                            palabra = "";
                            i++;
                        } else {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken(">", lineano);
                            palabra = "";
                        }
                    } else if (c == '=') {
                        if (linea[i + 1] == '=') {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken("==", lineano);
                            palabra = "";
                            i++;
                        } else {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken("=", lineano);
                            palabra = "";
                        }
                    } else if (c == '!') {
                        if (linea[i + 1] == '=') {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken("!=", lineano);
                            palabra = "";
                            i++;
                        } else {
                            if (palabra != "") getToken(palabra, lineano);
                            getToken("!", lineano);
                            palabra = "";
                        }
                    } else {
                        palabra += c;
                    }
                    
                } else { // Word boundary
                    if (palabra != "") getToken(palabra, lineano);
                    palabra = "";
                }
            }

        }

        if (palabra != "") getToken(palabra, lineano);
        palabra = "";

        comLinea = false; // Debido al salto de linea, el comentario de linea se acaba
        lineano++; // Sumamos en uno el numero de línea
    }

    archivo.close();

    system("pause");
    return 0;
}

TokenType getToken(string lexeme, int lineano) {

    TokenType token;
    StateType state = START;
    char c;

    if (lexeme == "program")       token = PROGRAM;
    else if (lexeme == "if")       token = IF;
    else if (lexeme == "else")     token = ELSE;
    else if (lexeme == "fi")       token = FI;
    else if (lexeme == "do")       token = DO;
    else if (lexeme == "until")    token = UNTIL;
    else if (lexeme == "while")    token = WHILE;
    else if (lexeme == "read")     token = READ;
    else if (lexeme == "write")    token = WRITE;
    else if (lexeme == "float")    token = FLOAT;
    else if (lexeme == "int")      token = INT;
    else if (lexeme == "bool")     token = BOOL;
    else if (lexeme == "not")      token = NOT;
    else if (lexeme == "and")      token = AND;
    else if (lexeme == "or")       token = OR;
    else {

        for (int i = 0; i <= lexeme.size(); i++) {

            c = (i == lexeme.size()) ? '\0' : lexeme[i];

            switch (state) {
                case START:
                    if (isdigit(c))      state = INNUM;
                    else if (isalpha(c)) state = INID;
                    else if (c == '<')   state = INLTE;
                    else if (c == '=')   state = INEQ;
                    else if (c == '>')   state = INGTE;
                    else if (c == '!')   state = INDIFF;
                    else if (c == '/')   state = INCOMMENT;
                    else {
                        state = DONE;
                        switch (c) {
                            case '+': token = PLUS;     break;
                            case '-': token = MINUS;    break;
                            case '*': token = TIMES;    break;
                            case '^': token = POW;      break;
                            case '(': token = LPAREN;   break;
                            case ')': token = RPAREN;   break;
                            case ';': token = SEMI;     break;
                            case ',': token = COMA;     break;
                            default:  token = ERROR;    break;
                        }
                        break;
                    }
                    break;
                case INNUM:
                    if (!isdigit(c) && (c != '\0')) {
                        state = DONE;
                        token = ERROR;
                    } else token = NUM;
                    break;
                case INID:
                    if (!isalpha(c) && !isdigit(c) && (c != '\0')) {
                        state = DONE;
                        token = ERROR;
                    } else token = ID;
                    break;
                case INLTE:
                    state = DONE;
                    if (c == '=')       token = LTE;
                    else if (c == '\0') token = LT;
                    else                token = ERROR;
                    break;
                case INEQ:
                    state = DONE;
                    if (c == '=')       token = EQ;
                    else if (c == '\0') token = ASSIGN;
                    else                token = ERROR;
                    break;
                case INGTE:
                    state = DONE;
                    if (c == '=')       token = GTE;
                    else if (c == '\0') token = GT;
                    else                token = ERROR;
                    break;
                case INDIFF:
                    state = DONE;
                    if (c == '=')       token = DIFF;
                    else                token = ERROR;
                    break;
                case INCOMMENT:
                    state = DONE;
                    if (c == '/')       token = LINECOMM;
                    else if (c == '*')  token = BLOCKCOMM;
                    else if (c == '\0') token = DIVISION;   
                    else                token = ERROR;
                    break;
                default: // Nunca debería de pasar
                    cout << "Scanner bug, estado: " << state << endl;
                    state = DONE;
                    token = ERROR;
                    break;
            }

            // Si ya acabó, nos salimos
            if (state == DONE) break;

        }
    }

    cout << "Lexema: " << lexeme << " | Linea: " << lineano << " | Token: " << token << endl;

    return token;
}
