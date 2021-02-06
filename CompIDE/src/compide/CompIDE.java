package compide;

import java.util.logging.Level;
import java.util.logging.Logger;
import javax.swing.UIManager;
import javax.swing.UnsupportedLookAndFeelException;

public class CompIDE {
    public static void main(String[] args) {
        // CAMBIAMOS LA APARIENCIA
        try {
            // WINDOWS
            // UIManager.setLookAndFeel("com.sun.java.swing.plaf.windows.WindowsLookAndFeel");
            // NIMBUS
            UIManager.setLookAndFeel("com.sun.java.swing.plaf.nimbus.NimbusLookAndFeel");
        } catch (ClassNotFoundException | InstantiationException | IllegalAccessException | UnsupportedLookAndFeelException ex) {
            Logger.getLogger(CompIDE.class.getName()).log(Level.SEVERE, null, ex);
        }
        
        // EMPIEZA EL PROGRAMA
        InterfazGrafica ide = new InterfazGrafica();
    }
}
