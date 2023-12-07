struct IntCode {}
trait Parser {
    fn parse(&self) -> bool;
}

impl Parser for IntCode {
    fn parse(&self) -> bool {}
}
