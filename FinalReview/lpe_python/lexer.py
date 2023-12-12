'''
CFG
E -> M + E|M - E|M
M -> N * M|N / M|sq M|N
N -> n|(E)
where n is any integer
'''

from functools import reduce
import re

def lex(instr):
  toklst = []
  number_re = re.compile(r"^(-?\d+)")
  terminal_re = re.compile(r"^(sq|[()\-+/*])")
  wspace_re = re.compile(r"^(\s+)")
  pos = 0
  strlen = len(instr)
  while pos < strlen:
    match = re.match(number_re,instr[pos:])
    if match:
      toklst.append(match.group(1))
      pos += len(match.group(1))
    else:
      match = re.match(terminal_re,instr[pos:])
      if match:
        toklst.append(match.group(0))
        pos += len(match.group(0))
      else:
        match = re.match(wspace_re,instr[pos:])
        if match:
          pos += len(match.group(1))
        else:
          raise SyntaxError("Invalid character")
  return toklst