use std::{
    error,
    fs::{self, File},
    io::{self, Error, Read},
};

// Box<dyn Error> to mean “any kind of error. More in chapter 17 trait objects
fn main() -> Result<(), Box<dyn error::Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}

fn main2() {
    //panic!("crash and burn")
    let path = "hello.txt";
    // let f = open_file(path);
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {}", error),
    // };
    // matching on kind of error
    let f = open_file(path);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create(path) {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {}", e),
            },
            others => panic!("Problem opening the file: {:?}", others),
        },
    };

    // simplify the above statement using closures (chapter 13)
    let f2 = open_file(path).unwrap_or_else(|_err| {
        File::create(path).unwrap_or_else(|err| panic!("Problem creating the file: {}", err))
    });

    // unwrap will panic if the result is of type Err
    let f2 = open_file(path).unwrap();
}

fn open_file(file: &str) -> Result<File, Error> {
    File::open(file)
}

fn read_content_from_file(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_content_from_file_v2(path: &str) -> Result<String, io::Error> {
    // ? will return the error of the function, if the Result is not of type Ok
    // It can also be used on Options
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_content_from_file_v3(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
