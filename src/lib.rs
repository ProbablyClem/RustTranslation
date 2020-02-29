//! # rtr
//! 
//! rtr is a crate that return strings from translation files <br/>
//! _Thoses files need to be manually created with the [rtrTranslator](https://github.com/ProbablyClem/rtrTranslator) software_
#[macro_use]
extern crate lazy_static;

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};


lazy_static! {
    static ref ORIGIN_VEC : Mutex<Vec<String>> = Mutex::new(Vec::new()); //The vector that containt every line of the origin langage
    static ref DEST_VEC : Mutex<Vec<String>> = Mutex::new(Vec::new()); //The vector that containt every line of the destination language (e.g fr, it, de...)
    static ref STATE : Mutex<AtomicBool> = Mutex::new(AtomicBool::new(true));
}

fn create_path(source: String) -> String {
    let mut new_path = String::from("./lang/.txt");
    new_path.insert_str(7, source.as_str());
    return new_path;
}


///Set the language for the program
/// # Examples
/// 
/// To set the program in french
/// 
/// ```
/// extern crate rtr;
/// 
/// rtr::init("fr");
/// ```
///Don't forget that it will need the ./lang/fr.txt file
/// 
pub fn init(new_lang: &str) -> Result<(), io::Error> {
    let f = File::open(create_path("origin".to_string()))?;

        let f = BufReader::new(f);

        for line in f.lines() {
            ORIGIN_VEC.lock().unwrap().push(line.unwrap());
        }

        let g = File::open(create_path(new_lang.to_string()))?;

        let g = BufReader::new(g);
        for line in g.lines() {
            DEST_VEC.lock().unwrap().push(line.unwrap());
        }

        Ok(())
}

///Disable the translation <br/>
/// [rtr](./fn.rtr.html) will now return the base String
/// 
/// # Examples
/// 
/// disable() is mostly used for error haddling
/// ```
/// extern crate rtr;
/// use rtr::rtr;
/// 
/// match rtr::init("fr") {
///     Ok(_) => (),
///     Err(e) => {
///         println!("couldn't load fr translation file");
///         rtr::disable();
///     }
/// }
/// ```
/// 
/// use [enable](./fn.enable.html) to enable it again
pub fn disable() {
    &STATE.lock().unwrap().store(false, Ordering::Relaxed);
}

///Enable the translation
///[rtr](./fn.rtr.html) will now try to read strings from translation <br/>
///*the translation then need to be loaded with [init](./fn.init.html)!*
pub fn enable() {
    &STATE.lock().unwrap().store(true, Ordering::Relaxed);
}

///Use with every str that you need to translate
/// # Examples
/// 
/// ```
/// extern crate rtr;
/// use rtr::rtr;
/// 
/// rtr::init("fr");
/// println!("{}", rtr("hello world"));
/// ```
/// Will return the sentence that correspond to "hello world" in the ./lang/fr.txt file
/// 
/// 
pub fn rtr(text: &str) -> String {
    if &STATE.lock().unwrap().load(Ordering::Relaxed) == &true {
        match ORIGIN_VEC.lock().unwrap().binary_search(&text.to_string()) {
            Ok(index) => {
                return DEST_VEC.lock().unwrap()[index].clone();
            }
            Err(_) => return text.to_string(),
        }
    }

    else {
        return text.to_string();
    }
}