open P5.Nfa
open P5.Regexp
open TestUtils
open OUnit2

let test_placeholder _ =
  assert_equal true true

let suite =
  "student"
  >::: [ "nfa_new_states" >:: test_placeholder ]

let _ = run_test_tt_main suite
