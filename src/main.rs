use clap::Parser;
use latex::PreambleElement;
use latex::{Document, DocumentClass, Element, Section};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    // Author name in the document
    #[arg(short, long)]
    author: String,
    // Title of the document
    #[arg(short, long)]
    title: String,
    // File path to preamble .tex file, Used to set packages used and other.
    #[arg(short, long)]
    preamblefile: String,
    #[arg(short, long)]
    exercisefile: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let author = args.author;
    let title = args.title;
    let pkg = args.preamblefile.to_string();
    let ex = File::open("exercises.txt")?;
    let filename = format!("{}-{}.tex",author,title);

    let mut doc = Document::new(DocumentClass::Article);


    let reader = BufReader::new(ex);

    //Document init:

    doc.preamble.title(&title);
    doc.preamble.author(&author);
    doc.preamble.push(PreambleElement::UserDefined(
        "\\include{".to_owned() + &pkg + "}",
    ));

    doc.push(Element::TitlePage);

    let mut imgc = 0;

    for line in reader.lines() {
        imgc+=1;
        let mut section = Section::new(&line.unwrap());
        let imgp = r"\includegraphics{".to_owned() +  &imgc.to_string() + "}";
        section.push(&*imgp);
        doc.push(section);
    }

    let prepd = latex::print(&doc)?;

    let mut f = File::create(&filename)?;
    write!(f, "{}", prepd)?;

    // Then call latexmk on it
    let compile_status = Command::new("latexmk")
        .arg("-pdf")
        .arg("-quiet")
        .arg(&filename)
        .status()?;

    let cleanup = Command::new("latexmk")
        .arg("-c")
        .arg("-quiet")
        .arg(&filename)
        .status()?;

    assert!(compile_status.success());
    assert!(cleanup.success());
    Ok(())
}
