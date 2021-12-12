from inputj12 import sample1, sample2, input


def find_paths(caves, connexions, path_to_there):
    paths = []
    print(caves)
    for cave in caves:
        if(cave not in connexions):
            paths.append([cave])
        elif(cave.isupper() or cave not in path_to_there):
            new_path_to_there = path_to_there + [cave]
            sub_paths = find_paths(connexions[cave], connexions, new_path_to_there)
            for sub_path in sub_paths:
                sub_path.insert(0, cave)
                paths.append(sub_path)
    return paths


def count_path(connexions):
    paths = find_paths(['start'], connexions, [])
    paths = [path for path in paths if path[-1] == 'end']
    print(paths)
    return len(paths)


def add_connexion(f, t, connexions):
    if(t == 'start' or f == 'end'):
        return
    if(f not in connexions):
        connexions[f] = []
    connexions[f].append(t)


def as_map(input):
    connexions = {}
    for row in input.split('\n'):
        de, a = row.split('-')
        add_connexion(de, a, connexions)
        add_connexion(a, de, connexions)
    return connexions


def test_count_paths():
    assert(count_path(as_map(sample1))) == 10
    assert(count_path(as_map(sample2))) == 226
    assert(count_path(as_map(input))) == 226


def test_as_map():
    assert(as_map('start-A\nstart-b\nA-b\nA-end')) == {
        'start': ['A', 'b'],
        'b': ['A'],
        'A': ['b', 'end']
    }
