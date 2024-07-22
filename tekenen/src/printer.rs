use std::fmt::{Display, Formatter};

pub trait Print {
    fn fmt(&self, printer: &mut Printer) -> std::fmt::Result;
}

impl<T: Display> Print for T {
    fn fmt(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.print(format!("{}", self).as_str())
    }
}

pub struct PrinterBuilder<'a> {
    source: &'a dyn Print,
}

impl<'a> Display for PrinterBuilder<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut printer = Printer {
            fmt: f,
            indentation: 0,
            indented: false,
            newlined: true,
            debug: false,
            previous: None,
        };
        self.source.fmt(&mut printer)
    }
}

pub struct Printer<'a, 'b> {
    fmt: &'b mut Formatter<'a>,
    indentation: i32,
    indented: bool,
    newlined: bool,
    debug: bool,
    
    previous: Option<String>,
}

impl<'a, 'b> Printer<'a, 'b> {
    pub fn new(source: &dyn Print) -> PrinterBuilder {
        PrinterBuilder { source }
    }

    fn print_indentation(&mut self) -> Result<(), std::fmt::Error> {
        if !self.indented {
            for _ in 0..self.indentation {
                self.fmt.write_str(" ")?;
            }

            self.indented = true;
            self.newlined = false;
        }
        Ok(())
    }

    pub fn print_newline(&mut self) -> Result<(), std::fmt::Error> {
        if !self.newlined {
            self.fmt.write_str("\n")?;

            self.newlined = true;
            self.indented = false;
        }
        Ok(())
    }

    pub fn print(&mut self, text: &str) -> Result<(), std::fmt::Error> {
        self.print_indentation()?;
        self.fmt.write_str(text)
    }

    pub fn println(&mut self, text: &str) -> Result<(), std::fmt::Error> {
        self.print(text)?;
        self.print_newline()
    }

    pub fn indent(&mut self, indentation: i32) {
        self.indentation += indentation;
    }

    pub fn value(&mut self, value: &dyn Print) -> Result<(), std::fmt::Error> {
        self.print_indentation()?;

        let indentation = self.indentation;
        value.fmt(self)?;
        self.indentation = indentation;

        self.print_newline()
    }

    pub fn property(&mut self, name: &str, value: &dyn Print) -> Result<(), std::fmt::Error> {
        self.print(name)?;
        self.fmt.write_str(": ")?;

        let indentation = self.indentation;
        value.fmt(self)?;
        self.indentation = indentation;

        self.print_newline()
    }

    pub fn set_previous(&mut self, previous: String) {
        self.previous = Some(previous);
    }

    pub fn print_previous(&mut self) -> Result<(), std::fmt::Error> {
        if let Some(previous) = self.previous.take() {
            self.print(previous.as_str())?;
            self.print_newline()?;
        }
        Ok(())
    }
}

/*

Output for FUI:

<DIV>
  position: [50, 20, 10, 10]
  children:
        <TEXT>
          position: [0, 0, 10, 10]
          text: "Hello, World!"


*/