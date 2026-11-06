use std::fs::File;
use std::io::{
    self,
    BufWriter,
    Write,
};

pub struct Out{
    stdout: bool,
    w: Box<dyn Write>,
}

impl Out {
    pub fn new(path: &str) -> anyhow::Result<Out> {
        let w: Box<dyn Write> = if path == "stdout" {
            Box::new(io::stdout())
        } else {
            Box::new(BufWriter::new(File::create(path)?))
        };
        if path == "stdout" {
            Ok(Out{
                stdout: true,
                w,
            })
        } else {
            Ok(Out{
                stdout: false,
                w,
            })
        }
    }

    pub fn write_ln(&mut self, ln: String) -> io::Result<()> {
        writeln!(self.w, "{ln}")
    }

    pub fn is_file(&self) -> bool {
        !self.stdout
    }
}

