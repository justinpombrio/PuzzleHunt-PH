use std::collections::HashMap;
use rocket::request::{FromForm, FormItems};

pub struct ExpandableFormToForm<F: FromExpandableForm>(pub F);

impl<'f, F: FromExpandableForm> FromForm<'f> for ExpandableFormToForm<F> {
    type Error = String;
    fn from_form(iter: &mut FormItems<'f>, strict: bool)
                 -> Result<ExpandableFormToForm<F>, String>
    {
        Ok(ExpandableFormToForm(F::from_form(iter, strict)?))
    }
}

pub trait FromExpandableForm : Sized {
    type Member;
    fn parts() -> Vec<&'static str>;
    fn member_parts() -> Vec<&'static str>;
    fn new_member(map: &HashMap<&str, &str>) -> Self::Member;
    fn new(map: &HashMap<&str, &str>, members: Vec<Self::Member>) -> Self;

    fn from_form(iter: &mut FormItems, strict: bool) -> Result<Self, String> {
        from_form(iter, strict)
    }
}

fn from_form<F: FromExpandableForm>(iter: &mut FormItems, strict: bool) -> Result<F, String> {
    if !strict { return Err("Not strict".to_string()); }

    let parts = F::parts();
    let member_parts = F::member_parts();
    let last_member_part = member_parts.last()
        .expect("must have at least one member part");

    let mut first = true;
    let mut map = HashMap::new();
    let mut members = vec!();
    let mut member_map = HashMap::new();
    for (key, value) in iter.map(|f| (f.key.as_str(), f.value.as_str())) {
        if parts.contains(&key) {
            map.insert(key, value);
        } else if member_parts.contains(&key) {
            member_map.insert(key, value);
            if &key == last_member_part {
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
    Ok(F::new(&map, members))
}
