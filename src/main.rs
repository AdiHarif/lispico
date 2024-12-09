use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mylisp.pest"]
struct MylispParser;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let programs = vec![
            "(a)",
            "(a b)",
            "(a b c)",
            "()",
            "(())",
            "((a))",
            "(a (b))",
            "(a (b c))",
        ];
        for program in programs {
            assert!(MylispParser::parse(Rule::program, program).is_ok());
        }

        let faulty_programs = vec!["(", ")", "(a", "a)", "(a b", "(a b c"];
        for program in faulty_programs {
            assert!(MylispParser::parse(Rule::program, program).is_err());
        }
    }
}
