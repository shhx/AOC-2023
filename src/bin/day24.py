import math
from collections import deque
from sys import argv

from numpy import array, cross

with open(argv[1]) as f:
    lines = f.readlines()

MIN = 200000000000000
MAX = 400000000000000
# MIN = 7
# MAX = 27


def instersect_xy(a, b):
    p, r = a
    q, s = b
    if cross(r, s) == 0:
        return False
    t = cross(q - p, s) / cross(r, s)
    u = cross(q - p, r) / cross(r, s)
    if t < 0 or u < 0:  # past
        return False
    inter_x = p[0] + t * r[0]
    inter_y = p[1] + t * r[1]
    if MIN <= inter_x <= MAX and MIN <= inter_y <= MAX:
        return True

acc = 0
for i, a in enumerate(lines):
    for b in lines[i+1:]:
        pos_a, vel_a = a.split('@')
        pos_a, vel_a = pos_a.strip().split(','), vel_a.strip().split(',')
        pos_a, vel_a = tuple(map(int, pos_a)), tuple(map(int, vel_a))
        pos_b, vel_b = b.split('@')
        pos_b, vel_b = pos_b.strip().split(','), vel_b.strip().split(',')
        pos_b, vel_b = tuple(map(float, pos_b)), tuple(map(float, vel_b))
        pos_a = array(pos_a[0:2])
        pos_b = array(pos_b[0:2])
        va = array(vel_a[0:2])
        vb = array(vel_b[0:2])
        # print("-------", pos_a, pos_b)
        if instersect_xy((pos_a, va), (pos_b, vb)):
            acc += 1
            # print("Inter", pos_a, pos_b, vel_a, vel_b)

print(acc)

import numpy as np

rocks = []
for l in lines[0:3]:
    p, v = l.split('@')
    p, v = p.strip().split(','), v.strip().split(',')
    p, v = tuple(map(int, p)), tuple(map(int, v))
    rocks.append((p, v))

(p1, v1), (p2, v2), (p3, v3) = rocks

A = np.array([
    [-(v1[1] - v2[1]), v1[0] - v2[0], 0, p1[1] - p2[1], -(p1[0] - p2[0]), 0],
    [-(v1[1] - v3[1]), v1[0] - v3[0], 0, p1[1] - p3[1], -(p1[0] - p3[0]), 0],

    [0, -(v1[2] - v2[2]), v1[1] - v2[1],  0, p1[2] - p2[2], -(p1[1] - p2[1])],
    [0, -(v1[2] - v3[2]), v1[1] - v3[1],  0, p1[2] - p3[2], -(p1[1] - p3[1])],

    [-(v1[2] - v2[2]), 0, v1[0] - v2[0],  p1[2] - p2[2], 0, -(p1[0] - p2[0])],
    [-(v1[2] - v3[2]), 0, v1[0] - v3[0],  p1[2] - p3[2], 0, -(p1[0] - p3[0])]
    ])

b = [
        (p1[1] * v1[0] - p2[1] * v2[0]) - (p1[0] * v1[1] - p2[0] * v2[1]),
        (p1[1] * v1[0] - p3[1] * v3[0]) - (p1[0] * v1[1] - p3[0] * v3[1]),

        (p1[2] * v1[1] - p2[2] * v2[1]) - (p1[1] * v1[2] - p2[1] * v2[2]),
        (p1[2] * v1[1] - p3[2] * v3[1]) - (p1[1] * v1[2] - p3[1] * v3[2]),

        (p1[2] * v1[0] - p2[2] * v2[0]) - (p1[0] * v1[2] - p2[0] * v2[2]),
        (p1[2] * v1[0] - p3[2] * v3[0]) - (p1[0] * v1[2] - p3[0] * v3[2])
     ]

x = np.linalg.solve(A, b)
print(round(sum(x[:3])))
