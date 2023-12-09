import re
from functools import reduce

def read_codons(codon_file):
  # This will open a file with a given path (codon_file)
  file = open(codon_file)
  pattern = r'^([A-Z][a-zA-Z]*):\s*((([AGUC](\{\d+\})?)(,\s)?)+)$'

  global codon_data
  codon_data = {}
  # Iterates through a file, storing each line in the line variable
  for line in file:
    line = line.strip()
    match = re.match(pattern, line)
    
    if match:
      amino_acid = match.group(1)
      sequences = match.group(2).split(', ')
      codon_seq = []
      
      for sequence in sequences:
        seq_match = re.match(r'([AGUC])(\{(\d+)\})?', sequence)
        if seq_match:
          codon_seq.append(expand_seq(sequence))

      codon_data[amino_acid] = codon_seq

# This helper function will expand a codon sequence with a bracketed number to the full repeated sequence
# For example, A{3} will be expanded to AAA
def expand_seq(sequence):
    # Regular expression pattern to match the short form
    pattern = r'([AGUC])\{(\d+)\}'
    
    # Function to replace each match with the expanded form
    def replace(match):
        letter = match.group(1)
        count = int(match.group(2))
        return letter * count
    
    # Use re.sub to replace matches with the expanded form
    expanded = re.sub(pattern, replace, sequence)
    
    return expanded


def read_evals(eval_file):
  # This will open a file with a given path (eval_file)
  file = open(eval_file)
  global orders_data
  orders_data = {}
  pattern = r'^([a-zA-Z0-9]+):(\s)*(L|R),(\s)*(PO|PR|I)$'
  # Iterates through a file, storing each line in the line variable
  for line in file:
    line.strip()
    match = re.match(pattern, line)

    if match:
      order = match.group(1)
      direction = match.group(3)
      operation = match.group(5)
      orders_data[order] = [direction, operation]
  
def encode(sequence):
  global codon_data
  
  pattern = r'([a-zA-Z]+)'
  sequence = sequence.strip()
  matchall = re.findall(pattern, sequence)
  

  if matchall:
    codon_seq = ''
    for match in matchall:
      longest = ''
      if match in codon_data:
        for codon in codon_data[match]:
          if len(codon) > len(longest):
            longest = codon
        codon_seq += longest
    return codon_seq.strip()


def decode(sequence):
  global codon_data
  decoded = []

  while sequence:
    max_match_len = 0
    matched_amino = ''

    for amino, codons in codon_data.items():
      for codon in codons:
        codon_len = len(codon)
        if sequence.startswith(codon) and codon_len > max_match_len:
          max_match_len = codon_len
          matched_amino = amino

    if matched_amino:
      decoded.append(matched_amino)
      sequence = sequence[max_match_len:]
    else:
      sequence = sequence[1:]

  return ' '.join(decoded)

def operate(sequence, eval_name):
  global orders_data, codon_data

  if eval_name not in orders_data:
    return None
  result = ''
  direction, operation = orders_data[eval_name]
  if direction == 'R':
    sequence = sequence[::-1]

  decoded = decode(sequence).split()

  def exchange(idx):
    amino = decoded[idx]
    possible_codons = []
    for codon in codon_data[amino]:
      if codon not in possible_codons:
        possible_codons.append(codon)
    for codon in possible_codons:
      if encode(amino) == codon:
        possible_codons.remove(codon)
        if possible_codons:
          return possible_codons[0]
        break

  start = False  
  i = 0
  while i < len(decoded):
    if decoded[i] == "START":
      i += 1
      start = True
      continue
    elif decoded[i] == "STOP" and start:
      break
    elif start:
      match operation:
        case 'PO':
          match decoded[i]:
            case 'DEL':
              prev = encode(decoded[i - 1])
              result = result[:-len(prev)]
              decoded.pop(i - 1)
            case 'EXCHANGE':
              prev = encode(decoded[i - 1])
              result = result[:-len(prev)]
              result += exchange(i - 1)
              i += 1
            case 'SWAP':
              if i > 1:
                prev1 = encode(decoded[i - 1])
                prev2 = encode(decoded[i - 2])
                result = result[:-len(prev1) - len(prev2)]
                decoded[i - 2], decoded[i - 1] = decoded[i - 1], decoded[i - 2]
                result += encode(decoded[i - 2]) + encode(decoded[i - 1])
                i += 1
              else:
                i += 1
            case _:
              result += encode(decoded[i])
              i += 1
        case 'PR':
          match decoded[i]:
            case 'DEL':
              decoded.pop(i + 1)
              i += 1
            case 'EXCHANGE':
              result += exchange(i + 1)
              i += 2
              continue
            case 'SWAP':
              decoded[i + 1], decoded[i + 2] = decoded[i + 2], decoded[i + 1]
              i += 1
            case _:
              result += encode(decoded[i])
              i += 1
        case 'I':
          match decoded[i]:
            case 'DEL':
              decoded.pop(i + 1)
              i += 1
            case 'EXCHANGE':
              result += exchange(i + 1)
              i += 2
            case 'SWAP':
              prev = encode(decoded[i - 1])
              result = result[:-len(prev)]
              decoded[i + 1], decoded[i - 1] = decoded[i - 1], decoded[i + 1]
              result += encode(decoded[i - 1])
              i += 1
            case _:
              result += encode(decoded[i])
              i += 1
    else:
      i += 1        

  return result