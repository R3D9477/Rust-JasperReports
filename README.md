## Rust bindings for JasperReport library
* can fill report with data items or with JSON

### overview:
* rsjrs::JWM
  * `new`
* rsjrs::JasperReport
  * `new`
  * `create_data_items_list`
  * `create_data_item`
  * `create_report_parameters`
  * `create_empty_data_source`
  * `fill_report`
* rsjrs::JasperPrint
  * `new`
  * `save_to_pdf`
* rsjrs::JRDataSource
  * `new_empty_data_source`
* rsjrs::ReportParameters
  * `new`
* rsjrs::DataItemsList
  * `new`
  * `add`
* rsjrs::DataItem
  * `new`
  * `set_integer`
  * `set_double`
  * `set_string`

### how to:
* open in a container
* verify java side work: `cd javaexample ; ./run_java_examples.sh` & check traget PDFs in folder `javaexample`
* remove generated PDFs in folder `javaexample`
* run `cargo run` & check traget PDFs in folder `javaexample`

Repository based on [vscLazyTemplate_Rust](https://github.com/R3D9477/vscLazyTemplate_Rust)

---

### If you like that repo, you can support me, I really appreciate it :heart: [![ko-fi](https://www.ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/R3D9477)
