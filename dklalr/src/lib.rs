use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

mod ast;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    macro_rules! test_value_variant {
        ($parser:expr, $variant:path, $value:expr, $expected:expr) => {
            if let $variant(actual) = $parser.parse($value).unwrap() {
                assert_eq!(actual, $expected);
            } else { panic!("Unexpected variant found") }
        }
    }

    #[test]
    fn test_arg_tail() {
        let p = parser::ArgTailParser::new();

        let n = p.parse("= foo").unwrap();
        assert!(n.0.is_some());

        let n = p.parse("").unwrap();
        assert!(n.0.is_none());
    }

    #[test]
    fn test_value_ident() {
        let parser = parser::ValueParser::new();

        test_value_variant!(parser, ValueNode::Ident, "anident", "anident");
        test_value_variant!(parser, ValueNode::Ident, "anIdent", "anIdent");
        test_value_variant!(parser, ValueNode::Ident, "_anident", "_anident");
        test_value_variant!(parser, ValueNode::Ident, "_9ident", "_9ident");
        test_value_variant!(parser, ValueNode::Ident, "anIdent", "anIdent");
        test_value_variant!(parser, ValueNode::Ident, "AnIdent", "AnIdent");
        test_value_variant!(parser, ValueNode::Ident, "IDENT", "IDENT");

        assert!(parser.parse("1ident").is_err());
    }

    #[test]
    fn test_value_string() {
        let parser = parser::ValueParser::new();

        test_value_variant!(parser, ValueNode::String, r#""foo""#, "foo");
    }

    #[test]
    fn test_value_number() {
        let parser = parser::ValueParser::new();

        test_value_variant!(parser, ValueNode::Number, "23", 23);
        test_value_variant!(parser, ValueNode::Number, "0", 0);
        test_value_variant!(parser, ValueNode::Number, "-15", -15);
    }

    #[test]
    fn test_value_bool() {
        let parser = parser::ValueParser::new();

        test_value_variant!(parser, ValueNode::Bool, "true", true);
        test_value_variant!(parser, ValueNode::Bool, "false", false);

        // These will get parsed as idents, not bools.
        test_value_variant!(parser, ValueNode::Ident, "True", "True");
        test_value_variant!(parser, ValueNode::Ident, "FALSE", "FALSE");
    }

    #[test]
    fn test_bool() {
        let parser = parser::BoolParser::new();

        let BoolNode(value) = parser.parse("true").unwrap();
        assert_eq!(value, true);

        let BoolNode(value) = parser.parse("false").unwrap();
        assert_eq!(value, false);

        assert!(parser.parse("True").is_err());
     }
}
