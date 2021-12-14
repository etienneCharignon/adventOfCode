from inputj14 import sample_template, sample_rules, template, rules


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


def count(polymer):
    count = {}
    for element in polymer:
        if(element not in count):
            count[element] = 0
        count[element] += 1
    return count


def run(count, template, rules):
    polymer = template
    for step in range(0, 10):
        polymer = insert(polymer, rules)
    return polymer


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


def test_run_ten_step_for_real():
    polymer = run(10, template, rules)
    counters = count(polymer)
    print(counters)
    counts = sorted(counters.values())
    assert counts[-1] - counts[0] == 2975
