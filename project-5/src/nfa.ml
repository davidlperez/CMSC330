open List
open Sets

(*********)
(* Types *)
(*********)

type ('q, 's) transition = 'q * 's option * 'q

type ('q, 's) nfa_t = {
  sigma: 's list;
  qs: 'q list;
  q0: 'q;
  fs: 'q list;
  delta: ('q, 's) transition list;
}

(***********)
(* Utility *)
(***********)

(* explode converts a string to a character list *)
let explode (s: string) : char list =
  let rec exp i l =
    if i < 0 then l else exp (i - 1) (s.[i] :: l)
  in
  exp (String.length s - 1) []

(****************)
(* Part 1: NFAs *)
(****************)

let move (nfa: ('q,'s) nfa_t) (qs: 'q list) (s: 's option) : 'q list =
  List.fold_left (fun acc (src, sym, dest) -> if (List.mem src qs && sym = s && not (List.exists(fun x -> x = dest) acc))
    then dest::acc else acc) [] nfa.delta

let e_closure (nfa: ('q,'s) nfa_t) (qs: 'q list) : 'q list =
  let rec aux states = 
    let rec aux2 sym state =
      match sym with
      | [] -> state
      | (src, None, dest) :: rest when List.mem src state && not (List.mem dest state) -> aux2 rest (dest::state)
      | _ :: rest -> aux2 rest state
    in
    let new_states = aux2 nfa.delta states in
    if new_states = states then states else aux new_states
  in
  aux qs

let accept (nfa: ('q,char) nfa_t) (s: string) : bool =
  let char_list = explode s in
  let start_state = e_closure nfa [nfa.q0] in
  let rec aux nfa curr char_list =
    match char_list with
    | [] -> List.exists (fun q -> List.mem q nfa.fs) curr
| h::t -> let next = List.flatten (List.map (fun q -> e_closure nfa [q]) (List.flatten (List.map (fun q -> move nfa [q] (Some h)) curr))) in
    aux nfa next t
  in
  aux nfa start_state char_list

(*******************************)
(* Part 2: Subset Construction *)
(*******************************)

let new_states (nfa: ('q,'s) nfa_t) (qs: 'q list) : 'q list list =
  List.map (fun sym -> e_closure nfa (move nfa qs (Some sym))) nfa.sigma

let new_trans (nfa: ('q,'s) nfa_t) (qs: 'q list) : ('q list, 's) transition list =
  List.map(fun sym -> (qs, Some sym, e_closure nfa (move nfa qs (Some sym)))) nfa.sigma

let new_finals (nfa: ('q,'s) nfa_t) (qs: 'q list) : 'q list list =
  if List.exists (fun q -> List.mem q nfa.fs) qs then [qs] else []

let rec nfa_to_dfa_step (nfa: ('q,'s) nfa_t) (dfa: ('q list, 's) nfa_t)
    (work: 'q list list) : ('q list, 's) nfa_t =
  match work with
  | [] -> dfa
  | s::rest ->
    let new_transitions = new_trans nfa s in
    let unvisited_states = List.filter (fun state -> not (mem state dfa.qs)) (new_states nfa s) in

    let new_dfa = {
      sigma = dfa.sigma;
      qs = union dfa.qs unvisited_states;
      q0 = dfa.q0;
      fs = union dfa.fs (new_finals nfa s);
      delta = union dfa.delta new_transitions;
    } in
    union rest unvisited_states
    |> nfa_to_dfa_step nfa new_dfa

let nfa_to_dfa (nfa: ('q,'s) nfa_t) : ('q list, 's) nfa_t =
  let initial_state = e_closure nfa [nfa.q0] in
  let dfa = {
    sigma = nfa.sigma;
    qs = [initial_state];
    q0 = initial_state;
    fs = if exists (fun q -> mem q nfa.fs) initial_state then [initial_state] else [];
    delta = [];
  } in
  nfa_to_dfa_step nfa dfa [initial_state]