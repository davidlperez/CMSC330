'''
CFG
E -> M + E|M - E|M
M -> N * M|N / M|sq M|N
N -> n|(E)
where n is any integer
'''
from lpe_python.lexer import lex
from lpe_python.parser import parser

class Node:
  def __init__(self,v,t=None,left=None,right=None):
    self.type = t
    self.value = v
    self.left = left
    self.right = right

def eval(tree):
  if tree.type:
    leftv = eval(tree.left)
    if tree.value == "sq":
      return leftv * leftv
    else:
      rightv = eval(tree.right)
    if tree.value == "+":
      rightv = eval(tree.right)
      return leftv + rightv
    if tree.value == "-":
      return leftv - rightv
    if tree.value == "*":
      return leftv * rightv
    if tree.value == "/":
      return leftv / rightv
  return int(tree.value)