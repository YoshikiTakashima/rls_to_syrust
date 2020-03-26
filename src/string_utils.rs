extern crate lazy_static;
use lazy_static::*;

extern crate regex;
use regex::Regex;
use std::string::String;

/* PATH RESOLUTION LIBRARIES */
pub fn is_rel_path(path: &String) -> bool {
    if let Some(bracket_ind) = path.find('<') {
	if let Some(name_sep_ind) = path.find("::") {
	    return bracket_ind < name_sep_ind;
	}
    }
    //Otherwise
    return false;
}

pub fn split_on_cols(path: &String) -> Vec<&str> {
    path.split("::").collect()
}

pub fn match_base_term(name: &String, base: &String, term: &String) -> bool {
    return name.starts_with(base) && name.ends_with(term);
}

pub fn get_term_name(path: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*?<(.*)>::").unwrap();
    }
    for cap in RE.captures_iter(path) {
	return Some(cap[1].to_string());
    }
    return None;
}

pub fn get_base_name(path: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.*?)<.*>::").unwrap();
    }
    for cap in RE.captures_iter(path) {
	return Some(cap[1].to_string());
    }
    return None;
}

/* function parsing libraries */
pub fn print_fn(name: &String, decl_fn: &String, maybe_dup: Option<&i32>) {
    lazy_static! {
	static ref RE: Regex = Regex::new(r"\((.*)\) -> (.*)").unwrap();
    }
    let mut inputs = "".to_string();
    let mut output = "".to_string();
    for cap in RE.captures_iter(decl_fn) {
	inputs  = cap[1].to_string();
	output = cap[2].to_string();
	break;
    }
    let dup = match maybe_dup {
	Some(d) => d,
	None => &0
    };
    let mut kind = "static".to_string();
    if inputs.contains("self") {
	kind =  "non-static".to_string();
    }
    println!("{{\"path\": {:?}, \"kind\": {:?}, \"inputs\": {:?}, \"output\": \"{}\", \"dup\": {} }},", name, &kind, &inputs, &output, &dup.to_string())
}

