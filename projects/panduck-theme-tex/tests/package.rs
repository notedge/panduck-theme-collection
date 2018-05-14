use panduck_theme_utility::pipeline::render_sass;
use notedown_ast::Result;

#[test]
fn test() -> Result<()> {
    let out = render_sass(include_str!("../static/html/light.sass"))?;
    println!("{}", out);
    Ok(())
}