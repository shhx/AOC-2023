from collections import deque
from sys import argv

with open(argv[1]) as f:
    lines = f.readlines()

m = [list(l.strip()) for l in lines]
print(m)

to_visit = deque()
to_visit.append((0, 1, set()))
total_visited = set()

dirs = {'<': (0, -1), '>': (0, 1), '^': (-1, 0), 'v': (1, 0)}
finish = (len(m)-1, len(m[0])-2)
print(finish, m[finish[0]][finish[1]])
length = 0
longest = None
while to_visit:
    n = to_visit.popleft()
    x, y, v = n
    # print(x, y, len(v))
    if (x, y) in v:
        continue
    if x < 0 or y < 0 or x >= len(m) or y >= len(m[0]):
        continue
    if m[x][y] == '#':
        continue
    # total_visited.add((x, y))
    v.add((x, y))
    if (x, y) == finish:
        print('found-----------')
        # length = max(length, len(v))
        if len(v) > length:
            length = len(v)
            longest = v
        continue

    # if m[x][y] in dirs:
    #     dx, dy = dirs[m[x][y]]
    #     to_visit.append((x+dx, y+dy, v.copy()))
    #     continue

    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
        to_visit.append((x+dx, y+dy, v.copy()))

# for i in range(len(m)):
#     for j in range(len(m[0])):
#         if (i, j) not in longest:
#             print(m[i][j], end='')
#         else:
#             print('O', end='')
#     print()
print(length-1)
