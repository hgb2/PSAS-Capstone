use std::fs::File;
use std::io::BufReader;
//use xml::common::XmlVersion;
use self::xml::common::XmlVersion;

//use xml::reader::{EventReader, XmlEvent};
use self::xml::reader::{EventReader, XmlEvent};
use ConfigPins;

extern crate xml;


///////////////////////////////////////////////////////////////////////////////
// Function name: xml_reader
//
// Purpose: Reads pin addresses from an XML file.
//		
// INPUTS: name   -- struct for pin addresses.
//
// RETURN: Ok(0)  -- all is well
//         Err(xml::reader::Error) -- an error occurred while reading from file
//
///////////////////////////////////////////////////////////////////////////////
pub fn xml_reader(config_pins: &mut ConfigPins ) -> Result<u8, xml::reader::Error> {

    let file: File;
    match File::open("../common/Pin_Config.xml") {
        Ok(f_in) => { file = f_in; },
        Err(e) => panic!("Failed to open configuration file: {}", e),
    }
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    
    
    for e in parser {
        match e {
            Ok(XmlEvent::StartDocument { version, encoding, .. })=> {
            //xml-rs 0.3 is "mostly" XML version 1.0 complient, and supports only utf-8.
                
                // Verify  XML version 1.0
                print!("\nXML version: {}\n", version);
                if version != XmlVersion::Version10 {
                    panic!("Configuration file XML version is not 1.0");
                }
                
                //Verify utf-8 encoding.
                print!("XML encoding: {}\n", encoding);
                if !((encoding == "utf-8") || (encoding == "UTF-8")) {
                    panic!("Configuration file XML encoding is not utf-8");
                }
                
            }

            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                let check_name: String = name.local_name;
                
                // Set cw_pin pin value
                if check_name == "cw_pin" {
                    if attributes.len() == 0 {
                        panic!("XML Configuration file: specify value for cw_pin.");
                    }
                    //cast to u32, then to u64
                    let temp: u32;
                    match attributes[0].value.parse() {
                        Ok(val) => { temp = val },
                        Err(e) => panic!("Failed to retrieve cw_pin, error: {}", e),
                    }
                    config_pins.cw_pin = temp as u64;
                    
                // Set cw_pin pin value
                } else if  check_name == "ccw_pin" {
                    if attributes.len() == 0 {
                        panic!("XML Configuration file: specify value for ccw_pin.");
                    }
                    //cast to u32, then to u64
                    let temp: u32;
                    match attributes[0].value.parse() {
                        Ok(val) => { temp = val },
                        Err(e) => panic!("Failed to retrieve ccw_pin, error: {}", e),
                    }
                    config_pins.ccw_pin = temp as u64;
                    
                // Set estop pin value
                } else if  check_name == "estop_pin" {
                    if attributes.len() == 0 {
                        panic!("XML Configuration file: specify value for estop_pin.");
                    }
                    let temp: u32;
                    match attributes[0].value.parse() {
                        Ok(val) => { temp = val },
                        Err(e) => panic!("Failed to retrieve estop_pin, error: {}", e),
                    }
                    config_pins.estop_pin = temp as u64;   
                }
            } Err(e) => {
                //println!("Error: {}", e);
                return Err(e);
            }
            _ => {}
        }  
    }
    Ok(0)
}