// Esta clase es la interfaz gráfica con el usuario
// Se encargará de manejar los eventos que el usuario este colocando
// Espero no quede mucho código, sino tenemos que ver como
// separar el código en más clases

// Paquete
package compide;

// IMPORTS
import java.awt.BorderLayout;
import java.awt.Image;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.KeyEvent;
import java.io.IOException;
import java.util.logging.Level;
import java.util.logging.Logger;
import javax.imageio.ImageIO;
import javax.swing.*;

// CLASE PRINCIPAL
public class InterfazGrafica extends JFrame {
    public InterfazGrafica(){
        super("CompIDE");
        this.setSize(800,600);
        this.setIconImage(new ImageIcon(this.getClass().getResource("/res/img/logo.png")).getImage());
        this.setDefaultCloseOperation(EXIT_ON_CLOSE); // Probable a cambio para cerrar correctamente
        this.setLocationRelativeTo(null);
        this.iniciarComponentes();
    }
    
    private void iniciarComponentes() {
        JMenuBar barraMenu = new JMenuBar();
        
        // AÑADIMOS LA PESTAÑA DE ARCHIVO
        barraMenu.add(this.crearMenuArchivo());
        
        // AÑADIMOS LA PESTAÑA DE EDITAR
        barraMenu.add(this.crearMenuEditar());
        
        // AÑADIMOS LA PESTAÑA DE FORMATO
        barraMenu.add(this.crearMenuFormato());
        
        // AÑADIMOS LA PESTAÑA DE COMPILAR
        barraMenu.add(this.crearMenuCompilar());
        
        // AÑADIMOS LA PESTAÑA DE AYUDA
        barraMenu.add(this.crearMenuAyuda());
        
        JTextArea areaTexto = new JTextArea(5, 5);
        areaTexto.setLineWrap(true);
        areaTexto.setWrapStyleWord(true);
        JScrollPane scroll = new JScrollPane(areaTexto);
        this.getContentPane().add(barraMenu, BorderLayout.NORTH);
        this.getContentPane().add(areaTexto, BorderLayout.CENTER);
        this.setJMenuBar(barraMenu);
        this.setVisible(true);
    }
    
