class BingoBoard:
    """Bingo board"""

    def __init__(self):
        self.data = []
        self.x = 0
        self.y = 0
        self.width = 0
        self.height = 0

    def addNumber(self, value):
        self.data.append((self.x, self.y, value, 0))
        self.x += 1

    def addRow(self, row):
        cells = row.split()
        for number in cells:
            self.addNumber(number)
        self.x = 0
        self.y += 1
        self.width = len(cells)
        self.height = self.y

    def hasNumber(self, number):
        def checkCell(cell):
            x, y, v, s = cell
            if (number == v):
                return (x, y, v, 1)
            else:
                return (x, y, v, s)
        self.data = list(map(checkCell, self.data))

    def hasRow(self):
        for row in range(0, self.height):
            wonCells = [v for (x, y, v, s) in self.data if x in range(0, self.width) and y == row and s == 1]
            if(len(wonCells) == self.width):
                return True
        return False

    def hasColumn(self):
        for col in range(0, self.width):
            wonCells = [v for (x, y, v, s) in self.data if x == col and y in range(0, self.height) and s == 1]
            if(len(wonCells) == self.height):
                return True
        return False

    def score(self):
        return sum([int(v) for (x, y, v, s) in self.data if s == 0])
