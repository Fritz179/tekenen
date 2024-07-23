use std::{cell::{Cell, RefCell}, fmt::{Debug, Display, Formatter}};

pub trait Print {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result;
}

impl Print for bool {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.print_string(if *self { "true" } else { "false" })
    }
}

impl Print for String {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.print_string(self)
    }
}

impl Print for &str {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.print_string(self)
    }
}

impl<T: Print + Copy> Print for Cell<T> {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.println(&self.get())
    }
}

impl<T: Print> Print for RefCell<T> {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.println(&*self.borrow())
    }
}

pub struct PrinterBuilder<'source> {
    source: &'source dyn Print,
}

impl<'source> PrinterBuilder<'source> {
    fn build<'inner, 'outer>(&self, f: &'outer mut Formatter<'inner>, debug: bool) -> Printer<'inner, 'outer> {
        Printer {
            fmt: f,
            indentation: 0,
            indented: false,
            newlined: true,
            debug,
            blocked: false,
            previous: None,
        }
    }
}

impl<'source> Display for PrinterBuilder<'source> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut printer = self.build(f, false);
        self.source.print(&mut printer)
    }
}

impl<'source> Debug for PrinterBuilder<'source> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut printer = self.build(f, true);
        self.source.print(&mut printer)
    }
}

pub struct Printer<'inner, 'outer> {
    fmt: &'outer mut Formatter<'inner>,
    indentation: i32,
    indented: bool,
    newlined: bool,

    debug: bool,
    blocked: bool,

    previous: Option<String>,
}

impl<'inner, 'outer> Printer<'inner, 'outer> {
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

    fn print_newline(&mut self) -> Result<(), std::fmt::Error> {
        if !self.newlined {
            self.fmt.write_str("\n")?;

            self.newlined = true;
            self.indented = false;
        }
        Ok(())
    }

    fn print_string(&mut self, text: &str) -> Result<(), std::fmt::Error> {
        self.print_indentation()?;
        self.fmt.write_str(text)
    }

    pub fn print_debug(&mut self, debug: &dyn Debug) -> std::fmt::Result {
        self.print(&format!("{debug:?}"))
    }

    fn blocked(&mut self) -> bool {
        if self.blocked {
            self.blocked = false;
            true
        } else {
            false
        }
    }

    pub fn debug(&mut self) -> &mut Self {
        self.blocked = !self.debug;
        self
    }

    pub fn indent(&mut self, indentation: i32) {
        self.indentation += indentation;
    }

    pub fn print(&mut self, value: &dyn Print) -> Result<(), std::fmt::Error> {
        if self.blocked() {
            return Ok(())
        }

        self.print_indentation()?;

        let indentation = self.indentation;
        value.print(self)?;
        self.indentation = indentation;

        Ok(())
    }

    pub fn println(&mut self, value: &dyn Print) -> Result<(), std::fmt::Error> {
        if self.blocked() {
            return Ok(())
        }

        self.print_indentation()?;

        let indentation = self.indentation;
        value.print(self)?;
        self.indentation = indentation;

        self.print_newline()
    }

    pub fn property(&mut self, name: &str, value: &dyn Print) -> Result<(), std::fmt::Error> {
        if self.blocked() {
            return Ok(())
        }

        self.print_string(name)?;
        self.fmt.write_str(": ")?;

        self.println(value)
    }

    pub fn set_previous(&mut self, previous: String) {
        self.previous = Some(previous);
    }

    pub fn print_previous(&mut self) -> Result<(), std::fmt::Error> {
        if self.blocked() {
            return Ok(())
        }

        if let Some(previous) = self.previous.take() {
            self.print_string(previous.as_str())?;
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