    // LOS DIFERENTES COMPONENTES DE LOS MENUS
    private JMenu crearMenuArchivo() {
        JMenu archivo = new JMenu("Archivo");
        archivo.setMnemonic('A');
        
        // NUEVO ARCHIVO
        JMenuItem nuevo = new JMenuItem("Nuevo");
        nuevo.addActionListener((ActionEvent e) -> {
            System.out.println("Seleccionaste nuevo");
        });
        nuevo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_N, ActionEvent.CTRL_MASK));
        nuevo.setIcon(crearIcono("/res/img/nuevo_archivo.png"));
        archivo.add(nuevo);
        
        // ABRIR ARCHIVO
        JMenuItem abrir = new JMenuItem("Abrir");
        abrir.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste abrir un archivo");
        });
        abrir.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_O, ActionEvent.CTRL_MASK));
        abrir.setIcon(crearIcono("/res/img/abrir_archivo.png"));
        archivo.add(abrir);
        
        // GUARDAR ARCHIVO
        JMenuItem guardar = new JMenuItem("Guardar");
        guardar.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste guardar el archivo");
        });
        guardar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_S, ActionEvent.CTRL_MASK));
        guardar.setIcon(crearIcono("/res/img/guardar.png"));
        archivo.add(guardar);
        
        // GUARDAR COMO ARCHIVO
        JMenuItem guardarComo = new JMenuItem("Guardar como");
        guardarComo.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste guardar como el archivo");
        });
        guardarComo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_S, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK));
        guardarComo.setIcon(crearIcono("/res/img/guardar.png"));
        archivo.add(guardarComo);
        
        // CERRAR ARCHIVO
        JMenuItem cerrarArchivo = new JMenuItem("Cerrar archivo");
        cerrarArchivo.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste cerrar el archivo");
        });
        cerrarArchivo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_B, ActionEvent.CTRL_MASK));
        cerrarArchivo.setIcon(crearIcono("/res/img/cerrar_archivo.png"));
        archivo.add(cerrarArchivo);
        
        archivo.add(new JSeparator()); // Una rayita separadora.
        
        // SALIR
        JMenuItem salir = new JMenuItem("Salir");
        salir.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste salir");
            System.exit(0);
        });
        salir.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_Q, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK));
        salir.setIcon(crearIcono("/res/img/salir.png"));
        archivo.add(salir);
        
        return archivo;
    }
    
    private JMenu crearMenuEditar() {
        JMenu editar = new JMenu("Editar");
        editar.setMnemonic('E');
        
        // DESHACER
        JMenuItem deshacer = new JMenuItem("Deshacer");
        deshacer.addActionListener((ActionEvent e) -> {
            System.out.println("Seleccionaste deshacer");
        });
        deshacer.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_Z, ActionEvent.CTRL_MASK));
        deshacer.setIcon(crearIcono("/res/img/deshacer.png"));
        editar.add(deshacer);
        
        // REHACER
        JMenuItem rehacer = new JMenuItem("Rehacer");
        rehacer.addActionListener((ActionEvent e) -> {
            System.out.println("Seleccionaste rehacer");
        });
        rehacer.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_Z, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK));
        rehacer.setIcon(crearIcono("/res/img/rehacer.png"));
        editar.add(rehacer);
        
        editar.add(new JSeparator()); // Una rayita separadora.
        
        // CORTAR
        JMenuItem cortar = new JMenuItem("Cortar");
        cortar.addActionListener((ActionEvent e) -> {
            System.out.println("Seleccionaste cortar");
        });
        cortar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_X, ActionEvent.CTRL_MASK));
        cortar.setIcon(crearIcono("/res/img/cortar.png"));
        editar.add(cortar);
        
        // PEGAR
        JMenuItem pegar = new JMenuItem("Pegar");
        pegar.addActionListener((ActionEvent e) -> {
            System.out.println("Seleccionaste pegar");
        });
        pegar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_V, ActionEvent.CTRL_MASK));
        pegar.setIcon(crearIcono("/res/img/pegar.png"));
        editar.add(pegar);
        
        // COPIAR
        JMenuItem copiar = new JMenuItem("Copiar");
        copiar.addActionListener((ActionEvent e) -> {
            System.out.println("Seleccionaste copiar");
        });
        copiar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_C, ActionEvent.CTRL_MASK));
        copiar.setIcon(crearIcono("/res/img/copiar.png"));
        editar.add(copiar);
        
        editar.add(new JSeparator()); // Una rayita separadora.
        
        // SELECCIONAR TODO
        JMenuItem seleccionarTodo = new JMenuItem("Seleccionar Todo");
        seleccionarTodo.addActionListener((ActionEvent e) -> {
            System.out.println("Seleccionaste seleccionar Todo");
        });
        seleccionarTodo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_A, ActionEvent.CTRL_MASK));
        seleccionarTodo.setIcon(crearIcono("/res/img/seleccionar.png"));
        editar.add(seleccionarTodo);
        
        return editar;
    }
    
    private JMenu crearMenuFormato(){
        JMenu formato = new JMenu("Formato");
        formato.setMnemonic('F');
        
        JMenuItem opcion1 = new JMenuItem("Opcion 1");
        opcion1.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste la opcion 1");
        });
        opcion1.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_L, ActionEvent.CTRL_MASK));
        formato.add(opcion1);
        
        return formato;
    }
    
    private JMenu crearMenuCompilar(){
        JMenu compilar = new JMenu("Compilar");
        compilar.setMnemonic('C');
        
        JMenuItem compile = new JMenuItem("Compila");
        compile.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste la opcion compila");
        });
        compile.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_C, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK));
        compile.setIcon(crearIcono("/res/img/compilar.png"));
        compilar.add(compile);
        
        return compilar;
    }
    
    private JMenu crearMenuAyuda(){
        JMenu ayuda = new JMenu("Ayuda");
        ayuda.setMnemonic('y');
        
        JMenuItem acercaDe = new JMenuItem("Acerca de");
        acercaDe.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste la opcion acerca de");
        });
        acercaDe.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_H, ActionEvent.CTRL_MASK));
        acercaDe.setIcon(crearIcono("/res/img/ayuda.png"));
        ayuda.add(acercaDe);
        
        return ayuda;
    }
    
    private ImageIcon crearIcono(String archivo){
        ImageIcon i;
        i = new ImageIcon((getClass().getResource(archivo)));
        Image image = i.getImage(); // transform it
        Image newimg = image.getScaledInstance(15, 15, java.awt.Image.SCALE_SMOOTH); // scale it the smooth way 
        i = new ImageIcon(newimg);  // transform it back
        return i;
    }
}
