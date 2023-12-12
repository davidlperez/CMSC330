open String
open Str

let lexer instr =
  let number_re = Str.regexp "(-?[0-9]+)" in
  let terminal_re = Str.regexp "(sq|[()\\-+/*])" in
  let wspace_re = Str.regexp "(\\s+)" in

  let rec lex pos toklst =
    if pos >= String.length instr then List.rev toklst
    else if Str.string_match number_re instr pos then
      let tok = Str.matched_string instr in
      lex (pos + String.length tok) (tok::toklst)
    else if Str.string_match terminal_re instr pos then
      let tok = Str.matched_string instr in
      lex (pos + String.length tok) (tok::toklst)
    else if Str.string_match wspace_re instr pos then
      let tok = Str.matched_string instr in
      lex (pos + String.length tok) toklst
    else
      failwith ("lexer: unexpected character at position " ^ string_of_int pos)
in
lex 0 []