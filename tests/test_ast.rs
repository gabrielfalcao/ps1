use k9::assert_equal;
use ps1g::{Token, Variable};

#[test]
fn test_ansireset() {
    let token = Token::AnsiReset;
    assert_equal!(token.to_string(), "\\[\\033[0m\\]");
}
#[test]
fn test_color() {
    let token = Token::Color(237);
    assert_equal!(token.to_string(), "\\[\\033[1;38;5;237m\\]");
}
#[test]
fn test_bgcolor() {
    let token = Token::BgColor(237);
    assert_equal!(token.to_string(), "\\[\\033[1;48;5;237m\\]");
}
#[test]
fn test_vcsparam() {
    let token = Token::VcsParam("git".to_string(), "branch".to_string());
    assert_equal!(token.to_string(), "$(ps1 --resolve git:branch)");
}
#[test]
fn test_unhandled() {
    let token = Token::Unhandled("?-unhandled+!".to_string());
    assert_equal!(token.to_string(), "?-unhandled+!");
}
#[test]
fn test_variable_promptend() {
    let token = Token::Variable(Variable::PromptEnd);
    assert_equal!(token.to_string(), "\\$");
}

#[test]
fn test_variable() {
    assert_equal!(Token::Variable(Variable::Username).to_string(), "\\u");
}
