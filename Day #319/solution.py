# Day 319: 8-puzzle solver: A* search with Manhattan-distance heuristic; blank=0.
# Time ~O(b^d) bounded by states explored; Space O(states stored).
import heapq

def manhattan(b):
    d = 0
    for i, v in enumerate(b):
        if v == 0:
            continue
        gr, gc = (v - 1) // 3, (v - 1) % 3
        r, c = i // 3, i % 3
        d += abs(gr - r) + abs(gc - c)
    return d

def solve(start, goal):
    start, goal = tuple(start), tuple(goal)
    moves = [(-3, "Up"), (3, "Down"), (-1, "Left"), (1, "Right")]
    g = {start: 0}
    parent = {}
    pq = [(manhattan(start), 0, start)]
    while pq:
        f, gc, cur = heapq.heappop(pq)
        if gc > g[cur]:
            continue
        if cur == goal:
            break
        blank = cur.index(0)
        r, c = blank // 3, blank % 3
        for delta, name in moves:
            if name == "Up" and r == 0: continue
            if name == "Down" and r == 2: continue
            if name == "Left" and c == 0: continue
            if name == "Right" and c == 2: continue
            nb = blank + delta
            lst = list(cur)
            lst[blank], lst[nb] = lst[nb], lst[blank]
            nx = tuple(lst)
            ng = gc + 1
            if nx not in g or ng < g[nx]:
                g[nx] = ng
                parent[nx] = (cur, name)
                heapq.heappush(pq, (ng + manhattan(nx), ng, nx))
    path = []
    cur = goal
    while cur != start:
        prev, name = parent[cur]
        path.append(name)
        cur = prev
    path.reverse()
    return path

if __name__ == "__main__":
    start = [[1, 2, 3], [4, 5, 6], [7, 0, 8]]
    goal = [[1, 2, 3], [4, 5, 6], [7, 8, 0]]
    flat_start = [x for row in start for x in row]
    flat_goal = [x for row in goal for x in row]
    path = solve(flat_start, flat_goal)
    print("Solved in {} move(s): {}".format(len(path), ", ".join(path)))
