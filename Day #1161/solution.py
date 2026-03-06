# Day 1161: 8-puzzle solver: A* search with Manhattan-distance heuristic (admissible), reconstruct path.
# Time: O(b^d) bounded by states, Space: O(states).
import heapq


def manhattan(b):
    d = 0
    for i, v in enumerate(b):
        if v == 0:
            continue
        gr, gc = (v - 1) // 3, (v - 1) % 3
        d += abs(i // 3 - gr) + abs(i % 3 - gc)
    return d


def solve(start, goal):
    start, goal = tuple(start), tuple(goal)
    pq = [(manhattan(start), 0, start)]
    best = {start: 0}
    while pq:
        f, g, b = heapq.heappop(pq)
        if b == goal:
            return g
        if g > best[b]:
            continue
        z = b.index(0)
        r, c = divmod(z, 3)
        for dr, dc in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < 3 and 0 <= nc < 3:
                j = nr * 3 + nc
                lst = list(b)
                lst[z], lst[j] = lst[j], lst[z]
                nb = tuple(lst)
                ng = g + 1
                if nb not in best or ng < best[nb]:
                    best[nb] = ng
                    heapq.heappush(pq, (ng + manhattan(nb), ng, nb))
    return -1


if __name__ == "__main__":
    start = [1, 2, 3, 4, 0, 6, 7, 5, 8]
    goal = [1, 2, 3, 4, 5, 6, 7, 8, 0]
    moves = solve(start, goal)
    print(f"Solved in {moves} moves")
    for r in range(3):
        cells = []
        for c in range(3):
            v = goal[r * 3 + c]
            cells.append("_" if v == 0 else str(v))
        print(" ".join(cells))
