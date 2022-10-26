pub(crate) fn name() -> Option<&'static str> { option_env!("CARGO_PKG_NAME") }

pub(crate) fn version() -> Option<&'static str> { option_env!("CARGO_PKG_VERSION") }

pub(crate) fn description() -> Option<&'static str> { option_env!("CARGO_PKG_DESCRIPTION") }

pub(crate) fn authors() -> Option<&'static str> { option_env!("CARGO_PKG_AUTHORS") }

pub(crate) fn homepage() -> Option<&'static str> { option_env!("CARGO_PKG_HOMEPAGE") }

pub(crate) fn repository() -> Option<&'static str> { option_env!("CARGO_PKG_REPOSITORY") }

pub(crate) const USAGE: &str = "\
USAGE:
    tups script <script file>  - Execute script file
    tups eval <expression>     - Evaluate expression
    tups shell                 - Open shell
    tups version               - Print version
    tups help                  - Print help";

fn push_line(string: &mut String, suffix: &str) {
    string.push_str(suffix);
    string.push('\n');
}

pub(crate) fn name_and_version() -> Option<String> {
    match (name(), version()) {
        (Some(name), Some(version)) => { Some(format!("{} {}", name, version)) }
        (Some(name), None) => { Some(name.to_string()) }
        (None, Some(version)) => { Some(version.to_string()) }
        (None, None) => { None }
    }
}

pub(crate) fn about() -> String {
    let mut intro = String::new();
    if let Some(name_ad_version) = name_and_version() {
        push_line(&mut intro, &name_ad_version)
    }
    if let Some(description) = description() {
        push_line(&mut intro, description)
    }
    if let Some(authors) = authors() { push_line(&mut intro , authors) }
    intro.push('\n');
    push_line(&mut intro, USAGE);
    if let Some(homepage) = homepage() { push_line(&mut intro, homepage) }
    if let Some(repository) = repository() { push_line(&mut intro, repository) }
    intro
}

/*
Clap: tups 0.0.4
Oliver Ruebenacker <oliverr@broadinstitute.org>
Tups, the tuple stream transformer

USAGE:
    tups <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    eval      Evaluate expression
    help      Print this message or the help of the given subcommand(s)
    script    Execute script
    shell     Run shell
 */