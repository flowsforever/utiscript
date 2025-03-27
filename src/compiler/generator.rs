use crate::compiler::parser::{AstNode, Type, StructDef, StructInit, EnumDef, EnumInit};

pub fn generate_c_code(ast: &[AstNode]) -> String {
    let mut c_code = String::from("#include <stdio.h>\n\n");
    
    for node in ast {
        match node {
            AstNode::Variable(var) => {
                let c_type = match &var.var_type {
                    Type::Int => "int",
                    Type::Char => "char",
                    Type::Float => "float",
                    Type::Double => "double",
                    Type::UnsignedInt => "unsigned int",
                    Type::LongInt => "long int",
                    Type::Auto(t) => t.as_str(),
                };
                c_code.push_str(&format!("{} {} = {};\n", c_type, var.name, var.value));
            }
            AstNode::StructDef(sdef) => {
                c_code.push_str(&format!("struct {} {{\n", sdef.name));
                for field in &sdef.fields {
                    let ftype = match field.field_type {
                        Type::Int => "int",
                        Type::Char => "char",
                        Type::Float => "float",
                        Type::Double => "double",
                        _ => "int",
                    };
                    c_code.push_str(&format!("    {} {};\n", ftype, field.name));
                }
                c_code.push_str("};\n");
            }
            AstNode::StructInit(sinit) => {
                c_code.push_str(&format!("struct {} {} = {{", sinit.name, sinit.fields[0].0));
                for (i, (_, value)) in sinit.fields.iter().enumerate() {
                    if i > 0 {
                        c_code.push_str(", ");
                    }
                    c_code.push_str(value);
                }
                c_code.push_str("};\n");
            }
            AstNode::EnumDef(edef) => {
                c_code.push_str(&format!("enum {} {{\n", edef.name));
                for (i, variant) in edef.variants.iter().enumerate() {
                    if let EnumVariant::Simple(v) = variant {
                        c_code.push_str(&format!("    {}", v));
                        if i < edef.variants.len() - 1 {
                            c_code.push_str(",");
                        }
                        c_code.push_str("\n");
                    }
                }
                c_code.push_str("};\n");
            }
            AstNode::EnumInit(einit) => {
                c_code.push_str(&format!("enum {} {} = {};\n", einit.name, einit.variant, einit.variant));
            }
        }
    }
    
    c_code.push_str("\nint main() {\n    return 0;\n}\n");
    c_code
}