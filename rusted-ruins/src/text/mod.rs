mod text_id_impl;
mod to_text;

use crate::config;
use common::basic;
use fluent::*;
use std::collections::HashMap;
use std::path::PathBuf;
use unic_langid::LanguageIdentifier;
use walkdir::WalkDir;

/// Initialize lazy static
pub fn init() {
    use lazy_static::initialize;
    initialize(&OBJ_BUNDLE);
    initialize(&LOG_BUNDLE);
    initialize(&UI_BUNDLE);
    initialize(&TALK_BUNDLE);
    initialize(&MISC_BUNDLE);
}

lazy_static! {
    static ref OBJ_BUNDLE: Bundle = Bundle::load(basic::OBJ_TXT_DIR);
    static ref LOG_BUNDLE: Bundle = Bundle::load(basic::LOG_TXT_DIR);
    static ref UI_BUNDLE: Bundle = Bundle::load(basic::UI_TXT_DIR);
    static ref TALK_BUNDLE: Bundle = Bundle::load(basic::TALK_TXT_DIR);
    static ref MISC_BUNDLE: Bundle = Bundle::load(basic::MISC_TXT_DIR);
}

struct Bundle {
    first: FluentBundle<FluentResource>,
    second: FluentBundle<FluentResource>,
}

impl Bundle {
    fn load(kind: &str) -> Bundle {
        let first = load_resource(kind, &config::CONFIG.lang);
        let second_lang = &config::CONFIG.second_lang;
        let second = if second_lang == "" {
            Vec::new()
        } else {
            load_resource(kind, second_lang)
        };
        Bundle {
            first: new_bundle(&config::CONFIG.lang, first),
            second: new_bundle(&config::CONFIG.second_lang, second),
        }
    }

    fn format(&self, id: &str, args: Option<&FluentArgs>) -> Option<String> {
        let mut errors = vec![];
        if let Some(msg) = self.first.get_message(id) {
            if let Some(pattern) = msg.value {
                return Some(
                    self.first
                        .format_pattern(&pattern, args, &mut errors)
                        .into_owned(),
                );
            }
        }
        if let Some(msg) = self.second.get_message(id) {
            if let Some(pattern) = msg.value {
                return Some(
                    self.second
                        .format_pattern(&pattern, args, &mut errors)
                        .into_owned(),
                );
            }
        }
        None
    }
}

fn new_bundle(lang: &str, resource: Vec<FluentResource>) -> FluentBundle<FluentResource> {
    let langid: LanguageIdentifier = lang
        .parse()
        .expect("Parsing to language identifier failed.");
    let mut bundle = FluentBundle::new(&[langid]);

    for res in resource.into_iter() {
        if let Err(e) = bundle.add_resource(res) {
            warn!("Fluent add resource error: {:?}", e);
        }
    }

    bundle
}

fn load_resource(kind: &str, lang: &str) -> Vec<FluentResource> {
    let mut resource = Vec::new();
    let textdirs: Vec<PathBuf> = config::get_data_dirs()
        .into_iter()
        .map(|mut p| {
            p.push("text");
            p.push(lang);
            p.push(kind);
            p
        })
        .collect();

    for dir in textdirs {
        for f in WalkDir::new(dir).into_iter() {
            let f = match f {
                Ok(f) => f,
                Err(e) => {
                    warn!("{}", e);
                    continue;
                }
            };

            if !f.file_type().is_file()
                || f.path().extension().is_none()
                || f.path().extension().unwrap() != "ftl"
            {
                continue;
            }

            let s = match crate::util::read_file_as_string(f.path()) {
                Ok(s) => s,
                Err(e) => {
                    warn!("IO Error during reading a fluent file: {}", e);
                    continue;
                }
            };

            let r = match FluentResource::try_new(s) {
                Ok(r) => r,
                Err((r, err)) => {
                    for e in &err {
                        warn!(
                            "Fluent parse error in \"{}\" : {:?}",
                            f.path().to_string_lossy(),
                            e
                        );
                    }
                    r
                }
            };

            resource.push(r);
        }
    }

    resource
}

pub fn obj_txt(id: &str) -> String {
    if let Some(s) = OBJ_BUNDLE.format(id, None) {
        s
    } else {
        id.to_owned()
    }
}

#[allow(unused)]
pub fn obj_txt_checked(id: &str) -> Option<String> {
    OBJ_BUNDLE.format(id, None)
}

pub fn log_txt(id: &str) -> String {
    log_txt_with_args(id, None)
}

pub fn log_txt_with_args(id: &str, args: Option<&HashMap<&str, FluentValue>>) -> String {
    if let Some(s) = LOG_BUNDLE.format(id, args) {
        s
    } else {
        id.to_owned()
    }
}

pub fn ui_txt(id: &str) -> String {
    ui_txt_with_args(id, None)
}

pub fn ui_txt_with_args(id: &str, args: Option<&HashMap<&str, FluentValue>>) -> String {
    if let Some(s) = UI_BUNDLE.format(id, args) {
        s
    } else {
        id.to_owned()
    }
}

#[allow(unused)]
pub fn ui_txt_checked(id: &str) -> Option<String> {
    UI_BUNDLE.format(id, None)
}

pub fn talk_txt(id: &str) -> String {
    talk_txt_with_args(id, None)
}

pub fn talk_txt_with_args(id: &str, args: Option<&HashMap<&str, FluentValue>>) -> String {
    if let Some(s) = TALK_BUNDLE.format(id, args) {
        s
    } else {
        id.to_owned()
    }
}

pub fn talk_txt_checked(id: &str, args: Option<&HashMap<&str, FluentValue>>) -> Option<String> {
    TALK_BUNDLE.format(id, args)
}

pub fn misc_txt(id: &str) -> String {
    misc_txt_with_args(id, None)
}

pub fn misc_txt_with_args(id: &str, args: Option<&HashMap<&str, FluentValue>>) -> String {
    if let Some(s) = MISC_BUNDLE.format(id, args) {
        s
    } else {
        id.to_owned()
    }
}

/// This is helper trait for some data objects that need to be printed in game.
/// Logging macros use this.
pub trait ToText {
    fn to_text(&self) -> std::borrow::Cow<str>;
}

/// Types that have text id.
/// Returned text id is translated to appropriate words in text module.
pub trait ToTextId {
    fn to_textid(&self) -> &'static str;
}

pub fn to_txt<T: ToTextId>(a: &T) -> String {
    misc_txt(a.to_textid())
}

macro_rules! misc_txt_format {
    ($id:expr; $($target:ident = $value:expr),*) => {{
        let mut table: std::collections::HashMap<&str, fluent::FluentValue>
            = std::collections::HashMap::new();
        $(
            let value = fluent::FluentValue::String($value.to_text());
            table.insert(stringify!($target), value);
        )*

        crate::text::misc_txt_with_args($id, Some(&table))
    }}
}

macro_rules! ui_txt_format {
    ($id:expr; $($target:ident = $value:expr),*) => {{
        let mut table: std::collections::HashMap<&str, fluent::FluentValue>
            = std::collections::HashMap::new();
        $(
            let value = fluent::FluentValue::String($value.to_text());
            table.insert(stringify!($target), value);
        )*

        crate::text::ui_txt_with_args($id, Some(&table))
    }}
}
