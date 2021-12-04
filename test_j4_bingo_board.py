from j4_bingo_board import BingoBoard


def test_addNumber():
    board = BingoBoard()
    board.addNumber("12")
    assert board.data == [[0, 0, "12", 0]]
    board.addNumber("34")
    assert board.data == [[0, 0, "12", 0], [1, 0, "34", 0]]


def test_addRow():
    board = BingoBoard()
    board.addRow("12  4")
    board.addRow("56 78")
    assert board.data == [
        [0, 0, "12", 0],
        [1, 0, "4", 0],
        [0, 1, "56", 0],
        [1, 1, "78", 0]
    ]
    assert board.width == 2
    assert board.height == 2


def test_hasNumber():
    board = BingoBoard()
    board.addRow("12 34")
    board.hasNumber("12")
    assert board.data == [[0, 0, "12", 1], [1, 0, "34", 0]]


def test_hasNotNumber():
    board = BingoBoard()
    board.addRow("12 34")
    board.hasNumber("72")
    assert board.data == [[0, 0, "12", 0], [1, 0, "34", 0]]


def test_hasNotRow():
    board = BingoBoard()
    board.addRow("12 34")
    assert not board.hasRow()


def test_hasRow():
    board = BingoBoard()
    board.addRow("12 34")
    board.hasNumber("12")
    board.hasNumber("34")
    assert board.hasRow()


def test_hasRowInSecondPlace():
    board = BingoBoard()
    board.addRow("1 3")
    board.addRow("12 34")
    board.hasNumber("12")
    board.hasNumber("34")
    assert board.hasRow()


def test_hasColumn():
    board = BingoBoard()
    board.addRow("1 3")
    board.addRow("12 34")
    board.hasNumber("3")
    board.hasNumber("34")
    assert board.hasColumn()


def test_score():
    board = BingoBoard()
    board.addRow("1 3")
    board.addRow("12 34")
    board.hasNumber("3")
    board.hasNumber("34")
    assert board.score() == 13
