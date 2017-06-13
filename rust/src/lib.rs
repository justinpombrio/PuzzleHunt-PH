extern crate htmlescape;

use std::fmt;

use htmlescape::{encode_minimal, encode_attribute};


pub struct Page {
    hunt: Box<AsRef<str>>,
    body: Html
}

pub enum Html {
    Text(Box<AsRef<str>>)
}


impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hunt = encode_minimal((*self.hunt).as_ref());
        
        writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
        writeln!(f, "<?xml-stylesheet type=\"text/xsl\" href=\"/ph.xsl\"?>")?;
        
        writeln!(f, "<page>")?;
        writeln!(f, "  <hunt>{}</hunt>", hunt)?;
        writeln!(f, "  <content>{}</content>", self.body)?;
        writeln!(f, "</page>")
    }
}

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Html::Text(ref text) => {
                let text = (**text).as_ref();
                write!(f, "{}", encode_minimal(text))
            }
        }
    }
}


pub fn page<Text : AsRef<str> + 'static>(hunt: Text, body: Html) -> Page {
    Page{
        hunt: Box::new(hunt),
        body: body
    }
}

pub fn text<Text : AsRef<str> + 'static>(text: Text) -> Html {
    Html::Text(Box::new(text))
}
