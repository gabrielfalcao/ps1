use k9::assert_equal;
use ps1g::Config;

#[test]
fn test_config_render_unknown_chars() {
    let config = Config::new("{git:branch}\\${reset}:");
    assert_equal!(
        config.render_ps1().unwrap(),
        "$(ps1 --resolve git:branch)\\$\\[\\033[0m\\]:"
    );

    let config = Config::new("{220}\\u{231}:{220}\\W{37}{git:branch}{220}\\${reset}");
    assert_equal!(
        config.render_ps1().unwrap(),
        "\\[\\033[1;38;5;220m\\]\\u\\[\\033[1;38;5;231m\\]:\\[\\033[1;38;5;220m\\]\\w\\[\\033[1;38;5;37m\\]$(ps1 --resolve git:branch)\\[\\033[1;38;5;220m\\]\\$\\[\\033[0m\\]"

    );
}

#[test]
fn test_config_render_syntax_error() {
    let config = Config::new(
"\\[\\033[1;38;5;136m\\]\\u\\[\\033[1;38;5;31m\\]@\\[\\033[1;38;5;214m\\]\\w\\[\\033[1;38;5;220m\\]\\[\\033[1;38;5;67m\\]$(ps1 --resolve git:branch)\\[\\033[1;38;5;220m\\]$(ps1 --resolve prompt:end)\\[\\033[0m\\] ",
    );
    assert_equal!(
        config.render_ps1().unwrap(),
        "\\[\\033[1;38;5;79m\\]\\u@\\W\\$\\[\\033[0m\\]"
    );
}
#[test]
fn test_config_render_parse_error() {
    let config = Config::new("{foo}$");
    assert_equal!(config.render_ps1().unwrap(), "{foo}\\$");
}
#[test]
fn test_config_render_no_tokens() {
    let config = Config::new("\\u@\\w\\$");
    assert_equal!(config.render_ps1().unwrap(), "\\u@\\W\\$");
}

#[test]
fn test_config_render_color_token() {
    let config = Config::new("{79}\\u@\\w${reset}");
    assert_equal!(
        config.render_ps1().unwrap(),
        "\\[\\033[1;38;5;79m\\]\\u@\\W\\$\\[\\033[0m\\]"
    );
}
