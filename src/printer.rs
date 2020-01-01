use console::Term;

pub struct Printer {
    term: Term,
    sticky_line: Option<String>
}

impl Printer {
    pub fn new() -> Printer {
        Printer { term: Term::stdout(), sticky_line: None }
    }

    pub fn print_line(&self, s: &str) {
        if let Some(sticky_line) = &self.sticky_line {
            self.term.clear_last_lines(1).unwrap();
            self.term.write_line(s).unwrap();
            self.term.write_line(sticky_line.as_str()).unwrap();
        }
        else {
            self.term.write_line(s).unwrap();
        }
    }

    pub fn print_sticky_line(&mut self, s: &str) {
        if self.sticky_line != None {
            self.term.clear_last_lines(1).unwrap();
        }
        self.term.write_line(s).unwrap();
        self.sticky_line = Some(s.to_string());
    }

    pub fn _clear_sticky_line(&mut self) {
        self.sticky_line = None;
    }
}