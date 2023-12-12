open Str
open String


let rec parser toklst =
  let tree, remain = parse_e toklst in
  if remain = [] then
    tree
  else
    failwith "SyntaxError: leftover tokens"

and parse_e toklst =
  let mtree, remain = parse_m toklst in
  if List.length remain > 0 && (List.hd remain = Plus || List.hd remain = Minus) then
    let etree, new_remain = parse_e (List.tl remain) in
    if List.hd remain = Plus then
      Add (mtree, etree), new_remain
    else
      Subtract (mtree, etree), new_remain
  else
    mtree, remain

and parse_m toklst =
  let ntree, remain = parse_n toklst in
  if List.length remain > 0 && (List.hd remain = Times || List.hd remain = Divide) then
    let mtree, new_remain = parse_m (List.tl remain) in
    if List.hd remain = Times then
      Multiply (ntree, mtree), new_remain
    else
      Divide (ntree, mtree), new_remain
  else
    ntree, remain

and parse_n toklst =
  match toklst with
  | Number n :: remain -> Number n, remain
  | LParen :: remain ->
    let etree, new_remain = parse_e (List.tl toklst) in
    if List.hd new_remain = RParen then
      etree, List.tl new_remain
    else
      failwith "SyntaxError: unbalanced parens"
  | _ -> failwith "SyntaxError: expected a number or an opening parenthesis"

let rec string_of_expression = function
  | Add (e1, e2) -> "(" ^ (string_of_expression e1) ^ " + " ^ (string_of_expression e2) ^ ")"
  | Subtract (e1, e2) -> "(" ^ (string_of_expression e1) ^ " - " ^ (string_of_expression e2) ^ ")"
  | Multiply (e1, e2) -> "(" ^ (string_of_expression e1) ^ " * " ^ (string_of_expression e2) ^ ")"
  | Divide (e1, e2) -> "(" ^ (string_of_expression e1) ^ " / " ^ (string_of_expression e2) ^ ")"
  | Square e -> "sq(" ^ (string_of_expression e) ^ ")"
  | Number n -> string_of_int n

let () =
  let result = parser (lexer "1 + 2 * sq(3)") in
  let result_str = Printf.sprintf "%s" (string_of_expression result) in
  assert (result_str = "(1 + (2 * sq(3)))");
  print_endline "Assertion passed"
