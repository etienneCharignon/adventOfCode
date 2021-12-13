from inputj13 import sample_dots, input_dots, sample_folds, input_folds


def parse_dots(input):
    def to_dot(pair):
        x, y = pair.split(',')
        return (int(x), int(y))

    return list(map(to_dot, input.split('\n')))


def fold(transparent, direction, index):
    def fold(dot):
        x, y = dot
        if(direction == 1):
            if(y > index):
                return (x, y - (y - index) * 2)
        else:
            if(x > index):
                return (x - (x - index) * 2, y)
        return dot

    return set(map(fold, transparent))


def print_it(transparent):
    max_x = max([x for x, y in transparent]) + 1
    max_y = max([y for x, y in transparent]) + 1
    screen = [None] * max_y
    for row in range(0, max_y):
        screen[row] = [' '] * max_x

    for x, y in transparent:
        screen[y][x] = '#'

    for row in screen:
        print(''.join(row))
    return "x"


def fold_all(transparent, folds):
    for afold in folds:
        transparent = fold(transparent, afold[0], afold[1])
    return transparent


def test_pars_dots():
    assert parse_dots('6,10\n0,14') == [(6, 10), (0, 14)]


def test_print_transparent():
    assert print_it(parse_dots(sample_dots)) == "x"


def test_fold():
    assert fold(parse_dots('6,10\n0,14\n0,3'), 1, 7) == set([(6, 4), (0, 0), (0, 3)])
    assert fold(parse_dots('6,10\n5,14\n0,3'), 0, 3) == set([(0, 10), (1, 14), (0, 3)])


def test_count_dots():
    assert len(fold(parse_dots(sample_dots), 1, 7)) == 17
    assert len(fold(parse_dots(input_dots), 0, 655)) == 706


def test_fold_all():
    assert print_it(fold_all(parse_dots(sample_dots), sample_folds)) == "x"
    assert print_it(fold_all(parse_dots(input_dots), input_folds)) == "x"
