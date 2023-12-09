open LccTypes 

let match_token (toks : 'a list) (tok : 'a) : 'a list =
  match toks with
  | [] -> raise (Failure("List was empty"))
  | h::t when h = tok -> t
  | h::_ -> raise (Failure( 
      Printf.sprintf "Token passed in does not match first token in list"
    ))

let lookahead toks = match toks with
   h::t -> h
  | _ -> raise (Failure("Empty input to lookahead"))


(* Write your code below *)

let parse_lambda toks = 
  let rec parse toks = 
    match toks with
    | [] -> raise (Failure "parsing failed")
    | _ -> let (remain, ast) = parse_e toks in
      if remain <> [Lambda_EOF] then raise (Failure "parsing failed")
      else ast and parse_e toks =
        match lookahead toks with
        | Lambda_Var var -> let remain = match_token toks (Lambda_Var var) in
          (remain, Var var)
        | Lambda_LParen -> let remain = match_token toks Lambda_LParen in
          (match lookahead remain with
          | Lambda_Lambda -> parse_func remain
          | _ -> parse_app remain)
        | _ -> raise (Failure "parsing failed") and parse_func toks =
          let remain = match_token toks Lambda_Lambda in
            match lookahead remain with
            | Lambda_Var var -> let remain = match_token remain (Lambda_Var var) in
              let remain = match_token remain Lambda_Dot in
              let (remain, e) = parse_e remain in
              let remain = match_token remain Lambda_RParen in
              (remain, Func(var, e))
            | _ -> raise (Failure "parsing failed") and parse_app toks =
              let (remain, e1) = parse_e toks in
              let (remain, e2) = parse_e remain in
              match remain with
              | Lambda_RParen::_ -> let remain = match_token remain Lambda_RParen in
                (remain, Application (e1, e2))
              | _ -> raise (Failure "parsing failed") in
  parse toks

let parse_engl toks =
  let rec parse toks =
    match toks with
    | [] -> raise (Failure "parsing failed")
    | _ -> let (remain, ast) = parse_C toks in
      if remain <> [Engl_EOF] then raise (Failure "parsing failed")
      else ast and parse_C toks =
        match lookahead toks with
        | Engl_If -> let remain = match_token toks Engl_If in
          let (remain, c1) = parse_C remain in
          let remain = match_token remain Engl_Then in
          let (remain, c2) = parse_C remain in
          let remain = match_token remain Engl_Else in
          let (remain, c3) = parse_C remain in
          (remain, If (c1, c2, c3))
        | _ -> parse_H toks and parse_H toks =
          let (remain, u) = parse_U toks in
          match lookahead remain with
          | Engl_And -> let remain = match_token remain Engl_And in
            let (remain, h) = parse_H remain in
            (remain, And (u, h))
          | Engl_Or ->
            let remain = match_token remain Engl_Or in let (remain, h) = parse_H remain in
            (remain, Or (u,h))
          | _ -> (remain, u) and parse_U toks =
            match lookahead toks with
            | Engl_Not -> let remain = match_token toks Engl_Not in
              let (remain, u) = parse_U remain in (remain, Not u)
            | _ -> parse_M toks and parse_M toks =
              match lookahead toks with
              | Engl_True -> let remain = match_token toks Engl_True in (remain, Bool true)
              | Engl_False -> let remain = match_token toks Engl_False in (remain, Bool false)
              | Engl_LParen -> let remain = match_token toks Engl_LParen in
                let (remain, c) = parse_C remain in
                let remain = match_token remain Engl_RParen in (remain, c)
              | _ -> raise (Failure "parsing failed") in
  parse toks
              