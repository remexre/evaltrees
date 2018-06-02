use evaltrees::ast::{Decl, Type};
use failure::Error;
use linefeed::Interface;

pub fn run(decls: Vec<Decl<Type>>) -> Result<(), Error> {
    let mut reader = Interface::new("evaltrees")?;
    reader.set_prompt("> ");
    bail!("{:#?}", decls)
}
