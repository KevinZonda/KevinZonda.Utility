use std::io::{Read, stdin};
use std::{env, process};

fn main() {
    let args = get_args();
    let arg1 = if args.len() == 0 { "--help" } else {
        args[0].as_str()
    };
    let mut str = "".to_string();
    let mut pre_str = "";
    let mut after_str = "";
    match arg1 {
        "--stdin" | "--pipeline" | "-p" => {
            ensure_eq(args.len() - 1, 2);
            str = read_from_pipe();
            pre_str = &*args[1];
            after_str = &*args[2];
        }
        "--string" | "-s" => {
            ensure_eq(args.len() - 1, 3);
            str = args[1].to_string();
            pre_str = &*args[2];
            after_str = &*args[3];
        }
        "--help" | "-h" | "--?" => {
            println!("KevinZonda.Utility.Rust.Replacer");
            println!("  --help | -h | --?");
            println!("    Get Help");
            println!("  --stdin | --pipeline | -p <from> <to>");
            println!("    Replace with pipeline");
            println!("  --string | -s <text> <from> <to>");
            println!("    Replace with string");
            exit(true);
        }
        _ => {
            panic("error: non-recognisable argument ".to_string() + arg1);
        }
    };
    str = str.replace(pre_str, after_str);
    print!("{}", str)
    // match io::stdout().write_all(str.as_ref()) {
    //     Ok(_) => {}
    //     Err(err) => {
    //         panic(format!("error: {}", err));
    //     }
    // }
}

fn ensure_eq(arg_len: usize, len: usize) {
    if arg_len != len {
        panic(format!("error: argument required: {}, got {}", len, arg_len))
    }
}


fn read_from_pipe() -> String {
    let mut result = String::new();
    match stdin().read_to_string(&mut result) {
        Ok(_) => {}
        Err(error) => {
            panic("error: ".to_string() + error.to_string().as_str());
        }
    }
    result
}

fn exit(is_normal_exit: bool) {
    process::exit(if is_normal_exit { 0 } else { 1 });
}

fn panic(msg: String) {
    eprintln!("{}", msg);
    exit(false)
}

fn get_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    args
}