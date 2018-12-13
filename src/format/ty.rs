use std::str::FromStr;
use item::Prop;
use super::{
    prelude::*,
    Unit,
};

#[derive(Debug)]
pub enum Type {
    Bool(String, String),
    Int(Unit<isize>),
    Float(Unit<f32>),
    Str,
}

impl FromStr for Type {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Type> {
        if s.starts_with("bool") {
            let s = s[4..].trim();
            if s.is_empty() {
                Ok(Type::Bool("true".to_owned(), "false".to_owned()))
            } else if s.starts_with("[") && s.ends_with("]") {
                let labels = s[1..s.len()-1].split(",").map(|s|s.trim()).collect::<Vec<_>>();
                if labels.len() != 2 {
                    unimplemented!();
                }
                let yes = labels[0].to_owned();
                let no = labels[1].to_owned();
                Ok(Type::Bool(yes, no))
            } else {
                unimplemented!()
            }
        } else if s.starts_with("int") {
            s[3..].trim().parse().map(Type::Int)
        } else if s.starts_with("float") {
            s[5..].trim().parse().map(Type::Float)
        } else if s.starts_with("str") {
            Ok(Type::Str)
        } else {
            unimplemented!()
        }
    }
}

impl Parser for Type {
    type Output = Box<Prop>;
    fn parse(&self, s: &str) -> Result<Box<Prop>> {
        match self {
            Type::Bool(yes, no) =>
                if s == yes {
                    Ok(Box::new(true))
                } else if s == no {
                    Ok(Box::new(false))
                } else {
                    unimplemented!()
                    //Err(Box::new(format!("\"{}\" is not a bool", s)))
                },
            Type::Int(unit) => unit.parse(s),
            Type::Float(unit) => unit.parse(s),
            Type::Str => Ok(Box::new(s.to_owned()))
        }
    }
}
