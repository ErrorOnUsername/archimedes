" Vim syntax file
" Language: Archimedes
" Lastest Revision: 06-27-2022

if exists("b:current_syntax")
	finish
endif

let s:cpo_save = &cpo
set cpo&vim

"
" Keywords
"
syn keyword amds_conditionals    if else match
syn keyword amds_loops           while for loop
syn keyword amds_ctrl_flow       return break continue
syn keyword amds_boolean         true false
syn keyword amds_keyword         import decl let mut
syn keyword amds_type            nothing bool u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64
syn keyword amds_complex_type    struct enum
syn keyword amds_todo            contained NOTE TODO FIXME BUG

"
" Numbers
"
syn match amds_dec_number display "\v<\d%('?\d)*"
syn match amds_bin_number display "\v<0b[01]%('?[01])*"
syn match amds_oct_number display "\v<0o\o%('?\o)*"
syn match amds_hex_number display "\v<0x\x%('?\x)*"

hi def link amds_dec_number      amds_number
hi def link amds_bin_number      amds_number
hi def link amds_oct_number      amds_number
hi def link amds_hex_number      amds_number

"
" String
"
syn region amds_string_literal matchgroup=amds_string_delim start=+"+ skip=+\\\\\|\\"+ end=+"+ oneline contains=amds_escape
syn region amds_char_literal   matchgroup=amds_char_delim start=+'+ skip=+\\\\\|\\'+ end=+'+ oneline contains=amds_escape
syn match  amds_escape         display contained /\\./

"
" Operators
"
syn match amds_simple_op      display "\V\[-+/*=^&?|!><%~:;,]"
syn match amds_thicc_arrow_op display "\V=>"
syn match amds_thin_arrow_op  display "\V->"
syn match amds_range_op       display "\V.."

hi def link amds_simple_op       amds_operator
hi def link amds_thicc_arrow_op  amds_operator
hi def link amds_thin_arrow_op   amds_operator
hi def link amds_range_arrow_op  amds_operator

"
" Functions
"
syn match amds_proc_decl /decl\s\+\w\+\s\+:\s\+(/lc=4,he=e-4
syn match amds_proc_call /\w\+\s*(/me=e-1,he=e-1

hi def link amds_proc_decl       amds_proc
hi def link amds_proc_call       amds_proc

"
" Types
"
syn match amds_struct_decl /decl\s\+\w\+\s\+:\s\+struct\+/lc=4,me=e-8
syn match amds_enum_decl   /decl\s\+\w\+\s\+:\s\+enum\+/lc=4,me=e-6

hi def link amds_struct_decl     amds_type
hi def link amds_enum_decl       amds_type

"
" Comments
"
syn region amds_line_comment  start="//"  end="$"   contains=amds_todo
syn region amds_block_comment start="/\*" end="\*/" contains=amds_todo

hi def link amds_line_comment    amds_comment
hi def link amds_block_comment   amds_comment

"
" Linking to vim highlight types
"
hi def link amds_conditionals    Conditional
hi def link amds_loops           Repeat
hi def link amds_ctrl_flow       Special
hi def link amds_boolean         Boolean
hi def link amds_keyword         Keyword
hi def link amds_type            Type
hi def link amds_complex_type    Structure
hi def link amds_number          Number
hi def link amds_operator        Operator
hi def link amds_comment         Comment
hi def link amds_proc            Function
hi def link amds_string_literal  String
hi def link amds_string_delim    String
hi def link amds_char_literal    String
hi def link amds_char_delim      String

let b:current_syntax = "archimedes"

let &cpo = s:cpo_save
unlet s:cpo_save
