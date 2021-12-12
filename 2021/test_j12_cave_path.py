from inputj12 import sample1, sample2
# from inputj12 inport input


def is_authorised_visit(visited):
    small_visited_caves = [cave for cave in visited if cave.islower()]
    return (len(small_visited_caves) - len(set(small_visited_caves))) < 2


def find_paths(caves, connexions, visited):
    paths = []
    for cave in caves:
        to_be_visited = visited + [cave]
        if(cave not in connexions):
            paths.append([cave])
        elif(cave.isupper() or is_authorised_visit(to_be_visited)):
            sub_paths = find_paths(connexions[cave], connexions, to_be_visited)
            for sub_path in sub_paths:
                sub_path.insert(0, cave)
                paths.append(sub_path)
    return paths


def count_path(connexions):
    paths = find_paths(['start'], connexions, [])
    paths = [path for path in paths if path[-1] == 'end']
    # for path in paths:
    #     print(path)
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
    assert(count_path(as_map(sample1))) == 36
    assert(count_path(as_map(sample2))) == 3509
    # assert(count_path(as_map(input))) == 147784


def test_as_map():
    assert(as_map('start-A\nstart-b\nA-b\nA-end')) == {
        'start': ['A', 'b'],
        'b': ['A'],
        'A': ['b', 'end']
    }
