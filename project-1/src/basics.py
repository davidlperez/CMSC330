def isPalindrome(n):
  number = str(n)
  return number == number[::-1]  

def nthmax(n, a):
  a.sort(reverse=True)
  
  if n >= len(a):
    return None
  
  return a[n]

def freq(s):
  if not s:
    return ""
  
  char_count = {}
  for char in s:
    if char in char_count:
      char_count[char] += 1
    else:
      char_count[char] = 1
    
    max_char = max(char_count, key=char_count.get)
  return max_char

def zipHash(arr1, arr2):
  if len(arr1) != len(arr2):
    return None
  
  if not arr1 or not arr2:
    return {}
  
  hash = {}
  for i in range(len(arr1)):
    hash[arr1[i]] = arr2[i]
  return hash

def hashToArray(hash):
  if not hash:
    return []
  
  result = []
  for key in hash.keys():
      result.append([key, hash[key]])
  return result

def maxLambdaChain(init, lambdas):
  if not lambdas:
    return init
  
  result1 = lambdas[0](init)
  result2 = maxLambdaChain(result1, lambdas[1:])
  result3 = maxLambdaChain(init, lambdas[1:])

  return max(init, result1, result2, result3)