extern crate ph;
#[macro_use]
extern crate nickel;

use std::collections::HashMap;

use ph::*;

use nickel::{Nickel, HttpRouter, MediaType};

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
        let the_page = page(
            "The <b>BEST</b> hunt",
            text("The <i>BEST</i> body".to_string()));
        response.set(MediaType::Xml);
        return response.send(format!("{}", the_page));
    }});

    match server.listen("localhost:3000") {
        Ok(_) => (),
        Err(err) => panic!("Failed to start server: {}", err)
    }
}
