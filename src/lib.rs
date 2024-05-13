use pest::Parser;
use pest_derive::Parser;
mod colors;

#[derive(Parser)]
#[grammar = "tailwind.pest"]
pub struct TailwindParser;

pub fn parse_tailwind(input: &str) -> Result<Vec<&str>, Box<dyn std::error::Error>> {
    let mut style_list = TailwindParser::parse(Rule::style_list, input)?;
    let str_style_list = style_list
        .next()
        .unwrap()
        .into_inner()
        .map(|style| style.as_str())
        .collect::<Vec<&str>>();

    Ok(str_style_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_ast::parse_tailwind as parse_tailwind_ast;

    #[test]
    fn test_parse_naive_styles() {
        let input = "bg-red-500 text-white container";

        let output = parse_tailwind(input);

        assert!(output.is_ok());
        assert_eq!(
            output.unwrap(),
            vec!["bg-red-500", "text-white", "container"]
        );
    }

    #[test]
    fn test_tailwind_ast() {
        let input = "bg-red-500 text-white container";

        let output = parse_tailwind_ast(input);

        assert!(output.is_ok());
        assert_eq!(output.unwrap(), vec![]);
    }
}
