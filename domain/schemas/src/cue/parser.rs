use pest::Parser;

#[derive(Parser)]
#[grammar = "cue/cue.pest"]
pub struct CUEParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers() {
        let identifiers = vec!["a", "_x9", "fieldName", "αβ"];

        for identifier in identifiers.iter() {
            let result = CUEParser::parse(Rule::identifier, identifier);
            println!("{:?}", result);
            assert!(result.is_ok());
        }
    }
}
