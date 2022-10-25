use tups::run;

fn main() {
  match run() {
    Ok(value) => { println!("{}", value)}
    Err(error) => { eprintln!("{}", error)}
  };
}
