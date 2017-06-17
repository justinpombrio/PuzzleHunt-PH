use std::path::Path;
use std::io::Cursor;

use mustache;

pub fn render_mustache<P : AsRef<Path>>(path: P, data: mustache::Data) -> String {
    let template = mustache::compile_path(path).unwrap();
    let mut buff = Cursor::new(vec!());
    template.render_data(&mut buff, &data).unwrap();
    String::from_utf8(buff.into_inner()).unwrap()
}
