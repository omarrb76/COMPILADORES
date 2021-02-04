package compide;

public class CompIDE {
    public static void main(String[] args) {
        try {
            for (javax.swing.UIManager.LookAndFeelInfo info : javax.swing.UIManager.getInstalledLookAndFeels()) {
                if ("Metal".equals(info.getName())) {
                    javax.swing.UIManager.setLookAndFeel(info.getClassName());
                    break;
                }
            }
        } catch (Exception e){
            e.printStackTrace();
        }
        
        InterfazGrafica ide = new InterfazGrafica();
        /*ManipuladorArchivos m = new ManipuladorArchivos();
        m.leerTexto("src/res/prueba.txt");
        System.out.println(m.getTexto());
        m.setNombreArchivo("src/res/resultado.txt");
        m.escribirTexto("Soy omar y estoy probando mi programa\nEspero les guste mucho");*/
    }
}
