use std::path::Path;
use std::io::Cursor;
use std::convert::TryInto;

use mustache;
use mustache::MapBuilder;
use rocket::request::Request;
use rocket::response;
use rocket::response::content::Xml;
use rocket::response::{NamedFile, Redirect, Responder};
use rocket::http::uri::Uri;

use data::{AddToData, build_data};


pub fn redirect<U: TryInto<Uri<'static>>>(uri: U) -> Page {
    Page::Redirect(Redirect::to(uri))
}

pub fn xml<P : AsRef<Path>>(path: P, data: Vec<&AddToData>) -> Page {
    match render_mustache(path, build_data(data)) {
        Ok(xml) => Page::Xml(Xml(xml)),
        Err(msg) => Page::String(msg)
    }
}

pub fn error(title: &str, msg: &str) -> Page {
    xml("pages/site/error.xml", vec!(&ErrorPage::new(title, msg)))
}

pub fn file<P : AsRef<Path>>(path: P) -> Page {
    match NamedFile::open(path) {
        Ok(file) => Page::File(file),
        Err(_) => not_found("File not found. This is not a puzzle.")
    }
}

pub fn not_found(msg: &str) -> Page {
    error("404: Page Not Found", msg)
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
            .insert_str("error", self.error_msg)
    }
}

// Web Pages Enum

#[derive(Debug)]
pub enum Page {
    Redirect(Redirect),
    Xml(Xml<String>),
    File(NamedFile),
    String(String)
}

impl<'r> Responder<'r> for Page {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        match self {
            Page::Redirect(redirect) => redirect.respond_to(request),
            Page::Xml(xml) => xml.respond_to(request),
            Page::File(file) => file.respond_to(request),
            Page::String(s) => s.respond_to(request)
        }
    }
}


// Regular pages

fn render_mustache<P : AsRef<Path>>(path: P, data: mustache::Data) -> Result<String, String> {
    let template = match mustache::compile_path(path.as_ref()) {
        Ok(t) => t,
        Err(err) => return Err(format!("Failed to compile template '{:?}'\n{:?}", path.as_ref(), err))
    };
    let mut buff = Cursor::new(vec!());
    match template.render_data(&mut buff, &data) {
        Ok(()) => Ok(String::from_utf8(buff.into_inner()).unwrap_or("UTF8 rendering issue!".to_string())),
        Err(err) => Err(format!("Failed to render template '{:?}'. {}", path.as_ref(), err))
    }
}

// Error Pages

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
