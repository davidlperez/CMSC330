# Discussion 1

## Reminders

- Project 0 due Sunday, September 3, 2023 at 11:59 PM

## Python

### Indentation

**INDENTATION MATTERS!**

Not just for readability in Python, your code will not run if you don’t indent properly.

```python
# BAD
foo = 5
if foo == 5:
print(foo)

# GOOD
foo = 5
if foo == 5:
	print(foo)
```

**CONSISTENCY MATTERS!** Choose either tabs or spaces and keep it that way (if using spaces, always use the same number of spaces!) to avoid the dreaded `IndentationError`.

### `global` Keyword

The `global` keyword is necessary to prevent errors in your code if you want to specifically modify a top-level variable.

```python
foo = 5

def blah():
	foo = foo + 2
	return foo
	
def blah2():
	global foo # brings foo variable into scope
	foo = foo + 2
	return foo
	
print(blah()) # UnboundLocalError
print(blah2()) # prints 7
```

### Classes

Python is an object-oriented language like Java. We can declare classes like this:

```python
class Person:
	# constructor
	def __init__(self, fname, lname):
		self.fname = fname # instance variable
		self.lname = lname

	def get_name(self):
		return self.fname + " " + self.lname
```

### Nested Functions

We can declare functions inside functions.

```python
def foo():
	y = 1

	def bar(x):
		return x + y + 2
	
	print(bar(39)) # prints 42

print(bar(1)) # invalid
```

### Dynamic Typing

Python assigns types at runtime.

```python
# note variable "a" changes its type
a = 5
b = "hello"
a = "bye"

'''
static typing is also available when explicitly stated.
it's called type hints, but python does not enforce type hints,
so below code will run fine
'''
a: int = 32
a = "hello"

# can do this with functions as well
def add_numbers(a: int, b: int) -> int:
	return a + b

add_numbers(1, 2) # valid
add_numbers("Hello", "Bye") # also valid, but bad practice
```

## Rectangle Class

We will now implement a simple class in Python in order to highlight fundamental OOP concepts. 

A `Rectangle` represents a rectangle shape.  The methods below will be implemented in the `Rectangle` class in [rectangle.py](src/rectangle.py).

#### `__init__(self, width, height):`

- **Type**: `(int, int) -> _`
- **Description**: Given a length and width for the rectangle, store them in the `Rectangle` object.  You should perform any initialization steps for the `Rectangle` instance here. Note that we must keep track of the number of `Rectangle` objects that have been instantiated. 
- **Examples**:
  ```py
  square = Rectangle(5, 5)
  rectangle = Rectangle(2, 3)
  ```

#### `get_area(self):`

- **Type**: `(self) -> int`
- **Description**: Return the area of a `Rectangle`
- **Examples**:
  ```py
  square = Rectangle(5, 5)
  rectangle = Rectangle(2, 3)

  print(square.get_area())              # prints 25
  print(rectangle.get_area())           # prints 6
  ```

#### `get_num_rectangles():`

- **Type**: `() -> int`
- **Description**: Return the number of `Rectangles`
- **Examples**:
  ```py
  rec1 = Rectangle(5, 5)
  rec2 = Rectangle(2, 3)

  print(Rectangle.get_num_rectangles())  # prints 2

  rec3 = Rectangle(4, 1)
  rec4 = Rectangle(3, 3)

  print(Rectangle.get_num_rectangles())  # prints 4
  ```

---

## Intro to Git

### What is Git?

1. Git is an open-source version control system
2. Make sure to [set up an SSH key](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent) (refer to Project 0 for detailed instructions on how to do this).

### Navigate to Class GitHub (located on the [class page](https://bakalian.cs.umd.edu/330))

1. Click on Project 0
2. Accept Project 0

### Cloning the Repository

1. `git clone <insert link to your repo here>`
2. Go into your new repository: `cd <repo name>`

### Staging, Committing, and Pushing Changes

1. Make text file in the repository directory on your machine: `touch hello.txt`
2. Add: `git add .`  (the `.` after `git add` refers to the entire current directory, you can also add specific files/directories like so: `git add <file.txt>`)
3. Commit: `git commit -m “Add new text file”`
4. Push: `git push`

### Submitting

You can either submit from the base directory using the `submit` script, or by manually submitting on Gradescope.

### Pulling Changes

1. Edit text file on GitHub website (in your browser)
2. Pull: `git pull`

