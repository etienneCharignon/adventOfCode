class BingoBoard:
    """Bingo board"""

    def __init__(self):
        self.data = []
        self.x = 0
        self.y = 0
        self.width = 0
        self.height = 0

    def addNumber(self, value):
        self.data.append([self.x, self.y, value, 0])
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
        for cell in [cell for cell in self.data if number == cell[2]]:
            cell[3] = 1

    def hasRow(self):
        for y in range(0, self.height):
            wonRow = [cell for cell in self.data if cell[0] in range(0, self.width) and cell[1] == y and cell[3] == 1]
            if(len(wonRow) == self.width):
                return True
        return False

    def hasColumn(self):
        for x in range(0, self.width):
            wonCol = [cell for cell in self.data if cell[0] == x and cell[1] in range(0, self.height) and cell[3] == 1]
            if(len(wonCol) == self.width):
                return True
        return False

    def score(self):
        score = 0
        for cell in [cell for cell in self.data if cell[3] == 0]:
            score += int(cell[2])
        return score
