# Day 1213: Stable marriage via Gale-Shapley (men propose).
# Each free man proposes down his list; women keep their best suitor. Time O(N^2), Space O(N^2).
from collections import deque


def stable_match(guys, gals):
    rank = {w: {m: i for i, m in enumerate(prefs)} for w, prefs in gals.items()}
    nxt = {m: 0 for m in guys}
    engaged = {}  # woman -> man
    free = deque(guys)
    while free:
        m = free.popleft()
        w = guys[m][nxt[m]]
        nxt[m] += 1
        if w not in engaged:
            engaged[w] = m
        else:
            cur = engaged[w]
            if rank[w][m] < rank[w][cur]:
                engaged[w] = m
                free.append(cur)
            else:
                free.append(m)
    return engaged


if __name__ == "__main__":
    guys = {
        "andrew": ["caroline", "abigail", "betty"],
        "bill": ["caroline", "betty", "abigail"],
        "chester": ["betty", "caroline", "abigail"],
    }
    gals = {
        "abigail": ["andrew", "bill", "chester"],
        "betty": ["bill", "andrew", "chester"],
        "caroline": ["bill", "chester", "andrew"],
    }
    match = stable_match(guys, gals)
    for man in sorted(guys):
        w = next(w for w, mm in match.items() if mm == man)
        print(f"{man} - {w}")
    # andrew - abigail / bill - caroline / chester - betty
