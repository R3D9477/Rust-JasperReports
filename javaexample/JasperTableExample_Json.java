import java.io.File;
import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.OutputStream;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import net.sf.jasperreports.engine.JREmptyDataSource;
import net.sf.jasperreports.engine.JRException;
import net.sf.jasperreports.engine.JasperExportManager;
import net.sf.jasperreports.engine.JasperFillManager;
import net.sf.jasperreports.engine.JasperPrint;
import net.sf.jasperreports.engine.data.JRBeanCollectionDataSource;

public class JasperTableExample_Json {

    public static void main(String[] args) {
        try {
            String jasperFile = "template_table_json.jasper";
            String outputFile = "JasperTableExample_Json.pdf";
            String jsonFile = "devices.json";

            String jsonString = "";

            BufferedReader reader = new BufferedReader(new FileReader(jsonFile));
            String line = reader.readLine();
            while (line != null) {
                jsonString += line;
                line = reader.readLine();
            }
            reader.close();

            Map<String, Object> parameters = new HashMap<String, Object>();
            parameters.put("USER_JSON", jsonString);

            JasperPrint jasperPrint = JasperFillManager.fillReport(jasperFile, parameters);

            OutputStream outputStream = new FileOutputStream(new File(outputFile));
            JasperExportManager.exportReportToPdfStream(jasperPrint, outputStream);

            System.out.println("File Generated");
        } catch (JRException ex) {
            ex.printStackTrace();
        } catch (FileNotFoundException ex) {
            ex.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
