use std::io::{BufReader, BufWriter, Write};
use serde_json::Map;
use serde_json::Value as JsonValue;
use crate::data::json::JsonReader;
use crate::error::Error;
use crate::lang::value::Value;
use crate::methods::util::io::{file_or_stdin, file_or_stdout};

type Writer = BufWriter<Box<dyn Write>>;
type ValueMap = Map<String, JsonValue>;

pub(crate) fn json_to_tsv(input: Option<&str>, output: Option<&str>, buffer_size: usize)
                          -> Result<Value, Error> {
    let mut reader =
        JsonReader::from_reader(BufReader::new(file_or_stdin(input)?));
    let mut writer = BufWriter::new(file_or_stdout(output)?);
    let (buffer, header) = fill_buffer_and_header(&mut reader, buffer_size)?;
    writer.write_fmt(format_args!("{}\n", header.join("\t")))?;
    write_buffered(&mut writer, buffer, &header)?;
    write_remaining(&mut writer, reader, &header)?;
    Ok(Value::Unit)
}

fn fill_buffer_and_header(reader: &mut JsonReader, buffer_size: usize)
                          -> Result<(Vec<ValueMap>, Vec<String>), Error> {
    let mut buffer: Vec<ValueMap> = Vec::new();
    let mut header: Vec<String> = Vec::new();
    for value_map in reader.take(buffer_size) {
        let value_map = value_map?;
        for (key, _) in value_map.iter() {
            if !header.contains(key) {
                header.push(key.clone())
            }
        }
        buffer.push(value_map);
    }
    Ok((buffer, header))
}

fn write_buffered(writer: &mut Writer, buffer: Vec<ValueMap>, header: &[String]) -> Result<(), Error> {
    for value_map in buffer {
        write_row(writer, value_map, header)?
    }
    Ok(())
}

fn write_remaining(writer: &mut Writer, reader: JsonReader, header: &[String])
                   -> Result<(), Error> {
    for value_map in reader {
        let value_map = value_map?;
        write_row(writer, value_map, header)?;
    }
    Ok(())
}

fn write_row(writer: &mut Writer, mut value_map: ValueMap, header: &[String])
             -> Result<(), Error> {
    let line =
        header.iter().map(|key| value_map.remove(key)
            .map(|value| value.to_string()).unwrap_or_else(|| "".to_string()))
            .collect::<Vec<String>>().join("\t");
    if !value_map.is_empty() {
        for (key, value) in value_map {
            eprintln!("Unaccounted value {} -> {}", key, value)
        }
    }
    writer.write_fmt(format_args!("{}\n", line))?;
    Ok(())
}