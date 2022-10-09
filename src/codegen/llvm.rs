use std::ffi::{ CStr, CString };

use llvm_sys::core::*;
use llvm_sys::target::*;
use llvm_sys::LLVMModule;

use crate::ast::*;

pub fn init_llvm() {
    unsafe {
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmParsers();
        LLVM_InitializeAllAsmPrinters();
    }
}

struct Module {
    llvm_mod: *mut LLVMModule,
    static_strings: Vec<CString>,
}

impl Module {
    fn new_string_ptr(&mut self, s: &str) -> *const i8 {
        let cstring = CString::new(s).unwrap();
        let ptr = cstring.as_ptr() as *const _;
        self.static_strings.push(cstring);
        ptr
    }

    pub fn to_cstring(&self) -> CString {
        unsafe {
            let llvm_ir_ptr = LLVMPrintModuleToString(self.llvm_mod);
            let llvm_ir = CStr::from_ptr(llvm_ir_ptr as *const _);

            let mod_str = CString::new(llvm_ir.to_bytes()).unwrap();

            LLVMDisposeMessage(llvm_ir_ptr);

            mod_str
        }
    }

    pub fn add_proc_decl(&mut self, proc_decl: &ParsedProcDecl) {
    }

    pub fn add_proc_call(&mut self, proc_call: &ParsedProcCall) {
    }

    pub fn compile_proc_decl(&mut self, proc_decl: &ParsedProcDecl) {
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.llvm_mod);
        }
    }
}
