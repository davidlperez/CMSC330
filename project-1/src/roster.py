# You should write all four classes in this file
class Person:
   
    def __init__(self, name, age):
        self.name = name
        self.age = age

    def get_age(self):
        return self.age
    
    def set_age(self, x):
        self.age = x

class Student(Person):
    
    def __init__(self, name, age, grade):
        Person.__init__(self, name, age)
        self.grade = grade

    def get_grade(self):
        return self.grade
    
    def change_grade(self, x):
        self.grade = x

class Staff(Person):
    def __init__(self, name, age, position):
        Person.__init__(self, name, age)
        self.position = position

    def get_position(self):
        return self.position
    
    def change_position(self, newPosition):
        self.position = newPosition

class Roster:

    def __init__(self):
        self.roster = []
    
    def add(self, person):
        self.roster.append(person)

    def size(self):
        return len(self.roster)
    
    def remove(self, Person):
        self.roster.remove(Person)

    def get_person(self, name):
        for person in self.roster:
            if person.name == name:
                return person
        return None
    
    def map(self, f):
        for person in self.roster:
            f(person)
    
