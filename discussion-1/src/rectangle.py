class Rectangle :
    
    num_rectangles = 0

    def __init__(self, width, height):
        self.width = width
        self.height = height
        Rectangle.num_rectangles += 1

    def get_area(self):
        return self.width * self.height

    def get_num_rectangles():
        return Rectangle.num_rectangles

# test get_area
square = Rectangle(5, 5)
rectangle = Rectangle(2, 3)

print(square.get_area())              # prints 25
print(rectangle.get_area())           # prints 6

# test get_num_rectangles
rec1 = Rectangle(5, 5)
rec2 = Rectangle(2, 3)

print(Rectangle.get_num_rectangles())  # prints 2

rec3 = Rectangle(4, 1)
rec4 = Rectangle(3, 3)

print(Rectangle.get_num_rectangles())  # prints 4