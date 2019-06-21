use std::collections::HashMap;
use rocket::request::{FromForm, FormItems, FormItem};


pub trait ExpandableForm : Sized {
    type Member;
    fn parts() -> Vec<&'static str>;
    fn member_parts() -> Vec<&'static str>;
    fn new_member(map: &HashMap<String, String>) -> Self::Member;
    fn new(map: &HashMap<String, String>, members: Vec<Self::Member>) -> Self;
}

pub trait RegularForm : Sized {
    fn parts() -> Vec<&'static str>;
    fn new(map: &HashMap<String, String>) -> Result<Self, String>;
    
    fn from_form<'f>(iter: &mut FormItems<'f>, strict: bool) -> Result<Self, String> {
        if !strict { return Err(format!("Internal error: form submission was not strict.")); }
        let parts = Self::parts();
        let mut map = HashMap::new();
        for item in iter {
            let (key, value) = key_and_value(item)?;
            if parts.contains(&key.as_str()) {
                map.insert(key, value);
            } else {
                return Err(format!("Unrecognized key: {}.", key));
            }
        }
        Self::new(&map)
    }
}

pub enum RegularFormResult<F: RegularForm> {
    Ok(F),
    Err(String)
}

pub struct ExpandableFormToForm<F: ExpandableForm>(pub F);

impl<'f, F: RegularForm> FromForm<'f> for RegularFormResult<F> {
    type Error = ();
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<RegularFormResult<F>, ()>
    {
        match F::from_form(iter, strict) {
            Ok(form) => Ok(RegularFormResult::Ok(form)),
            Err(err) => Ok(RegularFormResult::Err(err))
        }
    }
}

impl<'f, F: ExpandableForm> FromForm<'f> for ExpandableFormToForm<F> {
    type Error = String;
    fn from_form(iter: &mut FormItems<'f>, strict: bool)
                 -> Result<ExpandableFormToForm<F>, String>
    {
        if !strict { return Err("Not strict".to_string()); }

        let parts = F::parts();
        let member_parts = F::member_parts();
        let last_member_part = match member_parts.last() {
            Some(part) => part,
            None => return Err("Failed to parse form.".to_string())
        };

        let mut first = true;
        let mut map = HashMap::new();
        let mut members = vec!();
        let mut member_map = HashMap::new();
        for item in iter {
            let (key, value) = key_and_value(item)?;
            if parts.contains(&key.as_str()) {
                map.insert(key, value);
            } else if member_parts.contains(&key.as_str()) {
                let is_last = &key == last_member_part;
                member_map.insert(key, value);
                if is_last {
                    if first {
                        // The first member is fake (b.c. expandable form)
                        first = false;
                    } else {
                        members.push(F::new_member(&member_map))
                    }
                    member_map = HashMap::new();
                }
            } else {
                return Err(format!("Unrecognized key: {}", key))
            }
        }
        Ok(ExpandableFormToForm(F::new(&map, members)))
    }
}

fn key_and_value(f: FormItem) -> Result<(String, String), String> {
    let key = match f.key.url_decode() {
        Ok(k) => k,
        Err(_) => return Err("Could not decode form key".to_string())
    };
    let value = match f.value.url_decode() {
        Ok(v) => v,
        Err(_) => return Err("Could not decode form value".to_string())
    };
    Ok((key, value))
}
