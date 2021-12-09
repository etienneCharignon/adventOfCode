# from inputj8 import input
from samplej8 import input
from j8_seven_segment_display import filter_one_signal, extract_signals

display = {
    'a': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    'b': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    'c': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    'd': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    'e': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    'f': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    'g': ['a', 'b', 'c', 'd', 'e', 'f', 'g']
}


def filter_signals(signals):
    def filter_display(signal):
        filter_one_signal(signal, display)

    map(filter_display, signals.split())


map(filter_signals, map(extract_signals, input.split('\n')))
print(display)
