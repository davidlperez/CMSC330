'''

CFG
E -> M + E|M - E|M
M -> N * M|N / M|sq M|N
N -> n|(E)
where n is any integer
'''

import re
from lexer import lex


class Node:
  def __init__(self,v,t=None,left=None,right=None):
    self.type = t
    self.value = v
    self.left = left
    self.right = right

  def __str__(self):
    ret = str(self.value) 
    if self.left:
      ret += " " + str(self.left)
    if self.right:
      ret += " " + str(self.right)
    return ret
  
def parser(toklst):
  tree, remain = parse_e(toklst)
  if remain == []:
    return tree
  else:
    raise Exception("Tokens Left: " + str(remain))
  
def parse_e(toklst):
  mtree, remain = parse_m(toklst)
  if len(remain) > 0 and remain[0] in ['+','-']:
    etree, new_remain = parse_e(remain[1:]) 
    return Node(remain[0], "op", mtree, etree), new_remain
  return mtree, remain

def parse_m(toklst):
  if len(toklst) > 0 and toklst[0] == 'sq':
    arg, new_remain = parse_m(toklst[1:])
    return Node(toklst[0], "op", arg), new_remain
  ntree, remain = parse_n(toklst)
  if len(remain) > 0 and remain[0] in ['*','/']:
    mtree, new_remain = parse_m(remain[1:]) 
    return Node(remain[0], "op", ntree, mtree), new_remain
  return ntree, remain

def parse_n(toklst):
  if len(toklst) > 0:
    fst = toklst[0]
  else:
    raise SyntaxError("Empty")
  if fst == "(":
    etree, remain = parse_e(toklst[1:])
    if len(remain) > 0 and remain[0] == ")":
      return etree, remain[1:]
    else:
      raise SyntaxError("Unbalanced Parenthesis")
  else:
    try:
      int(fst)
    except:
      raise SyntaxError("Not an integer")
    return Node(fst), toklst[1:]
  

print(parser(lex("1+2")))