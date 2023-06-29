use std::fs::File;
use std::io::BufReader;
use crate::data::io::tsv::TsvReader;
use crate::data::var_id::VarId;
use crate::error::{Error, map_err};
use crate::lang::value::Value;
use std::io::Write;
use statrs::function::erf::erfc;
use crate::data::io::line_parser::LineParser;

struct Record {
    var_id: VarId,
    endo_z: f64,
}

fn z_to_p(z: f64) -> f64 {
    erfc(z.abs() / 2.0_f64.sqrt())
}

pub(crate) fn phenet(input: &str, output: &str, z_threshold: f64) -> Result<Value, Error> {
    let tsv_reader =
        TsvReader::new(BufReader::new(
            map_err(File::open(input), input)?
        ), LineParser::new_tsv())?;
    let var_id_col =
        tsv_reader.header.first()
            .ok_or_else(|| Error::from("Input file header line is empty"))?.to_string();
    let mut records: Vec<Record> = Vec::new();
    for row_res in tsv_reader {
        let row = row_res?;
        let mut field_iter = row.iter();
        let var_id =
            VarId::parse(field_iter.next()
                .ok_or_else(|| { Error::from("Empty row") })?)?;
        let endo_z =
            field_iter.next()
                .ok_or_else(|| Error::from(format!("Missing Z after {}", var_id)))?
                .parse::<f64>().ok().unwrap_or(f64::NAN);
        if endo_z.abs() >= z_threshold {
            records.push(Record { var_id, endo_z });
        }
    }
    records.sort_by(|r1, r2| r1.var_id.cmp(&r2.var_id));
    let mut out_file = map_err(File::create(output), output)?;
    writeln!(out_file, "#i\t{}\tchrom\tpos\tref\talt\tz_endo\tabs(z_endo)\tp_endo", var_id_col)?;
    for (i, record) in records.iter().enumerate() {
        writeln!(out_file, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", i, record.var_id, record.var_id.chrom,
                 record.var_id.pos, record.var_id.seq_ref, record.var_id.seq_alt, record.endo_z,
                 record.endo_z.abs(), z_to_p(record.endo_z))?;
    }
    Ok(Value::Unit)
}