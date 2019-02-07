use std::path::Path;
use std::io::Cursor;

use mustache;

pub fn render_mustache<P : AsRef<Path>>(path: P, data: mustache::Data) -> String {
    let template = mustache::compile_path(path.as_ref()).unwrap_or_else(|err| {
        panic!("Failed to compile template '{:?}'\n{:?}", path.as_ref(), err)
    });
    let mut buff = Cursor::new(vec!());
    template.render_data(&mut buff, &data).unwrap_or_else(|err| {
        panic!("Failed to render template '{:?}'. {}", path.as_ref(), err);
    });
    String::from_utf8(buff.into_inner()).unwrap()
}
