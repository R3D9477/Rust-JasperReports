use std::cell::RefCell;
use std::rc::Rc;

use jni::{AttachGuard, InitArgsBuilder, JNIVersion, JavaVM, objects::JObject, objects::JValue};

//------------------------------------------------------------------------------------------------------------------------

pub struct JVM {
    jvm: JavaVM,
}

impl JVM {
    pub fn new(class_path: Vec<String>) -> Self {
        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .option(format!("-Djava.class.path={}", class_path.join(":")))
            .build()
            .unwrap();

        Self {
            jvm: JavaVM::new(jvm_args).unwrap(),
        }
    }
}

pub struct DataItem<'a> {
    data_item: JObject<'a>,
    jasper_report: JasperReport<'a>,
}

impl<'a> DataItem<'a> {
    fn new(jasper_report: JasperReport<'a>, class_name: &'static str) -> Self {
        let cls_item = jasper_report
            .jenv
            .borrow_mut()
            .find_class(class_name)
            .unwrap();
        let obj_item = jasper_report
            .jenv
            .borrow_mut()
            .new_object(&cls_item, "()V", &[])
            .unwrap();

        Self {
            data_item: obj_item,
            jasper_report,
        }
    }

    pub fn set_integer(&self, setter_name: &'static str, value: i32) {
        self.jasper_report
            .jenv
            .borrow_mut()
            .call_method(&self.data_item, setter_name, "(I)V", &[JValue::Int(value)])
            .unwrap();
    }

    pub fn set_double(&self, setter_name: &'static str, value: f64) {
        self.jasper_report
            .jenv
            .borrow_mut()
            .call_method(
                &self.data_item,
                setter_name,
                "(D)V",
                &[JValue::Double(value)],
            )
            .unwrap();
    }

    pub fn set_string(&self, setter_name: &'static str, value: &'static str) {
        let str_value = self
            .jasper_report
            .jenv
            .borrow_mut()
            .new_string(value)
            .unwrap();
        self.jasper_report
            .jenv
            .borrow_mut()
            .call_method(
                &self.data_item,
                setter_name,
                "(Ljava/lang/String;)V",
                &[JValue::Object(&str_value)],
            )
            .unwrap();
    }
}

pub struct DataItemsList<'a> {
    array_list: JObject<'a>,
    jasper_report: JasperReport<'a>,
}

impl<'a> DataItemsList<'a> {
    fn new(jasper_report: JasperReport<'a>) -> Self {
        let cls_array_list = jasper_report
            .jenv
            .borrow_mut()
            .find_class("java/util/ArrayList")
            .unwrap();
        let obj_items_list = jasper_report
            .jenv
            .borrow_mut()
            .new_object(cls_array_list, "()V", &[])
            .unwrap();

        Self {
            array_list: obj_items_list,
            jasper_report: jasper_report,
        }
    }

    pub fn add(&self, data_item: DataItem<'a>) {
        self.jasper_report
            .jenv
            .borrow_mut()
            .call_method(
                &self.array_list,
                "add",
                "(Ljava/lang/Object;)Z",
                &[JValue::Object(&data_item.data_item)],
            )
            .unwrap();
    }
}

pub struct ReportParameters<'a> {
    report_parameters: JObject<'a>,
    jasper_report: JasperReport<'a>,
}

impl<'a> ReportParameters<'a> {
    fn new(jasper_report: JasperReport<'a>) -> Self {
        let cls_array_list = jasper_report
            .jenv
            .borrow_mut()
            .find_class("java/util/HashMap")
            .unwrap();
        let obj_items_list = jasper_report
            .jenv
            .borrow_mut()
            .new_object(cls_array_list, "()V", &[])
            .unwrap();

        Self {
            report_parameters: obj_items_list,
            jasper_report: jasper_report,
        }
    }

    pub fn set_data_items_list(&self, parameter_name: &'static str, data_items: DataItemsList<'a>) {
        let cls_jr_bean = self
            .jasper_report
            .jenv
            .borrow_mut()
            .find_class("net/sf/jasperreports/engine/data/JRBeanCollectionDataSource")
            .unwrap();

        let obj_jr_bean = self
            .jasper_report
            .jenv
            .borrow_mut()
            .new_object(
                &cls_jr_bean,
                "(Ljava/util/Collection;)V",
                &[JValue::Object(&data_items.array_list)],
            )
            .unwrap();

        let str_jr_datasource_name = self
            .jasper_report
            .jenv
            .borrow_mut()
            .new_string(parameter_name)
            .unwrap();

        self.jasper_report
            .jenv
            .borrow_mut()
            .call_method(
                &self.report_parameters,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                &[
                    JValue::Object(&str_jr_datasource_name),
                    JValue::Object(&obj_jr_bean),
                ],
            )
            .unwrap();
    }

    pub fn set_string(&self, parameter_name: &'static str, data_string: String) {
        let str_jr_datasource_name = self
            .jasper_report
            .jenv
            .borrow_mut()
            .new_string(parameter_name)
            .unwrap();

        let str_jr_data_string = self
            .jasper_report
            .jenv
            .borrow_mut()
            .new_string(data_string)
            .unwrap();

        self.jasper_report
            .jenv
            .borrow_mut()
            .call_method(
                &self.report_parameters,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                &[
                    JValue::Object(&str_jr_datasource_name),
                    JValue::Object(&str_jr_data_string),
                ],
            )
            .unwrap();
    }
}

pub struct JRDataSource<'a> {
    data_source: JObject<'a>,
}

