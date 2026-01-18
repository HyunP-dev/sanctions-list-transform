use crate::SDNRecord;
use csv::ReaderBuilder;

pub fn get_sdn_list() -> Vec<SDNRecord> {
    let mut vec: Vec<SDNRecord> = Vec::new();
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path("data/sdn.csv")
        .unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        if record.len() != 12 {
            continue;
        }
        vec.push(SDNRecord {
            ent_num: record.get(0).unwrap().parse::<u32>().unwrap(),
            sdn_name: String::from(record.get(1).unwrap()),
            sdn_type: String::from(record.get(2).unwrap()),
            program: String::from(record.get(3).unwrap()),
            title: String::from(record.get(4).unwrap()),
            call_sign: String::from(record.get(5).unwrap()),
            vess_type: String::from(record.get(6).unwrap()),
            tonnage: String::from(record.get(7).unwrap()),
            grt: String::from(record.get(8).unwrap()),
            vess_flag: String::from(record.get(9).unwrap()),
            vess_owner: String::from(record.get(10).unwrap()),
            remarks: String::from(record.get(11).unwrap()),
        });
    }

    return vec;
}
