import re
from functools import reduce

class Fsm:
  def __init__(self,alphabet,states,start,final,transitions):
    self.sigma = alphabet
    self.states = states
    self.start = start
    self.final = final
    self.transitions = transitions
  def __str__(self):
    sigma = "Alphabet: " + str(self.sigma) + "\n"
    states = "States: " + str(self.states) + "\n"
    start = "Start: " + str(self.start) + "\n"
    final = "Final: " + str(self.final) + "\n"
    trans_header = "Transitions: [\n"
    thlen = len(trans_header)
    translist = ""
    for t in self.transitions:
      translist += " " * thlen + str(t)+ "\n"
    translist += " " * thlen + "]"
    transitions = trans_header + translist
    ret = sigma + states + start + final + transitions 
    return ret

count = 0

def fresh(): 
  global count
  count += 1
  return count

def char(string):
  start, final = fresh(), fresh()
  transitions = [(start,string,final)]
  return Fsm([string],[start,final],start,[final],transitions)

def concat(r1,r2):
  alphabet = r1.sigma + r2.sigma
  states = r1.states + r2.states
  start = r1.start
  final = r2.final[0]
  transitions = r1.transitions + r2.transitions
  for f in r1.final:
    transitions.append((f,"epsilon",r2.start))
  return Fsm(alphabet,states,start,[final],transitions)

def union(r1,r2):
  start, final = fresh(), fresh()
  alphabet = r1.sigma + r2.sigma
  states = r1.states + r2.states
  transitions = r1.transitions + r2.transitions
  transitions.append((start,"epsilon",r1.start))
  transitions.append((start,"epsilon",r2.start))
  for f in r1.final:
    transitions.append((f,"epsilon",final))
  for f in r2.final:
    transitions.append((f,"epsilon",final))
  return Fsm(alphabet,states,start,[final],transitions)

def star(r1):
  start, final = fresh(), fresh()
  alphabet = r1.sigma
  states = r1.states
  transitions = r1.transitions
  transitions.append((start,"epsilon",r1.start))
  transitions.append((start,"epsilon",final))
  for f in r1.final:
    transitions.append((f,"epsilon",r1.start))
    transitions.append((f,"epsilon",final))
  return Fsm(alphabet,states,start,[final],transitions)
  
def e_closure(s,nfa):
  x = set(s)
  
  while True:
    s = x.copy()
    x = x.union({dest for (src, transition, dest) in nfa.transitions if src in s and transition == "epsilon"})

    if s == x:
      break
  
  return list(x)

def move(c,s,nfa):
  answer = []
  for states in s:
    for transition in nfa.transitions:
      if transition[0] == states and transition[1] == c and transition[2] not in answer:
        answer.append(transition[2])
  return answer

def nfa_to_dfa(nfa): 
  dfa_states = [tuple(e_closure([nfa.start],nfa))]
  dfa_start = dfa_states[0]
  dfa_transitions = []
  dfa_finals = []

  visited = []

  while tuple(sorted(visited)) != tuple(sorted(dfa_states)):
    s = next(state for state in dfa_states if state not in visited)
    visited.append(s)

    for c in nfa.sigma:
      E = move(c,s,nfa)
      e = tuple(e_closure(E,nfa))

      if e not in dfa_states:
        dfa_states.append(e)

      dfa_transitions.append((s,c,e))

  for state in dfa_states:
    if any(f in state for f in nfa.final):
      dfa_finals.append(tuple(state))
  
  return Fsm(nfa.sigma,dfa_states,dfa_start,dfa_finals,dfa_transitions)

def accept(nfa,string):
  current_states = e_closure([nfa.start],nfa)

  for c in string:
    current_states = e_closure(move(c,current_states,nfa),nfa)
  
  return bool(set(current_states) & set(nfa.final))