use pest::Parser;
use pest_derive::Parser;
use std::path::Path;
use std::fs;

#[derive(Parser)]
#[grammar = "compiler/ut.pest"]
pub struct UtParser;

#[derive(Debug)]
pub enum Rule {
    file,
    variable,
    struct_def,
    struct_init,
    enum_def,
    enum_init,
}

#[derive(Debug)]
pub enum Type {
    Int, Char, Float, Double,
    UnsignedInt, LongInt,
    Auto(String),
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub var_type: Type,
}

#[derive(Debug)]
pub struct StructField {
    pub name: String,
    pub value: String,
    pub field_type: Type,
}

#[derive(Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct StructInit {
    pub name: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug)]
pub enum EnumVariant {
    Simple(String),
}

#[derive(Debug)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug)]
pub struct EnumInit {
    pub name: String,
    pub variant: String,
}

#[derive(Debug)]
pub enum AstNode {
    Variable(Variable),
    StructDef(StructDef),
    StructInit(StructInit),
    EnumDef(EnumDef),
    EnumInit(EnumInit),
}

pub fn parse_ut_file(path: &Path) -> Result<Vec<AstNode>, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let pairs = UtParser::parse(Rule::file, &content).map_err(|e| e.to_string())?;
    let mut ast = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::variable => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let value = inner.next().unwrap().as_str().to_string();
                let type_str = inner.next().map(|t| t.as_str());
                let var_type = match type_str {
                    Some("int") => Type::Int,
                    Some("char") => Type::Char,
                    Some("float") => Type::Float,
                    Some("double") => Type::Double,
                    Some("unsigned int") => Type::UnsignedInt,
                    Some("long int") => Type::LongInt,
                    None => match value.parse::<i32>() {
                        Ok(_) => Type::Auto("int".to_string()),
                        Err(_) => Type::Auto("float".to_string()),
                    },
                    Some(t) => return Err(format!("Unknown type: {}", t)),
                };
                ast.push(AstNode::Variable(Variable { name, value, var_type }));
            }
            Rule::struct_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let mut fields = Vec::new();
                for field in inner {
                    let mut f = field.into_inner();
                    let fname = f.next().unwrap().as_str().to_string();
                    let fvalue = f.next().unwrap().as_str().to_string();
                    let ftype = match f.next().unwrap().as_str() {
                        "int" => Type::Int,
                        "char" => Type::Char,
                        "float" => Type::Float,
                        "double" => Type::Double,
                        t => return Err(format!("Unknown struct field type: {}", t)),
                    };
                    fields.push(StructField { name: fname, value: fvalue, field_type: ftype });
                }
                ast.push(AstNode::StructDef(StructDef { name, fields }));
            }
            Rule::struct_init => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let mut fields = Vec::new();
                for field in inner {
                    let mut f = field.into_inner();
                    let fname = f.next().unwrap().as_str().to_string();
                    let fvalue = f.next().unwrap().as_str().to_string();
                    fields.push((fname, fvalue));
                }
                ast.push(AstNode::StructInit(StructInit { name, fields }));
            }
            Rule::enum_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let mut variants = Vec::new();
                for variant in inner {
                    variants.push(EnumVariant::Simple(variant.as_str().to_string()));
                }
                ast.push(AstNode::EnumDef(EnumDef { name, variants }));
            }
            Rule::enum_init => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let variant = inner.next().unwrap().as_str().to_string();
                ast.push(AstNode::EnumInit(EnumInit { name, variant }));
            }
            _ => {}
        }
    }
    Ok(ast)
}