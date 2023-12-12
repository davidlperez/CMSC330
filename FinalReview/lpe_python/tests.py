from lpe_python.lexer import lex
from lpe_python.parser import parser
from lpe_python.eval import eval

# Lexer
def test_lexer():
    assert(lex("1 + 2") == ['1', '+', '2']) 
    assert(lex("1+2") == ['1', '+', '2']) 
    assert(lex("1 * (3 - 4)") == ['1', '*', '(', '3', '-', '4', ')']) 
    assert(lex("1 * 3 - 4)") == ['1', '*', '3', '-', '4', ')'])
    assert(lex("1 * (3 - 4") == ['1', '*', '(', '3', '-', '4'])
    assert(lex("+ 1 2") == ['+', '1', '2'])
    assert(lex("") == [])
    assert(lex("-3") == ['-3'])
    assert(lex("- 3") == ['-','3'])
    assert(lex("1-3") == ['1', '-3'])
    assert(lex("sq 2") == ['sq', '2'])

    # things that shouldn't work
    try:
      (lex("a")) 
    except SyntaxError:
      assert(True)
    except:
      assert(False)
    try:
      (lex("1 + ?3")) 
    except SyntaxError:
      assert(True)
    except:
      assert(False)

# Parser
def test_parser():
    assert(str(parser(lex("1 + 2"))) == "+ 1 2")
    assert(str(parser(lex("1 + 2 - 3"))) == "+ 1 - 2 3")
    assert(str(parser(lex("1 * 2 - 3"))) == "- * 1 2 3")
    assert(str(parser(lex("1 * (2 - 3)"))) == "* 1 - 2 3")
    assert(str(parser(lex("(2 + (3 - 2)) + 1"))) == "+ + 2 - 3 2 1")
    assert(str(parser(lex("2"))) == "2")
    assert(str(parser(lex("sq 2"))) == "sq 2")

    #things that should not work
    try:
      parser(lex("1 * (2 - 3)")) 
    except SyntaxError:
      assert(True)
    except:
      assert(False)
    try:
      (parser(lex("1 * + 2")))
    except SyntaxError:
      assert(True)
    except:
      assert(False)
    try:
      (parser(lex("()")))
    except SyntaxError:
      assert(True)
    except:
      assert(False)
    try:
     (parser(lex("1 * + 2")))
    except SyntaxError:
      assert(True)
    except:
      assert(False)

def test_eval():
    # Evaluator
    assert(eval(parser(lex("4 + 2")))==6)
    assert(eval(parser(lex("4 * 2")))==8)
    assert(eval(parser(lex("4 / 2")))==2.0)
    assert(eval(parser(lex("4 - 2")))==2)
    assert(eval(parser(lex("4 * (2 - 3)")))==-4)
    assert(eval(parser(lex("4 * 2 - -3")))==11)
    assert(eval(parser(lex("1 + sq 2")))==5)