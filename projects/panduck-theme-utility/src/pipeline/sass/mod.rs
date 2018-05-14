use std::path::{Path, PathBuf};
use notedown_ast::Result;
use rsass::{compile_scss, compile_scss_path, Error, FileContext, FsFileContext, parse_scss_data, parse_scss_file, parse_scss_path, ScopeRef};
use rsass::output::{Format, Style};


// fn new() -> std::result::Result<Vec<u8>, Error> {
//     let items = parse_scss_path(&mut input.as_bytes(), "input.scss")?;
//     let format = Format {
//         style: Style::Compressed,
//         precision: 0,
//     };
//     format.write_root(
//         &items,
//         ScopeRef::new_global(format),
//         &self.file_context,
//     )
// }

pub fn render_sass(sass: &str, imports: &[PathBuf]) -> Result<String> {
    let format = Format {
        style: Style::Expanded,
        precision: 0,
    };
    let mut fs = FsFileContext::new();
    for file in imports {
        fs.push_path()
    }
    let items = parse_scss_data(input)?;
    let css =format.write_root(&items, ScopeRef::new_global(format), &fs)?;

    unsafe {
        Ok(String::from_utf8_unchecked(css))
    }
}


pub fn render_sass_path(sass: &Path) -> Result<String> {
    let format = Format {
        style: Style::Compressed,
        precision: 5,
    };
    let css = compile_scss_path(sass, format)?;
    unsafe {
        Ok(String::from_utf8_unchecked(css))
    }
}
