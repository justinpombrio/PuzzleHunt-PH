extern crate ph;
#[macro_use]
extern crate nickel;
extern crate mustache;

use std::path::Path;
use std::io::Cursor;

use std::collections::HashMap;

use ph::*;

use nickel::{Nickel, HttpRouter, MediaType};

fn render<P : AsRef<Path>>(path: P,
                           data: mustache::Data,
                           response: &mut nickel::Response) -> Vec<u8> {
    response.set(MediaType::Xml);
    let template = mustache::compile_path(path).unwrap();
    let mut buff = Cursor::new(vec!());
    template.render_data(&mut buff, &data).unwrap();
    buff.into_inner()
}

fn main() {
    let mut server = Nickel::new();

    server.get("/css/**", middleware! { |request, response| {
        println!("Serving {}", request.path_without_query().unwrap());
        let path = &request.path_without_query().unwrap()[1..];
        return match response.send_file(path) {
            Ok(x) => Ok(x),
            Err(err) => {
                println!("Boom! {}", err.message);
                Err(err)
            }
        }
    }});

    server.get("/ph.xsl", middleware! { |_, mut response| {
        println!("Serving ph.xsl");
        response.set(MediaType::Xslt);
        return response.send_file("ph.xsl");
    }});
    
    server.get("/index.xml", middleware! { |_, mut response| {
        println!("Serving index.xml");
        let data = mustache::MapBuilder::new()
            .insert_str("hunt", "The <b>best</b> hunt")
            .build();
        render("index.xml", data, &mut response)
    }});

    match server.listen("localhost:3000") {
        Ok(_) => (),
        Err(err) => panic!("Failed to start server: {}", err)
    }
}
