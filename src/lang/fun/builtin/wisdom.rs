use std::fmt::{Display, Formatter};
use jati::trees::symbols::ArgsFailure;
use jati::trees::types::Type;
use rand::thread_rng;
use crate::lang::fun::builtin::Gen;
use crate::lang::fun::Fun;
use crate::Value;
use crate::lang::fun::util::check_n_args;
use crate::lang::runtime::Runtime;
use rand::seq::SliceRandom;
use crate::error::Error;

pub(crate) struct Wisdom {}

impl Gen for Wisdom {
    fn new() -> Wisdom { Wisdom {} }
}

struct Quote {
    content: &'static str,
    author: &'static str,
}

impl Quote {
    const fn new(content: &'static str, author: &'static str) -> Quote { Quote { content, author } }
}

const QUOTES: [Quote; 10] = [
    Quote::new("Life is what happens when you're busy making other plans.",
               "John Lennon"),
    Quote::new("The greatest glory in living lies not in never falling, but in rising every time we fall.",
               "Nelson Mandela"),
    Quote::new("Perfection is achieved, not when there is nothing more to add, but when there is nothing left to take away.",
               " Antoine de Saint-Exupéry"),
    Quote::new("The old man lost his horse, how to know it is not good luck?",
               "Chinese proverb"),
    Quote::new("The wise speak only of what they know.",
               "Gandalf"),
    Quote::new("God, grant me the serenity to accept the things I cannot change,\n\
               courage to change the things I can,\n\
               and wisdom to know the difference.",
               "Reinhold Niebuhr"),
    Quote::new("You can fool some of the people all of the time, and all of the people \
    some of the time, but you can not fool all of the people all of the time.",
               "Abraham Lincoln"),
    Quote::new("There are only two ways to live your life. One is as though nothing is a \
    miracle. The other is as though everything is a miracle.",
               "Albert Einstein"),
    Quote::new("Love is the attempt to form a friendship inspired by beauty.",
               "Marcus Tullius Cicero"),
    Quote::new("I don't believe in collective guilt, but I do believe in collective responsibility.",
               "Audrey Hepburn"),
];

//
impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" - {}", self.content, self.author)
    }
}

impl Fun for Wisdom {
    fn tpe(&self) -> Type { Type::String }
    fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure> {
        check_n_args(arg_types, 0)
    }
    fn call(&self, _args: Vec<Value>, _runtime: &mut Runtime) -> Result<Value, Error> {
        let mut rng = thread_rng();
        let message = format!("{}", QUOTES.choose(&mut rng).unwrap());
        Ok(Value::String(message))
    }
}

