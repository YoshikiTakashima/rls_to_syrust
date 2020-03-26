mod string_utils;
use string_utils::*;

use rls_analysis::*;
use std::path::Path;
use std::collections::HashMap;

fn print_kind(def: &Def) {
    match def.kind {
	DefKind::Enum => println!("Kind: Enum, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::TupleVariant => println!("Kind: TupleVariant, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::StructVariant => println!("Kind: StructVariant, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Tuple => println!("Kind: Tuple, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Struct => println!("Kind: Struct, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Union => println!("Kind: Union, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Trait => println!("Kind: Trait, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Function => println!("Kind: Function, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::ForeignFunction => println!("Kind: ForeignFunction, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Method => println!("Kind: Method, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Macro => println!("Kind: Macro, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Mod => println!("Kind: Module, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Type => println!("Kind: Type, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Local => println!("Kind: Local, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Static => println!("Kind: Static, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::ForeignStatic => println!("Kind: Foreignstatic, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Const => println!("Kind: Const, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::Field => println!("Kind: Field, Name: {:?}, {:?}", def.qualname, def.value),
	DefKind::ExternType => println!("Kind: Externtype, Name: {:?}, {:?}", def.qualname, def.value)
    };
}

/*
Return value (Objects (i.e. Enum and Struct), Functions defined)
*/
fn decompose_defs<'a>(host: &AnalysisHost, defs : &'a Vec<(Id, Def)>) ->
     (HashMap<String, &'a (Id, Def)>, HashMap<String, &'a Def>,
      HashMap<Span, &'a (Id, Def)>) {
	 let mut objects: HashMap<String, &(Id, Def)> = HashMap::new();
	 let mut functions: HashMap<String, &Def> = HashMap::new();
	 let mut rel_path_methods: HashMap<String, &Def> = HashMap::new();
	 let mut trait_impls: HashMap<Span, &(Id, Def)> = HashMap::new();
	 
	 for def in defs.iter() {
	     match def.1.kind {
		 DefKind::Enum => {
		     objects.insert(def.1.qualname.clone(), def); 
		 },
		 DefKind::Struct => {
		     objects.insert(def.1.qualname.clone(), def);
		 },
		 DefKind::Function => {
		     functions.insert(def.1.qualname.clone(), &def.1); 
		 },
		 DefKind::Method => {
		     if is_rel_path(&def.1.qualname) {
			 rel_path_methods.insert(def.1.qualname.clone(), &def.1);
		     } else {
			 functions.insert(def.1.qualname.clone(), &def.1);
		     }
		 },
		 DefKind::Trait => {
		     if let Ok(impls) = host.find_impls(def.0) {
			 for span in impls.iter() {
			     trait_impls.insert(span.clone(), def);
			 }
		     }
		 },
		 _ => {}
	     }
	 }

	 for (pth, def) in rel_path_methods.iter() {
	     if let Some(fixed) = fix_path(&objects, &pth, def) {
		 functions.insert(fixed, def);
	     }
	 }
	 
	 return (objects, functions, trait_impls);
     }

fn fix_path(objects: & HashMap<String, &(Id, Def)>, path: &String, def: &Def) -> Option<String> {
    if let Some(base) = get_base_name(path) { //first part
	if let Some(mut term) = get_term_name(path) { //last part
	    if let Some(i) = term.find('<') {
		term = term[0..i].to_string();
	    }
	    for (name, _) in objects.iter() {
		if match_base_term(&name, &base, &term) {
		    return Some(format!("{}::{}", &name, &def.name));
		}
	    }
	}
    }
    return None;
}

fn main() {
    let host = AnalysisHost::new(Target::Debug);
    let p = Path::new("/home/ytakashima/Desktop/RustSynth/rls-analysis/test_data/rust-analysis");
    if host.reload(p, Path::new("/home/ytakashima/Desktop/RustSynth/rls_to_syrust")).is_ok() {	
	if let Ok(mut defs) =  host.dump_defs() {
	    /*for def in defs.iter() {
		print_kind(&def.1);
	    }*/
	    defs = defs.into_iter()
		.filter(|d|   d.1.qualname.starts_with("alloc")
			|| d.1.qualname.starts_with("std")
			|| d.1.qualname.starts_with("core"))
		.collect();
	    let (obj, fun, traits) = decompose_defs(&host, &defs);
	    
	    for (name, def) in fun.iter() {
		print_fn(&name, &def.value, Some(&-1));
	    }

	    let mut ctrmap: HashMap<String, i32> = HashMap::new();
	    for (name, id_def) in obj.iter() {
		if let Ok(impl_spans) = host.find_impls(id_def.0) {
		    for span in impl_spans.iter() {
			if let Some(i_df) = traits.get(&span) {
			    let ctr = &mut ctrmap;
			    let _x = host.for_each_child_def(i_df.0, move |_, df| {
				if df.kind == DefKind::Method || df.kind == DefKind::Function {
				    print_fn(&format!("{}::{}", name, &df.name),
					     &df.value, ctr.get(&df.name));
				    match ctr.get(&df.name) {
					Some(x) => {
					    ctr.insert(df.name.clone(), x+1);
					},
					None => {
					    ctr.insert(df.name.clone(), 1);   
					}
				    };
				}
			    });
			}
		    }
		}
		ctrmap.clear();
	    }
	}
    }
    //"fn (n: u128, d: u128, rem: Option<&mut u128>) -> u128"
    //"std<LazyKeyInner<T>>::take"
}
