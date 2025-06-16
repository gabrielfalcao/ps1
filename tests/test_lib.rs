use k9::assert_equal;
use ps1g::{parse_tokens, Result, Token, Variable};

#[test]
fn test_config_parse_no_tokens() -> Result<()> {
    let tokens = parse_tokens(r"\u@\w$").unwrap();
    assert_equal!(
        tokens,
        vec![
            Token::Variable(Variable::Username),
            Token::Unhandled("@".to_string()),
            Token::Variable(Variable::PwdShort),
            Token::Variable(Variable::PromptEnd),
        ]
    );
    Ok(())
}

#[test]
fn test_config_parse_color_token() -> Result<()> {
    let tokens = parse_tokens(r"{79}\u @ \w \${reset}").unwrap();
    assert_equal!(
        tokens,
        vec![
            Token::Color(79,),
            Token::Variable(Variable::Username),
            Token::Unhandled(" @ ".to_string()),
            Token::Variable(Variable::PwdShort),
            Token::Unhandled(" ".to_string()),
            Token::Variable(Variable::PromptEnd),
            Token::AnsiReset,
        ]
    );

    Ok(())
}
#[test]
fn test_config_parse_variable() -> Result<()> {
    let tokens = parse_tokens(r"\u@").unwrap();
    assert_equal!(
        tokens,
        vec![
            Token::Variable(Variable::Username),
            Token::Unhandled("@".to_string()),
        ]
    );

    Ok(())
}
