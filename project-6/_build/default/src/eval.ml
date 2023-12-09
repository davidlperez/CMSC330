open LccTypes 

let cntr = ref (-1)

let fresh () =
  cntr := !cntr + 1 ;
  !cntr

let rec lookup env var = match env with 
  [] -> None
  |(v,e)::t -> if v = var then e else lookup t var

let rec alpha_convert e = 
  let cntr = cntr := -1 in
  let rec conv env e =
    match e with
    | Var x -> (match lookup env x with
      | None -> Var x
      | Some y -> y)
    | Func (x, y) -> let new_var = string_of_int (fresh ()) in Func (new_var, conv ((x, Some(Var(new_var)))::env) y)
    | Application (e1, e2) -> Application (conv env e1, conv env e2) in
  conv [] e

  let rec equal_ast ast1 ast2 = match (ast1, ast2) with
  | Var x, Var y -> x = y
  | Func (x1, e1), Func (x2, e2) -> x1 = x2 && equal_ast e1 e2
  | Application (e11, e12), Application (e21, e22) -> equal_ast e11 e21 && equal_ast e12 e22
  | _, _ -> false

let isalpha ast1 ast2 =
  let alpha_ast1 = alpha_convert ast1 in
  let alpha_ast2 = alpha_convert ast2 in
  equal_ast alpha_ast1 alpha_ast2

let rec sub old_var new_var ast =
  match ast with
  | Var var -> if var = old_var then new_var else Var var
  | Func (var, e) -> if var = old_var then Func (var, e) else Func (var, sub old_var new_var e)
  | Application (e1, e2) -> Application (sub old_var new_var e1, sub old_var new_var e2)

let rec reduce env e = 
  match e with
  | Var var -> (match lookup env var with
    | Some e -> e
    | None -> Var var)
  | Func (var, e) -> Func (var, reduce env e)
  | Application (Func (var, e), arg) -> reduce env (sub var arg e)
  | Application (e1, e2) -> if e1 = reduce env e1 && e2 = reduce env e2 then Application (e1, e2) else reduce env (Application (reduce env e1, reduce env e2))

let rec laze env e = 
  match e with
  | Var var -> (match lookup env var with
    | Some env_e -> env_e
    | None -> e)
  | Func (var, e_body) -> Func (var, e_body)
  | Application (Func (var, e_body), e_arg) -> sub var e_arg e_body
  | Application (e1, e2) -> if laze env e1 = e1  then Application (e1, e2) else laze env e1

let rec eager env e = 
  match e with
  | Var var -> (match lookup env var with
    | Some env_e -> env_e
    | None -> e)
  | Func (var, e_body) -> Func (var, e_body)
  | Application (Func (var, e_body), arg) -> let new_arg = eager env arg in
    if new_arg = arg then 
      let extenv = (var, Some new_arg)::env in
      eager extenv e_body else Application (Func (var, e_body), new_arg)
  | Application (e1, e2) -> let new_e1 = eager env e1 in let new_e2 = eager env e2 in
    if new_e1 = e1 && new_e2 = e2 then Application (new_e1, new_e2) else eager env (Application (new_e1, new_e2))

let rec convert tree =
  match tree with
  | Bool true -> "(Lx.(Ly.x))"
  | Bool false -> "(Lx.(Ly.y))"
  | If (a, b, c) -> "((" ^ (convert a) ^ " " ^ (convert b) ^ ") " ^ (convert c) ^ ")"
  | Not a -> "((Lx.((x (Lx.(Ly.y))) (Lx.(Ly.x)))) " ^ (convert a) ^ ")"
  | And (a, b) -> "(((Lx.(Ly.((x y) (Lx.(Ly.y))))) " ^ (convert a) ^ ") " ^ (convert b) ^ ")"
  | Or (a, b) -> "(((Lx.(Ly.((x (Lx.(Ly.x))) y))) " ^ (convert a) ^ ") " ^ (convert b) ^ ")"

let rec readable tree = 
  match tree with
  | Func(x1, Func(y, Var(x2))) when x1 <> y && x1 = x2 -> "true"
  | Func(x, Func(y1, Var(y2))) when x <> y1 && y1 = y2 -> "false"
  | Application(Func(x1, Application(Application(Var(x2), Func(x3, Func(y1, Var(y2)))), Func(x4, Func(y3, Var(x5))))), a)
    when x1 = x2 && x2 = x3 && x3 = x4 && x4 = x5 && x1 <> y1 && y1 = y2  -> "(not " ^ readable a ^ ")"
  | Application(Application(Func(x1, Func(y1, Application( Application(Var(x2), Var(y2)), Func(x3, Func(y3, Var(y4)))))), a), b)
    when x1 = x2 && x2 = x3 && x1 <> y1 && y1 = y2 && y2 = y3 && y3 = y4 -> "(" ^ readable a ^ " and " ^ readable b ^ ")"
  | Application(Application(Func(x1, Func(y1, Application(Application(Var(x2), Func(x3, Func(y2, Var(x4)))), y))), a), b)
    when x1 = x2 && x2 = x3 && x3 = x4 && x1 <> y1 && y1 = y2 -> "(" ^ readable a ^ " or " ^ readable b ^ ")"
  | Application(Application(a, b), c) -> "(if " ^ readable a ^ " then " ^ readable b ^ " else " ^ readable c ^ ")"
  | _ -> "failed"
