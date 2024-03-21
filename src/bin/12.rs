use serde_json::{Error, Value};

fn main() -> Result<(), Error> {
    let input_buf = aoc_2015::aoc_io::get_input_as_reader(12);
    let input_json: Value = serde_json::from_reader(input_buf)?;

    let num_total = collect_json_numbers_all(&input_json);
    let num_nored = collect_json_numbers_nored(&input_json);
    aoc_2015::aoc_io::put_aoc_output((Some(num_total), Some(num_nored)));
    Ok(())
}

fn collect_json_numbers_all(json: &Value) -> i64 {
    match json {
        Value::Null | Value::Bool(_) | Value::String(_) => 0_i64,
        Value::Number(x) => x.as_i64().expect("Couldn't parse {x} as an i64"),
        Value::Array(xs) => xs
            .iter()
            .fold(0_i64, |acc, x| acc + collect_json_numbers_all(x)),
        Value::Object(xs) => xs
            .iter()
            .fold(0_i64, |acc, (_, x)| acc + collect_json_numbers_all(x)),
    }
}

fn collect_json_numbers_nored(json: &Value) -> i64 {
    match json {
        Value::Null | Value::Bool(_) | Value::String(_) => 0_i64,
        Value::Number(x) => x.as_i64().expect("Couldn't parse {x} as an i64"),
        Value::Array(xs) => xs
            .iter()
            .fold(0_i64, |acc, x| acc + collect_json_numbers_nored(x)),
        Value::Object(xs) if xs.iter().any(object_has_red) => 0,
        Value::Object(xs) => xs
            .iter()
            .fold(0_i64, |acc, (_, x)| acc + collect_json_numbers_nored(x)),
    }
}

fn object_has_red(ser_obj: (&String, &Value)) -> bool {
    let (_, json) = ser_obj;
    match json {
        Value::String(s) => s == "red",
        _ => false,
    }
}
