import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.OutputStream;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import java.util.jar.JarFile;
import java.util.Enumeration;
import java.util.jar.JarEntry;
import java.net.URL;
import java.net.URLClassLoader;
import java.io.IOException;
import java.net.MalformedURLException;
import java.lang.ClassNotFoundException;

public class JasperTableExample {

    public static void main(String[] args) {

        try {
            String pathToJar = "../../jar_cache/jsperreports-7.0.3.jar";

            JarFile jarFile = new JarFile(pathToJar);
            Enumeration<JarEntry> e = jarFile.entries();

            URL[] urls = { new URL("jar:file:" + pathToJar+"!/") };
            URLClassLoader cl = URLClassLoader.newInstance(urls);

            while (e.hasMoreElements()) {
                JarEntry je = e.nextElement();
                if(je.isDirectory() || !je.getName().endsWith(".class")){
                    continue;
                }
                // -6 because of .class
                String className = je.getName().substring(0,je.getName().length()-6);
                className = className.replace('/', '.');
                //System.out.println("> " + className);
                try {
                    Class c = cl.loadClass(className);
                }
                catch (java.lang.ClassNotFoundException ex) {
                    System.out.println(" --- " + ex.toString());
                }
                catch (java.lang.NoClassDefFoundError ex) {
                    System.out.println(" --- " + ex.toString());
                }
            }
        } catch (IOException ex) {
            ex.printStackTrace();
        }
    }
}
