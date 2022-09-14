use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::{Value};


struct CodonTable {
    species: String,
    codon_usage_table: HashMap<char, HashMap<String, f32>>,
    translations_table: HashMap<String, char>
}

fn read_json(raw_json:&str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed
}


impl CodonTable {

    fn load_usage_table(&self, codon_usage_fpath: String) {
        let mut file = File::open(codon_usage_fpath).expect("File not found!");
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        let json_object: Value = serde_json::from_str(&buff).unwrap();
        
        for (amino_acid, codons_map) in json_object.as_object().unwrap() {
            let amino_acid_bytes: u8 = amino_acid.as_bytes()[0];
            let amino_acid_char: char = amino_acid_bytes as char;
            
            let mut codon_usage_map: HashMap<String, f32> = HashMap::new();
            
            for (codon, usage) in codons_map[amino_acid].as_object().unwrap() {
                codon_usage_map.insert(codon.to_string(), usage);
                self.translations_table.insert(codon.to_string(), amino_acid_char);
            }
            
            self.codon_usage_table.insert(amino_acid_char, codon_usage_map);
        }

    }

    fn get_translation(&self, codon: String) -> Option<&char> {
        let amino_acid = self.translations_table.get(&codon);
        return amino_acid;
    }

    fn get_synonyms(&self, codon: String) -> Vec<&String> {
        let translation_res = self.get_translation(codon);

        if translation_res.is_none() {
            return vec![];
        }

        let amino_acid = translation_res.unwrap();
        
        let codons = self.codon_usage_table.get(&amino_acid);
        let mut synonyms = Vec::<&String>::new();
        for (codon, _) in &*codons.unwrap() {
            synonyms.push(codon);
        }

        return synonyms;
    }
}


fn main() {
    
    println!("Hello, world!");
}
