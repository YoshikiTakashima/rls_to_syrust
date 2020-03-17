use rls_analysis::*;
use std::path::Path;
use std::string::String;
use log::{info, trace, warn};

fn process_leaf(_host: &AnalysisHost, _def: &Def)  {
    println!("name: {:?}, kind: {:?}, type: {:?}", _def.qualname,  _def.kind, _host.show_type(&_def.span));
}

fn process_children(_parent_id: Id, _def: &Def) -> Def {    
    return _def.clone();
}

/*
Recursive helper.
*/
fn recurse_crate_tree(host: &AnalysisHost, _id: &Id) {
    if let Ok(current_def) = host.get_def(*_id) {
	if current_def.qualname.contains("string") { //&& current_def.qualname.contains("string")
	    println!("ID: {}", &_id);
	    process_leaf(host, &current_def);
	}
    }
    
    
    if let Ok(child_defns) =  host.for_each_child_def(*_id, process_children) {
	for i in 0..child_defns.len() {
	    if let Ok(child_id) = host.crate_local_id(&child_defns[i].span) {
		recurse_crate_tree(host, &child_id);
	    }
	}
    }
}

/*
Caller for init. Helps with top level filtering.
*/
fn parse_crate_tree(host: &AnalysisHost) {
    if let Ok(vec) = host.def_roots() {
	for i in 0..vec.len() {
	    if true { // or some predicate of vec[i].1
		recurse_crate_tree(host, &vec[i].0);
	    }
	}
    }
}

fn main() {
    let host = AnalysisHost::new(Target::Debug);
    let p = Path::new("/home/ytakashima/Desktop/RustSynth/rls-analysis/test_data/rust-analysis");
    if host.reload(p, Path::new("/home/ytakashima/Desktop/RustSynth/rls_to_syrust")).is_ok() {
	println!("Roots: {:?}\n", host.def_roots());
	//println!("Roots: {:?}\n", host.matching_defs("String::new"));
	//println!("Roots: {:?}\n", host.get_def(Id::new(8589934592)));
	//println!("Roots: {:?}\n", host.for_each_child_def(Id::new(21474836480), print_children));
	//parse_crate_tree(&host);
	//println!("{:?}\n", &host.def_parents(Id::new(21474841212))); 
	if let Ok(parents) = host.def_parents(Id::new(21474841212)) {
	    for j in 0..parents.len() {
		//println!("Types of IMPL(21474841212): {:?}\n\n", host.find_all_refs(&parents[j], true, true)); 
	    }
	}
	if let Ok(children2) = host.matching_defs("with_capacity") {
	    for k in 0..children2.len() {
		println!("with_capacity: {:?} {:?}", &children2[k], host.id(&children2[k].span));
	    }
	    println!("Array Length: {:?}", children2.len());
	}
    }
}
