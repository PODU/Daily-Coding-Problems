# Day 882: 8-puzzle solver via A* with Manhattan-distance heuristic (optimal moves).
# State = 9-char string, 0 = blank. Time/space depend on solution depth.
import heapq

GOAL = "123456780"


def manhattan(s):
    d = 0
    for i, ch in enumerate(s):
        if ch == "0":
            continue
        v = int(ch) - 1
        d += abs(i // 3 - v // 3) + abs(i % 3 - v % 3)
    return d


def solve(start):
    pq = [(manhattan(start), 0, start)]
    g = {start: 0}
    parent = {}
    INF = float("inf")
    while pq:
        f, gc, cur = heapq.heappop(pq)
        if cur == GOAL:
            break
        if gc > g.get(cur, INF):
            continue
        z = cur.index("0")
        r, c = divmod(z, 3)
        for dr, dc in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < 3 and 0 <= nc < 3:
                nz = nr * 3 + nc
                lst = list(cur)
                lst[z], lst[nz] = lst[nz], lst[z]
                nxt = "".join(lst)
                ng = gc + 1
                if ng < g.get(nxt, INF):
                    g[nxt] = ng
                    parent[nxt] = (cur, int(cur[nz]))
                    heapq.heappush(pq, (ng + manhattan(nxt), ng, nxt))
    moves = []
    cur = GOAL
    while cur != start:
        prev, tile = parent[cur]
        moves.append(tile)
        cur = prev
    return moves[::-1]


if __name__ == "__main__":
    start = "123406758"  # [[1,2,3],[4,_,6],[7,5,8]]
    moves = solve(start)
    print(f"Solved in {len(moves)} moves: {moves}")
