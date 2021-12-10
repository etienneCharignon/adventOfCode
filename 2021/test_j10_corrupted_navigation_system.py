from samplej10 import input
from inputj10 import rinput

SCORE = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137
}


def remove_good_pair(line):
    for pair in ['()', '{}', '[]', '<>']:
        if(pair in line):
            line = line.replace(pair, '')
            return remove_good_pair(line)
    return line


def remove_oppening(line):
    for signe in ['(', '{', '[', '<']:
        if(signe in line):
            line = line.replace(signe, '')
            return remove_oppening(line)
    return line


def score_corruption(line):
    if(line == ''):
        return 0
    return SCORE[line[0]]


def total_score(input):
    score = 0
    for line in input.split('\n'):
        cleared = remove_oppening(remove_good_pair(line))
        score += score_corruption(cleared)
    return score


def test_find_corruption():
    assert remove_good_pair('()') == ''
    assert remove_good_pair('{()}') == ''
    assert remove_good_pair('{([][])}') == ''
    assert remove_good_pair('{}') == ''
    assert remove_good_pair('{') == '{'
    assert remove_good_pair('{<>') == '{'
    assert remove_good_pair('()]') == ']'


def test_remove_oppening():
    assert remove_oppening(remove_good_pair('[{[{({}]{}}([{[{{{}}([]')) == ']}'
    assert remove_oppening(remove_good_pair('{([(<{}[<>[]}>{[]{[(<()>')) == '}>'


def test_score_corruption():
    assert score_corruption('>') == 25137
    assert score_corruption(']') == 57
    assert score_corruption('') == 0


def test_inputs():
    assert total_score(input) == 26397
    assert total_score(rinput) == 268845