impl<'a> JRDataSource<'a> {
    pub fn new_empty_data_source(jasper_report: JasperReport<'a>) -> Self {
        let cls_jr_empty_datasource = jasper_report
            .jenv
            .borrow_mut()
            .find_class("net/sf/jasperreports/engine/JREmptyDataSource")
            .unwrap();

        let obj_jr_empty_datasource = jasper_report
            .jenv
            .borrow_mut()
            .new_object(cls_jr_empty_datasource, "()V", &[])
            .unwrap();

        Self {
            data_source: obj_jr_empty_datasource,
        }
    }
}

pub struct JasperPrint<'a> {
    jasper_print: JObject<'a>,
    jasper_report: JasperReport<'a>,
}

impl<'a> JasperPrint<'a> {
    fn new(jasper_report: JasperReport<'a>, jobj: JObject<'a>) -> Self {
        Self {
            jasper_print: jobj,
            jasper_report: jasper_report,
        }
    }

    pub fn save_to_pdf(&self, pdf_report_file: &'static str) {
        let str_report_file = self
            .jasper_report
            .jenv
            .borrow_mut()
            .new_string(pdf_report_file)
            .unwrap();

        let cls_file = self
            .jasper_report
            .jenv
            .borrow_mut()
            .find_class("java/io/File")
            .unwrap();

        let obj_file = self
            .jasper_report
            .jenv
            .borrow_mut()
            .new_object(
                cls_file,
                "(Ljava/lang/String;)V",
                &[JValue::Object(&str_report_file)],
            )
            .unwrap();

        let cls_output_stream = self
            .jasper_report
            .jenv
            .borrow_mut()
            .find_class("java/io/FileOutputStream")
            .unwrap();

        let obj_output_stream = self
            .jasper_report
            .jenv
            .borrow_mut()
            .new_object(
                cls_output_stream,
                "(Ljava/io/File;)V",
                &[JValue::Object(&obj_file)],
            )
            .unwrap();

        let cls_jasper_export_manager = self
            .jasper_report
            .jenv
            .borrow_mut()
            .find_class("net/sf/jasperreports/engine/JasperExportManager")
            .unwrap();

        self.jasper_report
            .jenv
            .borrow_mut()
            .call_static_method(
                &cls_jasper_export_manager,
                "exportReportToPdfStream",
                "(Lnet/sf/jasperreports/engine/JasperPrint;Ljava/io/OutputStream;)V",
                &[
                    JValue::Object(&self.jasper_print),
                    JValue::Object(&obj_output_stream),
                ],
            )
            .unwrap();
    }
}

#[derive(Clone)]
pub struct JasperReport<'a> {
    pub jenv: Rc<RefCell<AttachGuard<'a>>>,
}

impl<'a> JasperReport<'a> {
    pub fn new(jvm: &'a JVM) -> Self {
        let ag: AttachGuard<'a> = jvm.jvm.attach_current_thread().unwrap();
        Self {
            jenv: Rc::new(RefCell::new(ag)),
        }
    }

    pub fn create_data_items_list(&self) -> DataItemsList<'a> {
        DataItemsList::new(self.clone())
    }

    pub fn create_data_item(&self, class_name: &'static str) -> DataItem<'a> {
        DataItem::new(self.clone(), class_name)
    }

    pub fn create_report_parameters(&mut self) -> ReportParameters<'a> {
        ReportParameters::new(self.clone())
    }

    pub fn create_empty_data_source(&mut self) -> JRDataSource<'a> {
        JRDataSource::new_empty_data_source(self.clone())
    }

    pub fn fill_report(
        &mut self,
        jasper_file: &'static str,
        report_parameters: ReportParameters<'a>,
        data_source: Option<JRDataSource>,
    ) -> JasperPrint<'a> {
        let str_jasper_file = self.jenv.borrow_mut().new_string(jasper_file).unwrap();

        let cls_jasper_fill_manager = self
            .jenv
            .borrow_mut()
            .find_class("net/sf/jasperreports/engine/JasperFillManager")
            .unwrap();

        let obj_jasper_print = match data_source {
            Some(some_jr_data_source) => {
                self.jenv.borrow_mut().call_static_method(
                    cls_jasper_fill_manager,
                    "fillReport",
                    "(Ljava/lang/String;Ljava/util/Map;Lnet/sf/jasperreports/engine/JRDataSource;)Lnet/sf/jasperreports/engine/JasperPrint;",
                    &[
                        JValue::Object(&str_jasper_file),
                        JValue::Object(&report_parameters.report_parameters),
                        JValue::Object(&some_jr_data_source.data_source),
                    ],
                )
                .unwrap().l().unwrap()
            }
            None => {
                self.jenv.borrow_mut().call_static_method(
                    cls_jasper_fill_manager,
                    "fillReport",
                    "(Ljava/lang/String;Ljava/util/Map;)Lnet/sf/jasperreports/engine/JasperPrint;",
                    &[
                        JValue::Object(&str_jasper_file),
                        JValue::Object(&report_parameters.report_parameters)
                    ],
                )
                .unwrap().l().unwrap()
            }
        };

        JasperPrint::new(self.clone(), obj_jasper_print)
    }
}

//------------------------------------------------------------------------------------------------------------------------

pub fn get_jar_files(jar_dir: &'static str) -> Vec<String> {
    std::fs::read_dir(jar_dir)
        .unwrap()
        .filter(|path| path.is_ok())
        .map(|path| path.unwrap().path())
        .filter(|path| path.extension().unwrap() == "jar")
        .map(|path| path.to_str().unwrap().to_owned())
        .collect()
}
