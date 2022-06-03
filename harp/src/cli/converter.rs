use std::io::BufRead;
use std::str::FromStr;
use std::{fs::File, fs::OpenOptions, io::BufReader, path::PathBuf};

use crate::ddl::field_types::FieldType;
use crate::ddl::key_types::KeyType;
use crate::ddl::query::{Field, Query};
use crate::ddl::remark_types::RemarkType;
use crate::ddl::template::create_query::CREATE_START;

const CODE_BLOCK: &str = "```mermaid";
// const ER_START: &str = "erDiagram";
const ENTITY_START: &str = " {";
const ENTITY_END: &str = "}";

/// # convert your er-diagram to ddl
///
pub fn converte_to_ddl(input_path: PathBuf, output_path: PathBuf) -> std::io::Result<()> {
    let input_file = OpenOptions::new().read(true).open(&input_path)?;

    let reader = BufReader::new(input_file);

    let mut is_er_block: bool = false;
    let mut is_entity_block: bool = false;
    let mut query = Query::default();
    let mut query_vec: Vec<Query> = Vec::new();
    let mut field: Field;
    let mut field_vec: Vec<Field> = Vec::new();
    for line in reader.lines() {
        let s = &line.unwrap();
        if !is_er_block {
            if let Some(_) = s.find(CODE_BLOCK) {
                is_er_block = true;
            }
        } else {
            if !is_entity_block {
                if let Some(_) = s.find(ENTITY_START) {
                    let table_name = s.replace(ENTITY_START, "");
                    *query.table_name_mut() = table_name;
                    is_entity_block = true; // into entity block
                }
            } else {
                if let Some(_) = s.find(ENTITY_END) {
                    *query.field_mut() = field_vec.clone();
                    query_vec.push(query.clone());
                    field_vec = Vec::new(); // prepare the next entity
                    query = Query::default(); // prepare the next entity
                    is_entity_block = false; // exit entity block
                } else {
                    field = Field::default(); // reinitialize Field
                    let mut field_line = s.split_whitespace();
                    // type field
                    let field_type = field_line.next().unwrap();
                    // name field
                    *field.field_name() = field_line.next().unwrap().to_owned();
                    // key or remark field
                    if let Some(next) = field_line.next() {
                        if let Ok(key_type) = KeyType::from_str(next) {
                            *field.key_type() = Some(key_type);
                        }
                        remark_convert(&mut field, next);
                    }
                    if let Some(next) = field_line.next() {
                        remark_convert(&mut field, next)
                    }

                    // if "String", it shoud be something like "varchar_32"
                    // if "Number" it shoud be something like "integer"
                    let mut field_types = field_type.split("_");
                    if let Some(field_type_separated) = field_types.next() {
                        if let Ok(exist_type) = FieldType::from_str(field_type_separated) {
                            *field.field_type() = exist_type;
                        }
                    }
                    if let Some(field_type_size) = field_types.next() {
                        *field.field_size() = Some(field_type_size.parse::<u16>().unwrap());
                    }
                    field_vec.push(field);
                }
            }
        }
    }

    for mut query in query_vec {
        let query_out = CREATE_START;
        for f in query.field_mut() {
            let mut field_out: &str = "  ";
            //field_out = field_out.to_string() + f.field_name();

            println!("{:?}", f)
        }
    }

    File::create(output_path).unwrap();

    Ok(())
}

fn remark_convert(field: &mut Field, remark_fields: &str) {
    let remark_fields = remark_fields.replace(r#"""#, "");
    for r in remark_fields.split_whitespace() {
        if let Ok(remark_type) = RemarkType::from_str(r) {
            match remark_type {
                RemarkType::NotNull => *field.is_not_null() = true,
                RemarkType::DafaultNull => *field.default_value() = Some(RemarkType::DafaultNull),
                RemarkType::DefaultCurrentTimestamp => {
                    *field.default_value() = Some(RemarkType::DefaultCurrentTimestamp)
                }
                RemarkType::Nothing => todo!(),
            }
        }
    }
}

// impl<Format> std::fmt::Display for Vec<Format> {
//     fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//         Ok(())
//     }
// }
