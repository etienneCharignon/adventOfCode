from inputj14 import sample_template, sample_rules
from inputj14 import rules, template


def insert(template, rules):
    polymer = ''
    previous = ''
    for element in template:
        if(previous == ''):
            previous = element
            continue
        pair = previous + element
        if(pair in rules):
            polymer += previous + rules[pair]
        else:
            polymer += previous
        previous = element
    polymer += previous
    return polymer


def add_count(element, count, number):
    if(element not in count):
        count[element] = 0
    count[element] += number


def count(polymer):
    count = {}
    for element in polymer:
        add_count(element, count, 1)
    return count


def run(iterations, template, rules):
    polymer = template
    for step in range(0, iterations):
        print(step)
        print(count(polymer))
        polymer = insert(polymer, rules)
    return polymer


def add_pair(pairs, pair, number):
    if(pair in pairs):
        pairs[pair] += number
    else:
        pairs[pair] = number


def init(template):
    previous = ''
    pairs = {}
    for element in template:
        if(previous == ''):
            previous = element
            continue
        add_pair(pairs, previous + element, 1)
        previous = element
    return pairs


def step(nb, pairs, rules, counters):
    if(nb == 0):
        return counters, pairs
    new_pairs = {}
    for pair in pairs.keys():
        a, b = pair
        if(pair in rules):
            add_count(rules[pair], counters, pairs[pair])
            add_pair(new_pairs, a + rules[pair], pairs[pair])
            add_pair(new_pairs, rules[pair] + b, pairs[pair])
        else:
            add_pair(new_pairs, pair, pairs[pair])
    print(new_pairs)
    return step(nb - 1, new_pairs, rules, counters)


def test_one_insertion():
    assert insert('NNCB', {'NN': 'B'}) == 'NBNCB'
    assert insert(sample_template, sample_rules) == 'NCNBCHB'
    assert insert('NCNBCHB', sample_rules) == 'NBCCNBBBCBHCB'


def test_count_elements():
    assert count('NNCB') == {'N': 2, 'C': 1, 'B': 1}


def test_run_ten_step():
    polymer = run(10, sample_template, sample_rules)
    counters = count(polymer)
    print(counters)
    counts = sorted(counters.values())
    assert counts[-1] - counts[0] == 1588


def test_init():
    assert init('NNCB') == {'NN': 1, 'NC': 1, 'CB': 1}
    assert init('FNFPPNKPPHSOKFFHOFOC')['PP'] == 2


def new_count(pairs):
    count = {}
    for pair in pairs:
        a, b = pair
        if(a not in count):
            count[a] = 0
        count[a] += pairs[pair]
        if(b not in count):
            count[b] = 0
        count[b] += pairs[pair]
    return count


def test_step():
    assert step(1, init(sample_template), sample_rules, {})[1] == {'NC': 1, 'CN': 1, 'NB': 1, 'BC': 1, 'CH': 1, 'HB': 1}
    assert step(2, init(sample_template), sample_rules, {})[1] == {'NB': 2,
                                                                   'BC': 2,
                                                                   'CC': 1,
                                                                   'CN': 1,
                                                                   'BB': 2,
                                                                   'CB': 2,
                                                                   'BH': 1,
                                                                   'HC': 1}
    print('and with 10')
    print(count(template))
    counters = count(template)
    counters, pairs = step(40, init(template), rules, counters)
    print(counters)
    counts = sorted(counters.values())
    assert counts[-1] - counts[0] == 3015383850689


# test_run_ten_step()
