target_area = ((124, 174), (-123, -86))
sample_target = ((20, 30), (-10, -5))
sample_vx_vy = [
    (23, -10), (25, -9), (27, -5), (29, -6), (22, -6), (21, -7), (9, 0), (27, -7), (24, -5),
    (25, -7), (26, -6), (25, -5), (6, 8), (11, -2), (20, -5), (29, -10), (6, 3), (28, -7),
    (8, 0), (30, -6), (29, -8), (20, -10), (6, 7), (6, 4), (6, 1), (14, -4), (21, -6),
    (26, -10), (7, -1), (7, 7), (8, -1), (21, -9), (6, 2), (20, -7), (30, -10), (14, -3),
    (20, -8), (13, -2), (7, 3), (28, -8), (29, -9), (15, -3), (22, -5), (26, -8), (25, -8),
    (25, -6), (15, -4), (9, -2), (15, -2), (12, -2), (28, -9), (12, -3), (24, -6), (23, -7),
    (25, -10), (7, 8), (11, -3), (26, -7), (7, 1), (23, -9), (6, 0), (22, -10), (27, -6),
    (8, 1), (22, -8), (13, -4), (7, 6), (28, -6), (11, -4), (12, -4), (26, -9), (7, 4),
    (24, -10), (23, -8), (30, -8), (7, 0), (9, -1), (10, -1), (26, -5), (22, -9), (6, 5),
    (7, 5), (23, -6), (28, -10), (10, -2), (11, -1), (20, -9), (14, -2), (29, -7), (13, -3),
    (23, -5), (24, -8), (27, -9), (30, -7), (28, -5), (21, -10), (7, 9), (6, 6), (21, -5),
    (27, -10), (7, 2), (30, -9), (21, -8), (22, -7), (24, -9), (20, -6), (6, 9), (29, -5),
    (8, -2), (27, -8), (30, -5), (24, -7)
]


def print_screen(points):
    screen = []
    min_y = min(points, key=lambda point: point[1])[1]
    max_y = max(points, key=lambda point: point[1])[1]
    max_x = max(points, key=lambda point: point[0])[0] + 1
    number_of_y = abs(min_y) + max_y + 1

    print((max_x, min_y, max_y))
    for row in range(min_y, max_y + 1):
        screen.append(['.'] * max_x)

    for row in range(0, number_of_y):
        print(''.join(screen[row]))

    for point in points:
        x, y, char = point
        print(x)
        print(y)
        screen[-1 * y][x] = char

    for row in range(0, number_of_y):
        print(''.join(screen[row]))


def init_screen(target):
    screen = [(0, 0, 'S')]
    for row in range(target[1][0], target[1][1]):
        for col in range(target[0][0], target[0][1]):
            screen.append((col, row, 'T'))
    return screen


def fx(v):
    return 0.5 * v * v + 0.5 * v


def xtest_display_game():
    print_screen(init_screen(sample_target))
    assert init_screen(sample_target) == [(0, 0, 'S'), (20, -10, " T")]


def find_vx(min_target_x, max_target_x):
    vmin = 1
    while(fx(vmin) < min_target_x):
        vmin += 1

    vmax = vmin
    while(fx(vmax) < max_target_x):
        vmax += 1
    return vmin, vmax - 1


def fxn(vx, n):
    return (n + 1) * vx - (n * n + n)/2


def fyn(vy, n):
    if(vy < 0):
        return fxn(vy, n)
    elif(vy == 0):
        return 0
    else:
        return (vy + 1) * vy - (n * n + n)/2


def findallx(vx):
    all_x = []
    v = vx
    x = v
    while(v > 0):
        all_x.append(x)
        v -= 1
        x += v

    return all_x


def findally(vy, miny):
    ally = []
    v = vy
    y = v
    while(v > 0):
        ally.append(y)
        v -= 1
        y += v

    while(y >= miny):
        ally.append(y)
        v -= 1
        y += v

    return ally


def find_vx_x(minx, maxx):
    all_vx_x = []
    for vx in range(1, maxx + 1):
        allx = findallx(vx)
        all_vx_x.append((vx, allx))

    return all_vx_x


def find_x_y(miny, maxy, minx, maxx):
    all_x_y = []
    for vy in range(miny, abs(miny)):
        for vx in range(1, maxx + 1):
            allx = findallx(vx)
            ally = findally(vy, miny)
            for n in range(0, len(ally)):
                if(n >= len(allx)):
                    x, y = allx[-1], ally[n]
                else:
                    x, y = allx[n], ally[n]
                if(x >= minx and x <= maxx and y >= miny and y <= maxy):
                    all_x_y.append((vx, vy))
                    break

    return all_x_y


def test_fx():
    assert find_vx(20, 30) == (6, 7)
    assert find_vx(124, 174) == (16, 18)


def test_find_x_n():
    assert len(find_vx_x(20, 30)) == 30
    assert len(find_vx_x(124, 174)) == 174


def test_find_y_n():
    assert len(find_x_y(-10, -5, 20, 30)) == 112
    # assert find_x_y(-10, -5, 20, 30) == sample_vx_vy
    # assert len(find_x_y(-123, -86, 124, 174)) == 3229


def test_findall():
    assert findally(-5, -10) == [-5]
#    assert findallx(30) == [30, 59, 87]
    assert findally(2, -10) == [2, 3, 3, 2, 0, -3, -7]
    assert findallx(7) == [7, 13, 18, 22, 25, 27, 28]
