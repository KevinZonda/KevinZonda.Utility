use std::{
    env::args,
    fmt::Display,
    io::{stdin, Read, Write},
    process::exit,
};

#[derive(Debug)]
enum Error {
    UnknownArgument(String),
    WrongArgumentNumber { expect: usize, got: usize },
    IO(std::io::Error),
    Msg { from: String, msg: String },
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
            Error::Msg {from, msg } => write!(f, "{from} error: {msg}"),
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
    let mut read_path: Option<String> = None;
    let mut write_path: Option<String> = None;
    let mut is_to_file = false;
    let empty_string = "".to_string();
    let (mut str, from, to) = match arg1.as_str() {
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
        },
        "--file" | "-f" => {
            ensure_argc(&args, 3)?;
            is_to_file = true;
            read_path = Some(args[0].clone());
            write_path = Some(args[3].clone());
            (&empty_string, &args[1], &args[2])
        }
        _ => {
            match args.len() {
                1 => {
                    pipe_input = read_from_pipe()?;
                    (&pipe_input, &arg1, &args[0])
                },
                2 => (&arg1, &args[0], &args[1]),
                _ => return Err(Error::UnknownArgument(arg1))
            }
        }
    };
    if is_to_file {
        str = match read_from_file(read_path.unwrap()) {
            Ok(x) => &x,
            Err(e) => return Err(
                Error::Msg{
                    from:  "IO".to_string(),
                    msg: format!("{}", e)
                }),
        };
    }
    let replaced = str.replace(from, to);
    if !is_to_file {
        println!("{}", replaced);
    } else {
        write_to_file(write_path.unwrap(), replaced)?;
    }
    Ok(())
}

fn read_from_pipe() -> Result<String, Error> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn read_from_file(path: String) -> Result<String, Error> {
    let mut buf = String::new();
    std::fs::File::open(path)?.read_to_string(&mut buf)?;
    Ok(buf)
}

fn write_to_file(path: String, content: String) -> Result<(), Error> {
    std::fs::File::create(path)?.write_all(content.as_bytes())?;
    Ok(())
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
