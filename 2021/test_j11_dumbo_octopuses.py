from samplej11 import all9_input, rinput


def parse(input):
    rows = input.split('\n')
    field = []
    for row in rows:
        field.append(list(map(int, [energy for energy in row])))
    return field


def print_field(field):
    def joinCol(row):
        return ''.join(map(str, row))

    return '\n'.join(map(joinCol, field))


def increase(field):
    for row in range(0, len(field)):
        for col in range(0, len(field[row])):
            field[row][col] += 1
    return field


def flash_around(row, col, field):
    count = 0
    for r in range(row - 1, row + 2):
        for c in range(col - 1, col + 2):
            if((r == row and c == col) or r < 0 or c < 0 or r > (len(field) - 1) or c > (len(field[row]) - 1)):
                continue
            if(field[r][c] != 0):
                field[r][c] += 1
                if(field[r][c] > 9):
                    field[r][c] = 0
                    count += 1
                    (field, around_count) = flash_around(r, c, field)
                    count += around_count
    return (field, count)


def flash(field):
    count = 0
    for row in range(0, len(field)):
        for col in range(0, len(field[row])):
            if(field[row][col] > 9):
                field[row][col] = 0
                count += 1
                (field, around_count) = flash_around(row, col, field)
                count += around_count
    return (field, count)


def all_flashes(field):
    def joinCol(row):
        return sum(row)

    return sum(map(joinCol, field)) == 0


def test_parse_input():
    assert parse(all9_input) == [[1, 1, 1, 1, 1], [1, 9, 9, 9, 1], [1, 9, 1, 9, 1], [1, 9, 9, 9, 1], [1, 1, 1, 1, 1]]
    assert print_field([[1, 2], [3, 4]]) == "12\n34"


def test_increase_energy():
    field = parse("111\n222\n999")
    assert print_field(increase(field)) == "222\n333\n101010"


def test_flash():
    field = parse("111\n222\n999")
    newField, flashes = flash(increase(field))
    assert print_field(newField) == "222\n565\n000"
    assert flashes == 3
    newField, flashes = flash(increase(parse(all9_input)))
    assert flashes == 9
    assert print_field(newField) == """34543
40004
50005
40004
34543"""


def test_count_flash_input():
    field = parse(rinput)
    count = 0
    for step in range(0, 1000):
        field = increase(field)
        (field, flashes) = flash(field)
        if(all_flashes(field)):
            assert step + 1 == 346
            return
        count += flashes
    assert count == 1694
