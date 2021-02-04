// Esta clase es la interfaz gráfica con el usuario
// Se encargará de manejar los eventos que el usuario este colocando
// Espero no quede mucho código, sino tenemos que ver como
// separar el código en más clases

// Paquete
package compide;

// IMPORTS
import java.awt.BorderLayout;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.KeyEvent;
import javax.swing.*;

// CLASE PRINCIPAL
public class InterfazGrafica extends JFrame {
    public InterfazGrafica(){
        super("CompIDE");
        this.setSize(800,600);
        this.setIconImage(new ImageIcon(this.getClass().getResource("/res/logo.png")).getImage());
        this.setDefaultCloseOperation(EXIT_ON_CLOSE); // Probable a cambio para cerrar correctamente
        this.setLocationRelativeTo(null);
        this.iniciarComponentes();
    }
    
    private void iniciarComponentes(){
        JMenuBar barraMenu = new JMenuBar();
        
        // AÑADIMOS LA PESTAÑA DE ARCHIVO
        barraMenu.add(this.crearMenuArchivo());
        
        // AÑADIMOS LA PESTAÑA DE EDITAR
        barraMenu.add(this.crearMenuEditar());
        
        JTextArea areaTexto = new JTextArea(5, 5);
        areaTexto.setLineWrap(true);
        areaTexto.setWrapStyleWord(true);
        JScrollPane scroll = new JScrollPane(areaTexto);
        this.getContentPane().add(barraMenu, BorderLayout.NORTH);
        this.getContentPane().add(areaTexto, BorderLayout.CENTER);
        this.setJMenuBar(barraMenu);
        this.setVisible(true);
    }
    
    private JMenu crearMenuArchivo(){
        JMenu archivo = new JMenu("Archivo");
        
        // NUEVO ARCHIVO
        JMenuItem nuevo = new JMenuItem("Nuevo");
        nuevo.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Seleccionaste nuevo");
            }
        });
        nuevo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_N, ActionEvent.CTRL_MASK));
        archivo.add(nuevo);
        
        // ABRIR ARCHIVO
        JMenuItem abrir = new JMenuItem("Abrir");
        abrir.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Elegiste abrir un archivo");
            }
        });
        abrir.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_O, ActionEvent.CTRL_MASK));
        archivo.add(abrir);
        
        // GUARDAR ARCHIVO
        JMenuItem guardar = new JMenuItem("Guardar");
        guardar.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Elegiste guardar el archivo");
            }
        });
        guardar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_S, ActionEvent.CTRL_MASK));
        archivo.add(guardar);
        
        // GUARDAR COMO ARCHIVO
        JMenuItem guardarComo = new JMenuItem("Guardar como");
        guardarComo.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Elegiste guardar como el archivo");
            }
        });
        guardarComo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_S, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK));
        archivo.add(guardarComo);
        
        // CERRAR ARCHIVO
        JMenuItem cerrarArchivo = new JMenuItem("Cerrar archivo");
        cerrarArchivo.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Elegiste cerrar el archivo");
            }
        });
        cerrarArchivo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_B, ActionEvent.CTRL_MASK));
        archivo.add(cerrarArchivo);
        
        archivo.add(new JSeparator()); // Una rayita separadora.
        
        // SALIR
        JMenuItem salir = new JMenuItem("Salir");
        salir.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Elegiste salir");
                System.exit(0);
            }
        });
        salir.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_Q, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK));
        archivo.add(salir);
        
        return archivo;
    }
    
    private JMenu crearMenuEditar() {
        JMenu editar = new JMenu("Editar");
        
        // DESHACER
        JMenuItem deshacer = new JMenuItem("Deshacer");
        deshacer.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Seleccionaste deshacer");
            }
        });
        deshacer.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_Z, ActionEvent.CTRL_MASK));
        editar.add(deshacer);
        
        // REHACER
        JMenuItem rehacer = new JMenuItem("Rehacer");
        rehacer.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Seleccionaste rehacer");
            }
        });
        rehacer.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_Z, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK));
        editar.add(rehacer);
        
        editar.add(new JSeparator()); // Una rayita separadora.
        
        // CORTAR
        JMenuItem cortar = new JMenuItem("Cortar");
        cortar.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Seleccionaste cortar");
            }
        });
        cortar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_X, ActionEvent.CTRL_MASK));
        editar.add(cortar);
        
        // PEGAR
        JMenuItem pegar = new JMenuItem("Pegar");
        pegar.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Seleccionaste pegar");
            }
        });
        pegar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_V, ActionEvent.CTRL_MASK));
        editar.add(pegar);
        
        // COPIAR
        JMenuItem copiar = new JMenuItem("Copiar");
        copiar.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Seleccionaste copiar");
            }
        });
        copiar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_C, ActionEvent.CTRL_MASK));
        editar.add(copiar);
        
        editar.add(new JSeparator()); // Una rayita separadora.
        
        // SELECCIONAR TODO
        JMenuItem seleccionarTodo = new JMenuItem("Seleccionar Todo");
        seleccionarTodo.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                System.out.println("Seleccionaste seleccionar Todo");
            }
        });
        seleccionarTodo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_A, ActionEvent.CTRL_MASK));
        editar.add(seleccionarTodo);
        
        return editar;
    }
}
