package compide;

import java.util.logging.Level;
import java.util.logging.Logger;
import javax.swing.UIManager;
import javax.swing.UnsupportedLookAndFeelException;

public class CompIDE {
    public static void main(String[] args) {
        try {
            // WINDOWS
            // UIManager.setLookAndFeel("com.sun.java.swing.plaf.windows.WindowsLookAndFeel");
            // NIMBUS
            UIManager.setLookAndFeel("com.sun.java.swing.plaf.nimbus.NimbusLookAndFeel");
        } catch (ClassNotFoundException | InstantiationException | IllegalAccessException | UnsupportedLookAndFeelException ex) {
            Logger.getLogger(CompIDE.class.getName()).log(Level.SEVERE, null, ex);
        }
        
        InterfazGrafica ide = new InterfazGrafica();
        /*ManipuladorArchivos m = new ManipuladorArchivos();
        m.leerTexto("src/res/prueba.txt");
        System.out.println(m.getTexto());
        m.setNombreArchivo("src/res/resultado.txt");
        m.escribirTexto("Soy omar y estoy probando mi programa\nEspero les guste mucho");*/
    }
}
