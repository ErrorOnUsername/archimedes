use crate::ast::ParsedType;

pub type TypeID = usize;

pub struct Typechecker {
    registry: Vec<ParsedType>
}

impl Typechecker {
    pub fn new() -> Self {
        Self {
            registry: Vec::new()
        }
    }

    pub fn preregister_primitives() {
        todo!()
    }

    pub fn register_type(
        &mut self,
        ty: ParsedType
    ) -> Result<TypeID, &'static str> {
        if self.registry.contains(&ty) {
            return Err("Type already defined");
        }

        self.registry.push(ty);

        Ok(self.registry.len() - 1)
    }

    pub fn get_type_id(
        &self,
        ty: &ParsedType
    ) -> Result<TypeID, &'static str> {
        for (idx, o_ty) in self.registry.iter().enumerate() {
            if o_ty == ty {
                return Ok(idx);
            }
        }

        Err("Type not registered")
    }

    pub fn is_type_registered(&self, ty: &ParsedType) -> bool {
        self.registry.contains(ty)
    }
}
