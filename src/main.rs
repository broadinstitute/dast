use tups::run;

fn main() {
  match run() {
    Ok(value) => {
      if !value.is_unit() {
        println!("{}", value)
      }
    }
    Err(error) => { eprintln!("{}", error)}
  };
}
