open Funs

(***********************************)
(* Part 1: Non-Recursive Functions *)
(***********************************)

let rev_tup (a,b,c) = (c,b,a);;

let is_even x = x mod 2 = 0;;

let volume (a,b,c) (x,y,z) = abs ((x-a)*(y-b)*(z-c));;

(*******************************)
(* Part 2: Recursive Functions *)
(*******************************)

let rec fibonacci n = 
  if n <= 1 then n 
  else fibonacci (n-1) + fibonacci (n-2);;

let rec log x y =
  if y < x then 0
  else 1 + log x (y/x);;

let rec gcf x y = 
  if y = 0 then x
  else gcf y (x mod y);;

let rec maxFuncChain init funcs = 
  match funcs with
  | [] -> init
  | hd::tl ->
      let result1 = hd init in
      let result2 = maxFuncChain result1 tl in
      let result3 = maxFuncChain init tl in
      max init (max result1 (max result2 result3));;

(*****************)
(* Part 3: Lists *)
(*****************)

let rec reverse lst = 
  let rec aux acc = function
    | [] -> acc
    | hd :: tl -> aux (hd :: acc) tl
  in
  aux [] lst;;

let rec zip lst1 lst2 = 
  match lst1, lst2 with
  | (a,b)::tl1, (c,d)::tl2 -> (a, b, c, d) :: zip tl1 tl2
  | _, _ -> [];;

let rec is_palindrome lst = 
  let rec reverse acc = function
    | [] -> acc
    | hd::tl -> reverse (hd::acc) tl
  in
  
  let rec equal_list lst1 lst2 = 
    match lst1, lst2 with
    | [], [] -> true
    | hd1::tl1, hd2::tl2 -> (hd1 = hd2) && equal_list tl1 tl2
    | _, _ -> false
  in
  equal_list lst (reverse [] lst)

let is_prime n =
  let rec aux d =
    if d * d > n then true
    else if n mod d = 0 then false
    else aux (d + 1)
  in
  if n <= 1 then false
  else aux 2;; 

let rec square_primes lst =
  match lst with
  | [] -> []
  | hd::tl ->
      if is_prime hd then
        (hd, hd * hd) :: square_primes tl
      else
        square_primes tl;;       

let rec partition p lst = 
  match lst with
  | [] -> ([], [])
  | hd::tl ->
      let (satisfy, dont_satisfy) = partition p tl in
      if p hd then
        (hd::satisfy, dont_satisfy)
      else
        (satisfy, hd::dont_satisfy);;

(*****************)
(* Part 4: HOF *)
(*****************)

let is_present lst x =
  let f y = if y = x then 1 else 0 in
  map f lst;;

let count_occ lst target =
  let f acc x = if x = target then acc + 1 else acc in
  fold f 0 lst;;
  
let jumping_tuples lst1 lst2 = 
  let zipped = zip lst1 lst2 in
  
  let first_half (idx, acc) (a,b,c,d) =
    if idx mod 2 = 0 then (idx+1, acc @ [d])
    else (idx+1, acc @ [a])
  in
  let second_half (idx, acc) (a,b,c,d) =
    if idx mod 2 = 0 then (idx+1, acc @ [a])
    else (idx+1, acc @ [d])
  in
  let (_, fst) = fold first_half (0, []) zipped
  in
  let (_, ans) = fold second_half (0, fst) zipped
  in
  ans;;

let addgenerator x = 
  let sum = fold (+) 0 [x] in
  fun y -> sum + y;;

let uniq lst =
  let add_if_not_present acc x =
    if fold (fun found y -> found || x = y) false acc then acc else x::acc
  in
  fold add_if_not_present [] lst;;

let ap fns args =
  let apply_one_fn fn = map fn args in
  let all_results = map apply_one_fn fns in
  fold (@) [] all_results;;