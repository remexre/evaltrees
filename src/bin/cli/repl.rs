use evaltrees::ast::Decl;
use failure::Error;
use linefeed::Interface;

pub fn run(decls: Vec<Decl>) -> Result<(), Error> {
    let mut reader = Interface::new("evaltrees")?;
    reader.set_prompt("> ");
    bail!("{:#?}", decls)
}
