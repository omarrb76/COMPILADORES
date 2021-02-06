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
import java.awt.event.WindowAdapter;
import java.awt.event.WindowEvent;
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
    Boolean acabadoDeCerrar; // Para que se controle un evento del texto dentro de areaTexto
    
    public InterfazGrafica(){
        super("CompIDE");
        titulo = "CompIDE";
        mismoArchivo = false;
        editado = false;
        acabadoDeCerrar = false;
        manipuladorArchivos = new ManipuladorArchivos();
        this.setSize(800,600);
        this.setIconImage(new ImageIcon(this.getClass().getResource("/res/img/logo.png")).getImage());
        this.setDefaultCloseOperation(DO_NOTHING_ON_CLOSE); // Para que no haga nada en esto, más bien se ejecuta una función que viene más abajo
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
        
        // CREAMOS EL AREA DE TEXTO
        areaTexto = new JTextArea(5, 5);
        areaTexto.setLineWrap(true);
        areaTexto.setWrapStyleWord(true);
        areaTexto.setEnabled(false);
        
        // ESTE PEDAZO DE CÓDIGO CHECA SI HUBO UN CAMBIO EN EL CONTENIDO DEL AREA DE TEXTO
        areaTexto.getDocument().addDocumentListener(new DocumentListener() {
            @Override
            public void insertUpdate(DocumentEvent e) {
                checarCambioTexto();
            }
            @Override
            public void removeUpdate(DocumentEvent e) {
                checarCambioTexto();
            }
            @Override
            public void changedUpdate(DocumentEvent e) {
                checarCambioTexto();
            }
        });
        
        // PARA QUE EL USUARIO TENGA UN SCROLLER PARA RECORER EL TEXTO EN CASO DE SER MUCHO
        JScrollPane scroll = new JScrollPane(areaTexto);
        this.getContentPane().add(barraMenu, BorderLayout.NORTH);
        this.getContentPane().add(areaTexto, BorderLayout.CENTER);
        
        // AGREGAMOS LA BARRA DE MENU Y PONEMOS COMO VISIBLE NUESTRO FRAME
        this.setJMenuBar(barraMenu);
        this.setVisible(true);
    }
    
    // LOS DIFERENTES COMPONENTES DE LOS MENUS
    private JMenu crearMenuArchivo() {
        JMenu archivo = new JMenu("Archivo");
        archivo.setMnemonic('A'); // ALT + A
        
        // NUEVO ARCHIVO
        JMenuItem nuevo = new JMenuItem("Nuevo");
        // LAS FUNCIONES DE LOS BOTONES ESTAN SUPER CONFUSAS, ASI QUE AL MENOS QUE QUIERAS SABER QUE
        // RAYOS ESTA PASANDO, LO DEJARE A QUIEN GUSTE
        nuevo.addActionListener((ActionEvent e) -> {
            if (editado) {
                int result = cerrarArchivo();
                if (result == JOptionPane.YES_OPTION){
                    cerrarArchivoNuevo();
                }
            } else {
                cerrarArchivoNuevo();
            }
        });
        nuevo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_N, ActionEvent.CTRL_MASK)); // CTRL + N
        nuevo.setIcon(crearIcono("/res/img/nuevo_archivo.png"));
        archivo.add(nuevo);
        
        // ABRIR ARCHIVO
        JMenuItem abrir = new JMenuItem("Abrir");
        abrir.addActionListener((ActionEvent e) -> {
            if (editado) {
                int result = cerrarArchivo();
                if (result == JOptionPane.NO_OPTION){
                    cerrarArchivoAbrir();
                }
            } else {
                cerrarArchivoAbrir();
            }
        });
        abrir.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_O, ActionEvent.CTRL_MASK)); // CTRL + O
        abrir.setIcon(crearIcono("/res/img/abrir_archivo.png"));
        archivo.add(abrir);
        
        // GUARDAR ARCHIVO
        JMenuItem guardar = new JMenuItem("Guardar");
        guardar.addActionListener((ActionEvent e) -> {
            guardarArchivo(mismoArchivo);
        });
        guardar.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_S, ActionEvent.CTRL_MASK)); // CTRL + S
        guardar.setIcon(crearIcono("/res/img/guardar.png"));
        archivo.add(guardar);
        
        // GUARDAR COMO ARCHIVO
        JMenuItem guardarComo = new JMenuItem("Guardar como");
        guardarComo.addActionListener((ActionEvent e) -> {
            editado = true; // Para que forsozamente nos muestre la ventana de elegir donde guardar
            guardarArchivo(false);
        });
        guardarComo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_S, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK)); // CTRL + SHIFT + S
        guardarComo.setIcon(crearIcono("/res/img/guardar.png"));
        archivo.add(guardarComo);
        
        // CERRAR ARCHIVO
        JMenuItem cerrarArchivo = new JMenuItem("Cerrar archivo");
        cerrarArchivo.addActionListener((ActionEvent e) -> {
            int resultado = cerrarArchivo();
            // Cerramos el archivo
            if (resultado == JOptionPane.YES_OPTION){
                acabadoDeCerrar = true;
                editado = false;
                mismoArchivo = false;
                areaTexto.setEnabled(false);
                areaTexto.setText(null);
                titulo = "CompIDE";
                this.setTitle(titulo);
            }
        });
        cerrarArchivo.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_B, ActionEvent.CTRL_MASK)); // CTRL + B
        cerrarArchivo.setIcon(crearIcono("/res/img/cerrar_archivo.png"));
        archivo.add(cerrarArchivo);
        
        archivo.add(new JSeparator()); // Una rayita separadora.
        
        // SALIR
        JMenuItem salir = new JMenuItem("Salir");
        salir.addActionListener((ActionEvent e) -> {
            int resultado = cerrarArchivo();
            if (resultado == JOptionPane.YES_OPTION){
                // Salimos de la aplicacion
                System.exit(0);
            }
        });
        salir.setAccelerator(KeyStroke.getKeyStroke(KeyEvent.VK_Q, ActionEvent.CTRL_MASK | ActionEvent.SHIFT_MASK)); // CTRL + SHIFT + Q
        salir.setIcon(crearIcono("/res/img/salir.png"));
        archivo.add(salir);
        
        // SALIR CON LA TACHA DE LA APLICACION
        this.addWindowListener(new WindowAdapter() {
            @Override
            public void windowClosing(WindowEvent e) {
                int resultado = cerrarArchivo();
                if (resultado == JOptionPane.YES_OPTION){
                    // Salimos de la aplicacion
                    System.exit(0);
                }
            }
        });
        
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
    private int guardarArchivo(Boolean guardarComo){
       int result = JFileChooser.CANCEL_OPTION;
        if (editado && areaTexto.isEnabled()) { // Tenemos que corroborar que exista un archivo a guardar, por eso puse el areaTexto.isEnabled();
            if (!guardarComo) {
                JFileChooser fileChooser = new JFileChooser();
                int seleccion = fileChooser.showSaveDialog(areaTexto);
                if (seleccion == JFileChooser.APPROVE_OPTION) {
                   File fichero = fileChooser.getSelectedFile();
                   manipuladorArchivos.setArchivo(fichero);
                } else {
                   return result;
                }
            }
            result = JFileChooser.APPROVE_OPTION;
            manipuladorArchivos.escribirTexto(areaTexto.getText());
            editado = false; // Lo acabamos de recien guardar, no puede estar editado
            titulo = "CompIDE - " + manipuladorArchivos.getArchivo().getName();
            this.setTitle(titulo);
            mismoArchivo = true;
        }
        return result;
    }
    
    // Este método se invoca cuando cerramos el archivo o aplicacion sin que hayamos guardado el archivo
    // Regresa la opcion que elijio el usuario, para saber si salir de la aplicacion o no
    private int cerrarArchivo(){
        int opcion = JOptionPane.YES_OPTION;
        if (areaTexto.isEnabled() && editado){ // Significa que hay un archivo ahi
            String title = "No se ha guardado el archivo: ";
            File nombreArchivo = manipuladorArchivos.getArchivo();
            if (nombreArchivo != null){ // Si existe el archivo
                title += nombreArchivo.getName() + "\n" + "¿Desea guardarlo?";
            } else {
                title += "Archivo nuevo" + "\n" + "¿Desea guardarlo?";
            }
            int result = JOptionPane.showConfirmDialog(this, title, "CompIDE", JOptionPane.YES_NO_OPTION);
            switch (result) {
                case JOptionPane.YES_OPTION:
                    System.out.println("Entre en la opcion SI del panel");
                    result = guardarArchivo(mismoArchivo);
                    if (result == JFileChooser.CANCEL_OPTION){
                        System.out.println("Entre en cancelar");
                        opcion = JOptionPane.NO_OPTION;
                    } else if (result == JFileChooser.APPROVE_OPTION){
                        System.out.println("Entre en aceptar");
                        opcion = JOptionPane.YES_OPTION;
                    }
                    break;
                case JOptionPane.NO_OPTION:
                    opcion = JOptionPane.YES_OPTION;
                    break;
                default:
                    opcion = JOptionPane.NO_OPTION;
                    break;
            }
        }
        return opcion;
    }
    
    // Metodo que cree para que no repitamos codigo
    private void cerrarArchivoNuevo(){
        areaTexto.setText(null);
        areaTexto.setEnabled(true);
        titulo = "CompIDE - Archivo nuevo *";
        setTitle(titulo);
        editado = true; // Porque acabamos de crear un nuevo archivo
        mismoArchivo = false;
        manipuladorArchivos.setArchivo(null); // No tenemos ningun archivo
        areaTexto.requestFocus();
    }
    
    private void cerrarArchivoAbrir(){
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
    }
    
    // FUNCION QUE PERTENECE AL CAMBIO DE TEXTO DEL areaTexto
    private void checarCambioTexto(){
        if (!editado){
            titulo += " *";
            setTitle(titulo);
        }
        if (!acabadoDeCerrar){
            editado = true;
            acabadoDeCerrar = false;
        }
    }
}
