// Esta clase lee los archivos y escribe sobre ellos
// tiene métodos para obtener lo que dice un archivo y para escribir sobre el
// Los pasos para leer o guardar un archivo son 
// PRIMERO: seteo el archivo
// SEGUNDO: leo o guardo
// TERCERO: en caso de leer, consigo la informacion con getTexto()
package compide;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.PrintWriter;
import java.util.logging.Level;
import java.util.logging.Logger;

public class ManipuladorArchivos {
    
    // ATRIBUTOS
    private String contenido;
    private File archivo;
    
    // CONSTRUCTOR
    public ManipuladorArchivos() {
        contenido = "";
        archivo = null;
    }
    
    // SETTERS AND GETTERS
    public void setTexto(String nuevo) {
        contenido = nuevo;
    }
    
    public String getTexto() {
        return contenido;
    }
    
    public void setArchivo(File nuevo) {
        archivo = nuevo;
    }
    
    public File getArchivo() {
        return archivo;
    }
    
    // METODOS
    public void leerTexto() {
        contenido = "";
        String linea;
        Boolean primeraLinea = true;
        try {
            FileReader file = new FileReader(archivo);
            BufferedReader buffer = new BufferedReader(file);
            while ((linea = buffer.readLine()) != null) {
                if (primeraLinea) {
                    contenido = linea;
                    primeraLinea = false;
                } else {
                    contenido += "\n" + linea;
                }
            }
            file.close();
            buffer.close();
        } catch (FileNotFoundException e){
            Logger.getLogger(ManipuladorArchivos.class.getName()).log(Level.SEVERE, null, e);
        } catch (IOException ex) {
            Logger.getLogger(ManipuladorArchivos.class.getName()).log(Level.SEVERE, null, ex);
        }
    }
    
    public Boolean escribirTexto(String nuevo){
        Boolean resultado = false; // Fracaso
        try {
            FileWriter fw = new FileWriter(archivo);
            PrintWriter pw = new PrintWriter(fw);
            pw.print(nuevo);
            fw.close();
            pw.close();
        } catch (IOException e) {
            Logger.getLogger(ManipuladorArchivos.class.getName()).log(Level.SEVERE, null, e);
        }
        return resultado;
    }
}
