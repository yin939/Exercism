class Matrix:
    def __init__(self, matrix_string):
        self.matrix = []
        for val in matrix_string.split('\n'):
            val = val.split(' ')
            self.matrix.append([int(num) for num in val])

    def row(self, index):
        return self.matrix[index - 1]

    def column(self, index):
        return [arr[index - 1] for arr in self.matrix]
