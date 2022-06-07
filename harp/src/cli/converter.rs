use std::io::{BufRead, Write};
use std::str::FromStr;
use std::{fs::File, fs::OpenOptions, io::BufReader, path::PathBuf};

use crate::ddl::domain::field_name::FieldName;
use crate::ddl::domain::field_size::FieldSize;
use crate::ddl::domain::query_start::QueryStart;
use crate::ddl::domain::to_query_string::ToQueryString;
use crate::ddl::field_types::FieldType;
use crate::ddl::key_types::KeyType;
use crate::ddl::query::{Field, Query};
use crate::ddl::remark_types::RemarkType;

const CODE_BLOCK: &str = "```mermaid";
// const ER_START: &str = "erDiagram";
const ENTITY_START: &str = " {";
const ENTITY_END: &str = "}";

/// # convert your er-diagram to ddl
///
pub fn converte_to_ddl(input_path: PathBuf, output_path: PathBuf) -> std::io::Result<()> {
    let input_file = OpenOptions::new().read(true).open(&input_path)?;
    println!("Target input file: {:?}", input_file);
    let reader = BufReader::new(input_file);
    let query_vec: &mut Vec<Query> = &mut Vec::new();
    read_file_to_query(query_vec, reader)?;

    // write query
    for query in query_vec {
        // Save the original output path
        let mut output_each_path = output_path.clone();
        output_each_path.push(String::from(query.query_start_mut().clone()) + ".sql");
        println!("{:?}", output_each_path);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&output_each_path)?;
        // write query starting
        writeln!(file, "{}", query.query_start_mut().to_query_string())?;

        for f in query.field_mut() {
            write_field(f, &mut file)?;
        }
        writeln!(file, "{}", ")")?
    }

    Ok(())
}

fn read_file_to_query(query_vec: &mut Vec<Query>, reader: BufReader<File>) -> std::io::Result<()> {
    let mut is_er_block: bool = false;
    let mut is_entity_block: bool = false;
    let mut query = Query::default();
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
                    *query.query_start_mut() = QueryStart::of(table_name).unwrap();
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
                    *field.field_name() = FieldName::of(field_line.next().unwrap()).unwrap();
                    // key or remark field
                    if let Some(next) = field_line.next() {
                        if let Ok(key_type) = KeyType::from_str(next) {
                            *field.key_type() = Some(key_type);
                        }
                        convert_remark(&mut field, next);
                    }
                    if let Some(next) = field_line.next() {
                        convert_remark(&mut field, next)
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
                        *field.field_size() = Some(
                            FieldSize::try_from(field_type_size.parse::<u16>().unwrap()).unwrap(),
                        );
                    }
                    field_vec.push(field);
                }
            }
        }
    }
    Ok(())
}

fn convert_remark(field: &mut Field, remark_fields: &str) {
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

fn write_field(field: &mut Field, file: &mut File) -> std::io::Result<()> {
    let mut field_out = String::from("  ");
    field_out = field_out + &field.field_name().to_query_string();
    field_out = field_out + " ";
    field_out = field_out + &field.field_type().as_ref();
    if let Some(size) = field.field_size() {
        field_out = field_out + &size.to_query_string();
    }
    // Not null
    if *field.is_not_null() {
        field_out = field_out + " NOT_NULL"
    }
    // Default value
    if let Some(default_value) = field.default_value() {
        field_out = field_out + " ";
        field_out = field_out + &default_value.to_string();
    }
    field_out = field_out + ",";
    // write field
    writeln!(file, "{}", field_out)?;

    Ok(())
}
