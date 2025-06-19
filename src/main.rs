fn main() {
    let mut class_file = rsjrs::get_jar_files("/home/build/jar_cache/");
    class_file.push("/home/build/rsproject/javaexample/".to_owned());

    let jvm = rsjrs::JVM::new(class_file);

    make_report_from_data_items_list(&jvm);
    make_report_from_json(&jvm);
}

fn make_report_from_data_items_list(jvm: &rsjrs::JVM) {
    let mut jasper_report = rsjrs::JasperReport::new(jvm);

    let items_list = jasper_report.create_data_items_list();
    {
        let item = jasper_report.create_data_item("Item");
        item.set_string("setName", "iPhone 6S");
        item.set_double("setPrice", 65000.00);
        items_list.add(item);
    }
    {
        let item = jasper_report.create_data_item("Item");
        item.set_string("setName", "iPad pro");
        item.set_double("setPrice", 70000.00);
        items_list.add(item);
    }

    let jasper_file = "/home/build/rsproject/javaexample/template_table_items.jasper";
    let report_file = "/home/build/rsproject/javaexample/template_table_items.pdf";

    let report_parameters = jasper_report.create_report_parameters();
    report_parameters.set_data_items_list("ItemsDataSource", items_list);

    let data_source = jasper_report.create_empty_data_source();
    let jasper_print = jasper_report.fill_report(jasper_file, report_parameters, Some(data_source));
    jasper_print.save_to_pdf(report_file);
}

fn make_report_from_json(jvm: &rsjrs::JVM) {
    let mut jasper_report = rsjrs::JasperReport::new(jvm);

    let json_string: String =
        std::fs::read_to_string("/home/build/rsproject/javaexample/devices.json").unwrap();

    let jasper_file = "/home/build/rsproject/javaexample/template_table_json.jasper";
    let report_file = "/home/build/rsproject/javaexample/template_table_json.pdf";

    let report_parameters = jasper_report.create_report_parameters();
    report_parameters.set_string("USER_JSON", json_string);

    let jasper_print = jasper_report.fill_report(jasper_file, report_parameters, None);
    jasper_print.save_to_pdf(report_file);
}
