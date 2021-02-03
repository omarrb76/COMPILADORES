// Esta clase es la interfaz gráfica con el usuario
// Se encargará de manejar los eventos que el usuario este colocando
// Espero no quede mucho código, sino tenemos que ver como
// separar el código en más clases

// Paquete
package compide;

// IMPORTS
import java.awt.Color;
import javax.swing.*;

// CLASE PRINCIPAL
public class InterfazGrafica extends JFrame {
    public InterfazGrafica(){
        super("CompIDE");
        this.setSize(800,500);
        this.setVisible(true);
        this.setDefaultCloseOperation(EXIT_ON_CLOSE); // Probable a cambio para cerrar correctamente
        this.setLocationRelativeTo(null);
        this.iniciarComponentes();
    }
    
    private void iniciarComponentes(){
        JPanel panel = new JPanel();
        panel.setBackground(Color.red);
        this.getContentPane().add(panel);
        
        JLabel etiqueta = new JLabel("Hola");
        panel.add(etiqueta);
    }
}
