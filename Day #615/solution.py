# Day 615: Stable Marriage (Gale-Shapley, men proposing). Each free man proposes down his list.
# Time: O(N^2), Space: O(N^2) for preference rank tables.
from collections import deque


def gale_shapley(guy_preferences, gal_preferences):
    men = list(guy_preferences.keys())
    rank = {w: {m: i for i, m in enumerate(prefs)}
            for w, prefs in gal_preferences.items()}
    nxt = {m: 0 for m in men}
    husband = {}
    free = deque(men)

    while free:
        m = free.popleft()
        w = guy_preferences[m][nxt[m]]
        nxt[m] += 1
        if w not in husband:
            husband[w] = m
        else:
            cur = husband[w]
            if rank[w][m] < rank[w][cur]:
                husband[w] = m
                free.append(cur)
            else:
                free.append(m)

    wife = {m: w for w, m in husband.items()}
    return {m: wife[m] for m in men}


if __name__ == "__main__":
    guy_preferences = {
        'andrew':  ['caroline', 'abigail', 'betty'],
        'bill':    ['caroline', 'betty', 'abigail'],
        'chester': ['betty', 'caroline', 'abigail'],
    }
    gal_preferences = {
        'abigail':  ['andrew', 'bill', 'chester'],
        'betty':    ['bill', 'andrew', 'chester'],
        'caroline': ['bill', 'chester', 'andrew'],
    }
    print(gale_shapley(guy_preferences, gal_preferences))
