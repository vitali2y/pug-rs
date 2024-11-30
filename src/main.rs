use pug::parse;
use std::{
    fs,
    io::{self, Read},
};

fn main() {
    // reading the Pug input from stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    // defining the include resolver function
    let include_resolver = |path: String| -> Result<pug::Ast, io::Error> {
        // reading the content of the included file
        let content = fs::read_to_string(format!("./assets/{}", path))?;
        // parsing the included file into an AST
        parse(content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    };

    // parsing the input Pug content
    let mut ast = parse(buffer).unwrap();

    // expanding all includes in the AST
    ast = ast.expand(include_resolver).unwrap();

    // converting the expanded AST to HTML and write it to stdout
    ast.to_html(&mut io::stdout()).unwrap();
}
