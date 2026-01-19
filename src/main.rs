mod maltego;

use maltego::*;
use std::env;

mod sdn;
use sdn::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let [_, value, _] = args.as_slice() else {
        eprintln!("run as maltego local transform.");
        std::process::exit(1);
    };

    for record in get_sdn_list() {
        if !record.remarks.contains(value) {
            continue;
        }

        let mut entities: Vec<Entity> = Vec::new();
        for remark in record.remarks.split("; ") {
            if !remark.contains("Digital Currency Address - XBT ") {
                continue;
            }
            let tokens: Vec<&str> = remark.split("XBT ").collect();
            entities.push(Entity {
                entity_type: "maltego.BTCAddress".to_string(),
                value: tokens[1].to_string(),
                weight: 100,
            })
        }

        entities.push(Entity {
            entity_type: "maltego.Person".to_string(),
            value: record.sdn_name,
            weight: 100,
        });

        let message = MaltegoMessage {
            response: MaltegoTransformResponseMessage {
                entities: Entities { entities: entities },
                ui_messages: UIMessages {
                    messages: vec![UIMessage {
                        message_type: "Debug".to_string(),
                        content: format!("DEBUG: {}", record.remarks).to_string(),
                    }],
                },
            },
        };
        println!("{}", message.to_xml());
    }

    return Ok(());
}
