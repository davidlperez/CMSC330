from functools import reduce
def is_present(lst,x):
  return x in lst

def count_occ(lst,target):
  return len(list(filter(lambda x: x == target, lst)))

def uniq(lst):
  answer = []
  for item in lst:
    if not is_present(answer,item):
      answer.append(item)
  return answer

def find_max(matrix):
  answer = 0
  for row in matrix:
    for item in row:
      if item > answer:
        answer = item
  return answer

def count_ones(matrix):
  answer = 0
  for row in matrix:
    for item in row:
      if item == 1:
        answer += 1
  return answer

def addgenerator(x):
  return lambda y: x+y

def apply_to_self():
  return lambda element, f: element + f(element)

def ap(fns,args):
  raise Exception("Not Implemented")

def map2(matrix,f):
  answer = []
  for row in matrix:
    answer.append(list(map(f,row)))
  return answer
