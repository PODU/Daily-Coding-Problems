# Day 1586: 8-puzzle solver via A* search with Manhattan-distance heuristic (admissible => optimal).
# Time: O(b^d * log) over states explored; Space: O(states) for visited/frontier.
import heapq

GOAL = (1, 2, 3, 4, 5, 6, 7, 8, 0)


def manhattan(b):
    d = 0
    for i, v in enumerate(b):
        if v == 0:
            continue
        gi = v - 1
        d += abs(i // 3 - gi // 3) + abs(i % 3 - gi % 3)
    return d


def solve(start):
    start = tuple(start)
    g = {start: 0}
    parent = {start: start}
    moved = {}
    open_heap = [(manhattan(start), 0, start)]
    dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    found = False
    while open_heap:
        f, gc, cur = heapq.heappop(open_heap)
        if cur == GOAL:
            found = True
            break
        if gc > g[cur]:
            continue
        z = cur.index(0)
        zr, zc = divmod(z, 3)
        for dr, dc in dirs:
            nr, nc = zr + dr, zc + dc
            if not (0 <= nr < 3 and 0 <= nc < 3):
                continue
            nz = nr * 3 + nc
            lst = list(cur)
            tile = lst[nz]
            lst[z], lst[nz] = tile, 0
            nb = tuple(lst)
            ng = gc + 1
            if ng < g.get(nb, 1 << 30):
                g[nb] = ng
                parent[nb] = cur
                moved[nb] = tile
                heapq.heappush(open_heap, (ng + manhattan(nb), ng, nb))
    tiles = []
    if found:
        cur = GOAL
        while cur != start:
            tiles.append(moved[cur])
            cur = parent[cur]
        tiles.reverse()
    return tiles


def main():
    start = [[1, 2, 3], [4, 5, 6], [0, 7, 8]]
    flat = [x for row in start for x in row]
    tiles = solve(flat)
    print(f"Solved in {len(tiles)} moves")
    print("Tiles slid: " + ", ".join(str(t) for t in tiles))


if __name__ == "__main__":
    main()
