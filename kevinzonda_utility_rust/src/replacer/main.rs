use std::{
    env::args,
    fmt::Display,
    io::{stdin, Read},
    process::exit,
};

#[derive(Debug)]
enum Error {
    UnknownArgument(String),
    WrongArgumentNumber { expect: usize, got: usize },
    IO(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownArgument(arg) => write!(f, "unknown argument: {arg}"),
            Error::WrongArgumentNumber { expect, got } => {
                write!(f, "argument expect: {expect}, got: {got}")
            }
            Error::IO(e) => write!(f, "io error: {e}"),
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let mut args = args().skip(1);
    let arg1 = args.next().unwrap_or_else(|| String::from("--help"));
    let args: Vec<String> = args.collect();
    let pipe_input: String;
    let (str, from, to) = match arg1.as_str() {
        "--stdin" | "--pipeline" | "-p" => {
            ensure_argc(&args, 2)?;
            pipe_input = read_from_pipe()?;
            (&pipe_input, &args[0], &args[1])
        }
        "--string" | "-s" => {
            ensure_argc(&args, 3)?;
            (&args[0], &args[1], &args[2])
        }
        "--help" | "-h" | "-?" => {
            let help_info = include_str!("./help_info.txt");
            eprintln!("{help_info}");
            return Ok(());
        }
        _ => return Err(Error::UnknownArgument(arg1)),
    };
    print!("{}", str.replace(from, to));
    Ok(())
}

fn read_from_pipe() -> Result<String, Error> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn ensure_argc(args: &[String], len: usize) -> Result<(), Error> {
    if args.len() != len {
        Err(Error::WrongArgumentNumber {
            expect: len,
            got: args.len(),
        })
    } else {
        Ok(())
    }
}
