use log::*;
use simple_logger::SimpleLogger;

// -- main controller -----------------------------------------------

fn main() {

    // -- init logging
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    debug!("logging ready");

    // -- set marc file path
    let marc_xml_path: String = match std::env::var("MRC_EXP__MARCXML_FILE_PATH") {
        Ok(val) => val,
        Err(e) => panic!(
            "\n\nCouldn't interpret MARC_XML_PATH; error, ``{:?}``; are envars loaded?\n\n",
            e
        ),
    };
    debug!("marc_xml_path, ``{:?}``", marc_xml_path);


    println!("Hello, world!");

    info!("end of main()");
}
