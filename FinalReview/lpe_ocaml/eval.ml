type node = { value: string; typ: string option; left: node option; right: node option }

let rec eval_node tree =
  match tree.typ with
  | Some t ->
    let leftv = eval_node tree.left in
    (match tree.value with
     | "sq" -> leftv * leftv
     | "+" ->
       let rightv = eval_node tree.right in
       leftv + rightv
     | "-" ->
       let rightv = eval_node tree.right in
       leftv - rightv
     | "*" ->
       let rightv = eval_node tree.right in
       leftv * rightv
     | "/" ->
       let rightv = eval_node tree.right in
       leftv / rightv
     | _ -> failwith "Invalid operation"
    )
  | None -> int_of_string tree.value

let eval tree = eval_node tree
