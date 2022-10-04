use std::collections::HashMap;

use crate::ast::{
    ParsedType,
    ParsedModule,
};
use crate::token::PrimitiveType;

pub type TypeID = usize;

pub enum TypeInfo {
    Primitive(PrimitiveType, usize),
    Struct(String, HashMap<String, Box<TypeInfo>>),
    Enum(String, Vec<Box<TypeInfo>>),
    EnumVariant(String, HashMap<String, Box<TypeInfo>>),
}

impl TypeInfo {
    pub fn name(&self) -> &String {
        todo!()
    }
}

pub struct Typechecker {
    registry: Vec<TypeInfo>
}

impl Typechecker {
    pub fn new() -> Self {
        Self {
            registry: Vec::new()
        }
    }

    pub fn primitive(ty: PrimitiveType) -> ParsedType {
        match ty {
            PrimitiveType::Nothing => ParsedType::Name(Vec::new(), String::from("nothing")),
            PrimitiveType::Bool => ParsedType::Name(Vec::new(), String::from("bool")),
            PrimitiveType::I8 => ParsedType::Name(Vec::new(), String::from("i8")),
            PrimitiveType::U8 => ParsedType::Name(Vec::new(), String::from("u8")),
            PrimitiveType::I16 => ParsedType::Name(Vec::new(), String::from("i16")),
            PrimitiveType::U16 => ParsedType::Name(Vec::new(), String::from("u16")),
            PrimitiveType::I32 => ParsedType::Name(Vec::new(), String::from("i32")),
            PrimitiveType::U32 => ParsedType::Name(Vec::new(), String::from("u32")),
            PrimitiveType::I64 => ParsedType::Name(Vec::new(), String::from("i64")),
            PrimitiveType::U64 => ParsedType::Name(Vec::new(), String::from("u64")),
            PrimitiveType::F32 => ParsedType::Name(Vec::new(), String::from("f32")),
            PrimitiveType::F64 => ParsedType::Name(Vec::new(), String::from("f64")),
            PrimitiveType::Char => ParsedType::Name(Vec::new(), String::from("char")),
            PrimitiveType::String => ParsedType::Name(Vec::new(), String::from("string")),
        }
    }

    pub fn preregister_primitives(&mut self) {
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::Nothing)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::Bool)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::I8)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::U8)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::I16)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::U16)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::I32)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::U32)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::I64)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::U64)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::F32)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::F64)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::Char)).unwrap();
        let _ = self.register_type(&Typechecker::primitive(PrimitiveType::String)).unwrap();
    }

    pub fn register_type(
        &mut self,
        ty: &ParsedType
    ) -> Result<TypeID, &'static str> {
        todo!()
    }

    pub fn get_type_id(
        &self,
        ty_name: &String
    ) -> Result<TypeID, &'static str> {
        todo!()
    }

    pub fn is_type_registered(&self, ty_name: &String) -> bool {
        todo!()
    }

    fn add_type_info(&self, ty: &ParsedType) {
        todo!()
    }

    pub fn verify_module(&mut self, module: &ParsedModule) -> Result<(), &'static str>{
        todo!()
    }
}
