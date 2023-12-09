open LccTypes
open Str
open String

let lex_lambda input =
  let length = String.length input in

  let is_variable_char c =
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') in
  let rec lex pos =
    if pos >= length then [Lambda_EOF]
    else
      match input.[pos] with
      | '(' -> Lambda_LParen :: lex (pos + 1)
      | ')' -> Lambda_RParen :: lex (pos + 1)
      | '.' -> Lambda_Dot :: lex (pos + 1)
      | 'L' -> Lambda_Lambda :: lex (pos + 1)
      | ' ' -> lex (pos + 1)
      | c when is_variable_char c -> Lambda_Var (String.make 1 c) :: lex (pos + 1)
      | _ -> raise (Failure "tokenizing failed")
    in
  lex 0

let lex_engl input = 
  let length = String.length input in

  let rec lex pos =
    if pos >= length then [Engl_EOF]
    else if Str.string_match (Str.regexp "(") input pos then
      Engl_LParen :: (lex (pos + 1))
    else if Str.string_match (Str.regexp ")") input pos then
      Engl_RParen :: (lex (pos + 1))
    else if Str.string_match (Str.regexp "true") input pos then
      Engl_True :: (lex (pos + 4))
    else if Str.string_match (Str.regexp "false") input pos then
      Engl_False :: (lex (pos + 5))
    else if Str.string_match (Str.regexp "if") input pos then
      Engl_If :: (lex (pos + 2))
    else if Str.string_match (Str.regexp "then") input pos then
      Engl_Then :: (lex (pos + 4))
    else if Str.string_match (Str.regexp "else") input pos then
      Engl_Else :: (lex (pos + 4))
    else if Str.string_match (Str.regexp "and") input pos then 
      Engl_And :: (lex (pos + 3))
    else if Str.string_match (Str.regexp "or") input pos then
      Engl_Or :: (lex (pos + 2))
    else if Str.string_match (Str.regexp "not") input pos then
      Engl_Not :: (lex (pos + 3))
    else if Str.string_match (Str.regexp " ") input pos then
      lex (pos + 1)
    else raise (Failure "tokenizing failed")
  in
  lex 0