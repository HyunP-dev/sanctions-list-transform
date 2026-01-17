mod maltego;

use maltego::*;
use csv::ReaderBuilder;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (i, arg) in env::args().enumerate() {
        if i == 0 {}
        if i == 1 {
            let mut rdr = ReaderBuilder::new()
                .flexible(true)
                .from_path("data/sdn.csv")?;

            for result in rdr.records() {
                let record = result?;
                if record.len() >= 12 {
                    let data = record.get(11).unwrap();
                    if data.contains(&arg) {
                        let name = record.get(1).unwrap();
                        let message = MaltegoMessage {
                            response: MaltegoTransformResponseMessage {
                                entities: Entities {
                                    entities: vec![Entity {
                                        entity_type: "maltego.Person".to_string(),
                                        value: String::from(name),
                                        weight: 100,
                                    }],
                                },
                                ui_messages: UIMessages {
                                    messages: vec![UIMessage {
                                        message_type: "Debug".to_string(),
                                        content: format!("DEBUG: {}", data).to_string()
                                    }],
                                },
                            },
                        };
                        let xml = quick_xml::se::to_string(&message).unwrap();
                        println!("{}", xml);
                    }
                }
            }
        }
        if i == 2 {}
    }

    return Ok(());
}
