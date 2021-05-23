from typing import List


class Garden:
    plant = {
        'G': 'Grass',
        'C': 'Clover',
        'R': 'Radishes',
        'V': 'Violets'
    }

    defaultName: List[str] = ['Alice', 'Bob', 'Charlie', 'David',
                              'Eve', 'Fred', 'Ginny', 'Harriet',
                              'Ileana', 'Joseph', 'Kincaid', 'Larry']

    def __init__(self, diagram: str, students: List[str] = defaultName):
        self.diagram = diagram.split('\n')
        self.students = students

    def plants(self, name):
        if name in self.defaultName:
            inx = (ord(name[0]) - ord('A')) * 2
            return self.helper(inx)

        self.students.sort()
        inx = self.students.index(name) * 2
        return self.helper(inx)

    def helper(self, inx: int):
        ans = []
        row1 = self.diagram[0][inx: inx+2]
        row2 = self.diagram[1][inx: inx+2]
        for c in row1:
            ans.append(self.plant[c])
        for c in row2:
            ans.append(self.plant[c])

        return ans
