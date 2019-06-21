use std::path::Path;
use std::fs::File;
use std::io::Cursor;

use mustache;
use mustache::MapBuilder;
use rocket::response::content::Xml;

use data::{AddToData, build_data};


// Regular pages

pub fn serve_file<P : AsRef<Path>>(path: P) -> Option<File> {
    File::open(path).ok()
}

pub fn render_xml<P : AsRef<Path>>(path: P, data: Vec<&AddToData>) -> Xml<String> {
    Xml(render_mustache(path, build_data(data)))
}

fn render_mustache<P : AsRef<Path>>(path: P, data: mustache::Data) -> String {
    let template = mustache::compile_path(path.as_ref()).unwrap_or_else(|err| {
        panic!("Failed to compile template '{:?}'\n{:?}", path.as_ref(), err)
    });
    let mut buff = Cursor::new(vec!());
    template.render_data(&mut buff, &data).unwrap_or_else(|err| {
        panic!("Failed to render template '{:?}'. {}", path.as_ref(), err);
    });
    String::from_utf8(buff.into_inner()).unwrap()
}

// Error Pages

pub fn render_error(title: &str, msg: &str) -> Xml<String> {
    render_xml("pages/site/error.xml", vec!(&ErrorPage::new(title, msg)))
}

struct ErrorPage<'a> {
    title: &'a str,
    message: &'a str
}

impl<'a> ErrorPage<'a> {
    fn new(title: &'a str, message: &'a str) -> ErrorPage<'a> {
        ErrorPage { title, message }
    }
}

impl<'a> AddToData for ErrorPage<'a> {
    fn add_to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("title", self.title)
            .insert_str("message", self.message)
    }
}

// Error Messages

pub fn error_msg<'a>(msg: &'a str) -> ErrorMessage<'a> {
    ErrorMessage {
        error_msg: msg
    }
}

pub struct ErrorMessage<'a> {
    error_msg: &'a str
}

impl<'a> AddToData for ErrorMessage<'a> {
    fn add_to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("error_msg", self.error_msg)
    }
}
