use dast::run;

fn main() {
  match run() {
    Ok(_) => { }
    Err(error) => { eprintln!("{}", error)}
  };
}
