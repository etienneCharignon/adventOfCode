from inputj4 import random, boards
# from samplej4 import random, boards
from j4_bingo_board import BingoBoard

random = random.split(',')
boards_inputs = boards.split('\n')

boards = []

for row in boards_inputs:
    if (row == ''):
        board = BingoBoard()
        boards.append(board)
    else:
        board.addRow(row)

for number in random:
    winningBoads = []
    for board in boards:
        board.hasNumber(number)
        if(board.hasRow() or board.hasColumn()):
            winner = int(number)
            print(f"number : {winner}")
            print(int(number) * board.score())
            winningBoads.append(board)
    for winningBoard in winningBoads:
        boards.remove(winningBoard)
