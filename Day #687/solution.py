# Day 687: 8-puzzle solver via A* with Manhattan-distance heuristic (admissible -> optimal solution).
# State = 9-tuple (blank=0); explore by sliding a tile into the blank.
import heapq

GOAL = (1, 2, 3, 4, 5, 6, 7, 8, 0)


def manhattan(b):
    d = 0
    for i, v in enumerate(b):
        if v == 0:
            continue
        gr, gc = (v - 1) // 3, (v - 1) % 3
        r, c = i // 3, i % 3
        d += abs(gr - r) + abs(gc - c)
    return d


def neighbors(b):
    z = b.index(0)
    zr, zc = z // 3, z % 3
    for dr, dc in ((-1, 0), (1, 0), (0, -1), (0, 1)):
        nr, nc = zr + dr, zc + dc
        if 0 <= nr < 3 and 0 <= nc < 3:
            nz = nr * 3 + nc
            lst = list(b)
            lst[z], lst[nz] = lst[nz], lst[z]
            yield tuple(lst)


def solve(start):
    g = {start: 0}
    parent = {start: None}
    pq = [(manhattan(start), start)]
    while pq:
        f, cur = heapq.heappop(pq)
        if cur == GOAL:
            break
        if f > g[cur] + manhattan(cur):  # stale entry
            continue
        for nb in neighbors(cur):
            ng = g[cur] + 1
            if nb not in g or ng < g[nb]:
                g[nb] = ng
                parent[nb] = cur
                heapq.heappush(pq, (ng + manhattan(nb), nb))

    path = []
    cur = GOAL
    while cur is not None:
        path.append(cur)
        cur = parent[cur]
    path.reverse()
    return path


if __name__ == '__main__':
    start = (1, 2, 3, 4, 0, 6, 7, 5, 8)
    path = solve(start)
    moves = len(path) - 1
    print(f"Solved in {moves} moves")
    print("Goal board:")
    gb = path[-1]
    for i in range(0, 9, 3):
        print(' '.join(str(x) for x in gb[i:i + 3]))
