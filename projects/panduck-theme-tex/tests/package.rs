use std::path::PathBuf;
use panduck_theme_utility::pipeline::{render_sass, render_sass_path};
use notedown_ast::Result;

#[test]
fn test() -> Result<()> {
    let out = render_sass_path(
        PathBuf::from("../../static/html/light.sass").as_path(),
        &vec![
            PathBuf::from("../static/html/light.sass")
        ]
    )?;
    println!("{}", out);
    Ok(())
}