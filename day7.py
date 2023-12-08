import sys
from collections import Counter
from functools import cmp_to_key

file = open("input7.txt", "r")
# file = open(sys.argv[1], "r")
lines = file.readlines()
file.close()
cards = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"]
cards2 = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J"]
cards.reverse()
cards2.reverse()
card_value = {}
plays = {}
plays_values = {(5,): 7, (4, 1): 6, (3, 2): 5, (3, 1, 1): 4, (2, 2, 1): 3, (2, 1, 1, 1): 2, (1, 1, 1, 1, 1): 1}
ordered = []

def order_play(a, b):
    cards_a, _ = a
    cards_b, _ = b
    freqs_a = Counter(list(cards_a))
    freqs_b = Counter(list(cards_b))
    # print(tuple(sorted(freqs_a.values(), reverse=True)), tuple(sorted(freqs_b.values(), reverse=True)))
    play_value_a = plays_values[tuple(sorted(freqs_a.values(), reverse=True))]
    play_value_b = plays_values[tuple(sorted(freqs_b.values(), reverse=True))]
    if play_value_a == play_value_b:
        for a, b in zip(cards_a, cards_b):
            if a == b:
                continue
            else:
                # return cards.index(b) - cards.index(a)
                return cards.index(a) - cards.index(b)
        assert False
    else:
        return (play_value_a - play_value_b)

def convert_j(freqs):
    if 'J' in freqs:
        freq_j = freqs['J']
        del freqs['J']
        mc = freqs.most_common()
        if len(mc) == 0:
            freqs['J'] = freq_j
            return freqs
        if len(mc) == 1:
            freqs[mc[0][0]] += freq_j
            return freqs
        for i in range(len(mc)-1):
            c1, f1 = mc[i]
            c2, f2 = mc[i + 1]
            if c1 != 'J':
                if f1 != f2:
                    m = c1
                else:
                    if cards.index(c1) > cards.index(c2):
                        m = c1
                    else:
                        m = c2
                break
        assert m is not None
        freqs[m] += freq_j
    return freqs


def order_play2(a, b):
    cards_a, _ = a
    cards_b, _ = b
    freqs_a = Counter(list(cards_a))
    freqs_b = Counter(list(cards_b))
    # print(tuple(sorted(freqs_a.values(), reverse=True)), tuple(sorted(freqs_b.values(), reverse=True)))
    freqs_a = convert_j(freqs_a)
    freqs_b = convert_j(freqs_b)
    play_value_a = plays_values[tuple(sorted(freqs_a.values(), reverse=True))]
    play_value_b = plays_values[tuple(sorted(freqs_b.values(), reverse=True))]
    if play_value_a == play_value_b:
        for a, b in zip(cards_a, cards_b):
            if a == b:
                continue
            else:
                return cards2.index(a) - cards2.index(b)
        assert False
    else:
        return (play_value_a - play_value_b)

plays = [(line.strip().split(" ")[0], line.strip().split(" ")[1]) for line in lines]
plays.sort(key=cmp_to_key(order_play))
acc = 0
for i, play in enumerate(plays):
    acc += int(play[1]) * (i + 1)
# print(plays)
print(acc)

acc = 0
plays.sort(key=cmp_to_key(order_play2))
for i, play in enumerate(plays):
    acc += int(play[1]) * (i + 1)
# print(plays)
print(acc)
# for line in lines:
#     cards, bid = line.strip().split(" ")
#     freqs = Counter(list(cards))
#     play_value = plays_values[tuple(sorted(freqs.values(), reverse=True))]
#     print(freqs, play_value)
#     # freqs = sorted(freqs.items(), key=lambda x: x[1], reverse=True)
#     plays[tuple(freqs)] = bid
