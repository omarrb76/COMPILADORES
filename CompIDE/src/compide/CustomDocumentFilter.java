/*
 Este archivo se encarga de colorear el documento para que el usuario pueda ver mejor que rayos esta escribiendo
*/
package compide;

import java.awt.Color;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import javax.swing.JTextPane;
import javax.swing.SwingUtilities;
import javax.swing.text.AttributeSet;
import javax.swing.text.BadLocationException;
import javax.swing.text.DocumentFilter;
import javax.swing.text.StyleConstants;
import javax.swing.text.StyleContext;
import javax.swing.text.StyledDocument;

public class CustomDocumentFilter extends DocumentFilter {
        
    // Variables de estilo y para guardar la referencia de areaTexto (se encuentra en la interfaz gráfica)
    private final StyledDocument styledDocument;
    private final JTextPane areaTexto;

    // COLORES PARA PINTAR
    private final StyleContext styleContext = StyleContext.getDefaultStyleContext();
    private final AttributeSet grayAttributeSet = styleContext.addAttribute(styleContext.getEmptySet(), StyleConstants.Foreground, Color.GRAY);
    private final AttributeSet blueAttributeSet = styleContext.addAttribute(styleContext.getEmptySet(), StyleConstants.Foreground, Color.BLUE);
    private final AttributeSet redAttributeSet = styleContext.addAttribute(styleContext.getEmptySet(), StyleConstants.Foreground, Color.RED);
    private final AttributeSet magentaAttributeSet = styleContext.addAttribute(styleContext.getEmptySet(), StyleConstants.Foreground, Color.MAGENTA);
    private final AttributeSet blackAttributeSet = styleContext.addAttribute(styleContext.getEmptySet(), StyleConstants.Foreground, Color.BLACK);

    // Use a regular expression to find the words you are looking for
    Pattern patternComentario = buildPatternComentario();
    Pattern patternPalabrasReservadas = buildPatternPalabrasReservadas();
    Pattern patternSimbolosEspeciales = buildPatternSimbolosEspeciales();
    Pattern patternNumeros = buildPatternNumeros();

    // COSEGUIMOS EL AREA DE TEXTO
    public CustomDocumentFilter(JTextPane areaTexto) {
        this.areaTexto = areaTexto;
        styledDocument = areaTexto.getStyledDocument();
    }

    // OVERRIDES, TODOS MANDAN A LLAMAR A PINTAR EL TEXTO
    @Override
    public void insertString(DocumentFilter.FilterBypass fb, int offset, String text, AttributeSet attributeSet) throws BadLocationException {
        super.insertString(fb, offset, text, attributeSet);
        handleTextChanged();
    }

    @Override
    public void remove(DocumentFilter.FilterBypass fb, int offset, int length) throws BadLocationException {
        super.remove(fb, offset, length);
        handleTextChanged();
    }

    @Override
    public void replace(DocumentFilter.FilterBypass fb, int offset, int length, String text, AttributeSet attributeSet) throws BadLocationException {
        super.replace(fb, offset, length, text, attributeSet);
        handleTextChanged();
    }

    // HILO PARA PINTAR EL TEXTO, PARA QUE NO INTERRUMPA AL USUARIO
    private void handleTextChanged() {
        SwingUtilities.invokeLater(new Runnable() {
            @Override
            public void run() {
                updateTextStyles();
            }
        });
    }

    // PATTRONES DE COMENTARIOS
    private Pattern buildPatternComentario() {
        StringBuilder sb = new StringBuilder();
        sb.append("(//.*$?)|(/\\*.[^\\*/]*(\\*/))");
        Pattern p = Pattern.compile(sb.toString());
        return p;
    }
    
    // PATRONES DE PALABRAS RESERVADAS
    private Pattern buildPatternPalabrasReservadas() {
        StringBuilder sb = new StringBuilder();
        sb.append("\\bprogram\\b|\\bif\\b|\\belse\\b|\\bfi\\b|\\bdo\\b|\\buntil\\b|\\bwhile\\b|\\bread\\b|\\bwrite\\b|\\bfloat\\b|\\bint\\b|\\bbool\\b|\\bnot\\b|\\band\\b|\\bor\\b|\\btrue\\b|\\bfalse\\b|\\bthen\\b");
        Pattern p = Pattern.compile(sb.toString());
        return p;
    }
    
    // PATRONES DE SIMBOLOS ESPECIALES
    private Pattern buildPatternSimbolosEspeciales() {
        StringBuilder sb = new StringBuilder();
        sb.append("\\+|\\-|\\*|\\/|\\^|<|<=|>|>=|==|!=|=|;|,|\\(|\\)|\\{|\\}");
        Pattern p = Pattern.compile(sb.toString());
        return p;
    }

    // PATRONES DE NUMEROS
    private Pattern buildPatternNumeros() {
        StringBuilder sb = new StringBuilder();
        sb.append("\\b\\d+\\b");
        Pattern p = Pattern.compile(sb.toString());
        return p;
    }

    // PINTAR EL TEXTO
    private void updateTextStyles() {
        // Clear existing styles
        styledDocument.setCharacterAttributes(0, areaTexto.getText().length(), blackAttributeSet, true);
        
        // Por cuestiones de posicionamiento de texto y debugeo
        String linea = areaTexto.getText().replaceAll("\\r", "");

        // PINTAMOS SIMBOLOS ESPECIALES
        Matcher matcher = patternSimbolosEspeciales.matcher(linea);
        while (matcher.find()) {
            // Change the color of recognized tokens
            styledDocument.setCharacterAttributes(matcher.start(), matcher.end() - matcher.start(), redAttributeSet, false);
        }
        
        // PINTAMOS PALABRAS RESERVADAS
        matcher = patternPalabrasReservadas.matcher(linea);
        while (matcher.find()) {
            // Change the color of recognized tokens
            styledDocument.setCharacterAttributes(matcher.start(), matcher.end() - matcher.start(), blueAttributeSet, false);
        }
        
        // PINTAMOS NUMEROS
        matcher = patternNumeros.matcher(linea);
        while (matcher.find()) {
            // Change the color of recognized tokens
            styledDocument.setCharacterAttributes(matcher.start(), matcher.end() - matcher.start(), magentaAttributeSet, false);
        }
        
        // PINTAMOS COMENTARIOS
        matcher = patternComentario.matcher(linea);
        while (matcher.find()) {
            // Change the color of recognized tokens
            styledDocument.setCharacterAttributes(matcher.start(), matcher.end() - matcher.start(), grayAttributeSet, false);
        }
    }
}