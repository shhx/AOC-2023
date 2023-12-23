from collections import defaultdict, deque
from sys import argv

with open(argv[1]) as f:
    lines = f.readlines()

m = [list(l.strip()) for l in lines]

to_visit = deque()
to_visit.append((0, 1, set()))
total_visited = set()

dirs = {"<": (0, -1), ">": (0, 1), "^": (-1, 0), "v": (1, 0)}
finish = (len(m) - 1, len(m[0]) - 2)
print(finish, m[finish[0]][finish[1]])


edges = defaultdict(set)
for x in range(len(m)):
    for y in range(len(m[0])):
        if m[x][y] == ".":
            for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                xn, yn = x + dx, y + dy
                if xn < 0 or yn < 0 or xn >= len(m) or yn >= len(m[0]):
                    continue
                if m[xn][yn] in [".", "<", ">", "^", "v"]:
                    edges[(x, y)].add((xn, yn, 1))
                    edges[(xn, yn)].add((x, y, 1))

# print(edges)
while True:
    for n, neighbors in edges.items():
        if len(neighbors) == 2:
            (x1, y1, d1), (x2, y2, d2) = neighbors
            edges[(x1, y1)].remove((n[0], n[1], d1))
            edges[(x2, y2)].remove((n[0], n[1], d2))
            edges[(x1, y1)].add((x2, y2, d1 + d2))
            edges[(x2, y2)].add((x1, y1, d1 + d2))
            del edges[n]
            break
    else:
        break

print("Edges", len(edges))
to_visit = deque()
to_visit.append((0, 1, 0, set()))

length = 0
longest = None
while to_visit:
    n = to_visit.pop()
    x, y, l, v = n
    if (x, y) in v:
        continue
    if (x, y) not in edges:
        continue
    v.add((x, y))
    if (x, y) == finish:
        length = max(length, l)
        continue

    to_visit.extend((xn, yn, l + ln, v.copy()) for xn, yn, ln in edges[(x, y)])

print(length)
