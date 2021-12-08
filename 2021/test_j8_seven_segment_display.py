from samplej8 import input
from inputj8 import rinput


def counting_easy_digits(display):
    digits = display.split()
    return len([digit for digit in digits if len(digit) in [2, 4, 3, 7]])


def counting_all_easy_digits(input):
    def extract_display(line):
        return line.split(' | ')[1]

    return sum(map(counting_easy_digits, map(extract_display, input.split('\n'))))


def test_counting_easy_digits():
    assert counting_easy_digits('fdgacbe') == 1
    assert counting_easy_digits('fdgacbe cefdb cefbgd gcbe') == 2
    assert counting_easy_digits('fcgedb cgb dgebacf gc') == 3


def test_counting_all_easy_digits():
    assert counting_all_easy_digits(input) == 26
    assert counting_all_easy_digits(rinput) == 247
