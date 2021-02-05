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
import java.awt.event.KeyEvent;
import java.io.File;
import javax.swing.*;
import javax.swing.event.DocumentEvent;
import javax.swing.event.DocumentListener;

// CLASE PRINCIPAL
public class InterfazGrafica extends JFrame {
    
    // ATRIBUTOS
    JTextArea areaTexto; // El area donde el usuario escribirá
    ManipuladorArchivos manipuladorArchivos; // Para leer y guardar el archivo
    Boolean editado; // Para saber si esta siendo editado
    String titulo; // Para poner de titulo
    Boolean mismoArchivo; // Para guardar en el mismo archivo (Guardar) o pedir que elija un directorio
    
    public InterfazGrafica(){
        super("CompIDE");
        titulo = "CompIDE";
        mismoArchivo = false;
        manipuladorArchivos = new ManipuladorArchivos();
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
        
        areaTexto = new JTextArea(5, 5);
        areaTexto.setLineWrap(true);
        areaTexto.setWrapStyleWord(true);
        areaTexto.setEnabled(false);
        
        areaTexto.getDocument().addDocumentListener(new DocumentListener() {
            @Override
            public void insertUpdate(DocumentEvent e) {
                if (!editado){
                    titulo += " *";
                    setTitle(titulo);
                }
                editado = true;
            }
            @Override
            public void removeUpdate(DocumentEvent e) {
                if (!editado){
                    titulo += " *";
                    setTitle(titulo);
                }
                editado = true;
            }
            @Override
            public void changedUpdate(DocumentEvent e) {
                if (!editado){
                    titulo += " *";
                    setTitle(titulo);
                }
                editado = true;
            }
        });
        
        
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
            areaTexto.setEnabled(true);
            titulo = "CompIDE - Archivo nuevo *";
            setTitle(titulo);
            editado = true; // Porque acabamos de crear un nuevo archivo
            mismoArchivo = false;
            areaTexto.requestFocus();
        });
        nuevo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_N, ActionEvent.CTRL_MASK));
        nuevo.setIcon(crearIcono("/res/img/nuevo_archivo.png"));
        archivo.add(nuevo);
        
        // ABRIR ARCHIVO
        JMenuItem abrir = new JMenuItem("Abrir");
        abrir.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste abrir un archivo");
            JFileChooser fileChooser = new JFileChooser();
            int seleccion = fileChooser.showOpenDialog(areaTexto);
            if (seleccion == JFileChooser.APPROVE_OPTION) {
               File fichero = fileChooser.getSelectedFile();
               // Aqui la informacion de apertura
               areaTexto.setEnabled(false);
               manipuladorArchivos.setArchivo(fichero);
               manipuladorArchivos.leerTexto();
               areaTexto.setText(manipuladorArchivos.getTexto());
               titulo = "CompIDE - " + fichero.getName();
               this.setTitle(titulo);
               areaTexto.setEnabled(true);
               editado = false; // Lo acabamos de recien abrir, no puede estar editado
               mismoArchivo = true;
            }
        });
        abrir.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_O, ActionEvent.CTRL_MASK));
        abrir.setIcon(crearIcono("/res/img/abrir_archivo.png"));
        archivo.add(abrir);
        
        // GUARDAR ARCHIVO
        JMenuItem guardar = new JMenuItem("Guardar");
        guardar.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste guardar el archivo");
            guardarArchivo(mismoArchivo);
        });
        guardar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_S, ActionEvent.CTRL_MASK));
        guardar.setIcon(crearIcono("/res/img/guardar.png"));
        archivo.add(guardar);
        
        // GUARDAR COMO ARCHIVO
        JMenuItem guardarComo = new JMenuItem("Guardar como");
        guardarComo.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste guardar como el archivo");
            editado = true; // Para que forsozamente nos muestre la ventana de elegir donde guardar
            guardarArchivo(false);
        });
        guardarComo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_S, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK));
        guardarComo.setIcon(crearIcono("/res/img/guardar.png"));
        archivo.add(guardarComo);
        
        // CERRAR ARCHIVO
        JMenuItem cerrarArchivo = new JMenuItem("Cerrar archivo");
        cerrarArchivo.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste cerrar el archivo");
            int result = JOptionPane.showConfirmDialog(this, "¿Desea cerrar el archivo?", "CompIDE",
               JOptionPane.YES_NO_OPTION);
            switch (result) {
                case JOptionPane.YES_OPTION:
                    System.out.println("Elegiste si");
                    break;
                case JOptionPane.NO_OPTION:
                    System.out.println("Elegiste no");
                    break;
                default:
                    System.out.println("Elegiste nada");
                    break;
            }
        });
        cerrarArchivo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_B, ActionEvent.CTRL_MASK));
        cerrarArchivo.setIcon(crearIcono("/res/img/cerrar_archivo.png"));
        archivo.add(cerrarArchivo);
        
        archivo.add(new JSeparator()); // Una rayita separadora.
        
        // SALIR
        JMenuItem salir = new JMenuItem("Salir");
        salir.addActionListener((ActionEvent e) -> {
            System.out.println("Elegiste salir");
            int result = JOptionPane.showConfirmDialog(this, "¿Desea salir de la aplicación?", "CompIDE",
               JOptionPane.YES_NO_OPTION);
            switch (result) {
                case JOptionPane.YES_OPTION:
                    System.out.println("Elegiste si");
                    System.exit(0);
                    break;
                case JOptionPane.NO_OPTION:
                    System.out.println("Elegiste no");
                    break;
                default:
                    System.out.println("Elegiste nada");
                    break;
            }
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
            JOptionPane.showMessageDialog(
                    InterfazGrafica.this, 
                    "CompIDE versión 1.0\n"
                            + "Desarrollado por:\n"
                            + "- Carlos García Gutiérrez\n"
                            + "- Daniela Yael Rodríguez Reyes\n"
                            + "- Omar Artturo Ruiz Bernal\n\n"
                            + "Proyecto final de Compiladores I", 
                    "Acerca de", 
                    JOptionPane.INFORMATION_MESSAGE);
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
    
    // Este método es para los ActionListener de los botones de guardar, para no repetir código
    private void guardarArchivo(Boolean guardarComo){
        if (editado) {
            if (!guardarComo) {
                JFileChooser fileChooser = new JFileChooser();
                int seleccion = fileChooser.showSaveDialog(areaTexto);
                if (seleccion == JFileChooser.APPROVE_OPTION) {
                   File fichero = fileChooser.getSelectedFile();
                   // Aqui la informacion de guardado
                   manipuladorArchivos.setArchivo(fichero);
                   manipuladorArchivos.escribirTexto(areaTexto.getText());
                   editado = false; // Lo acabamos de recien guardar, no puede estar editado
                   titulo = "CompIDE - " + fichero.getName();
                   this.setTitle(titulo);
                }
            }
            manipuladorArchivos.escribirTexto(areaTexto.getText());
            editado = false; // Lo acabamos de recien guardar, no puede estar editado
            titulo = "CompIDE - " + manipuladorArchivos.getArchivo().getName();
            this.setTitle(titulo);
            mismoArchivo = true;
        }
    }
}
