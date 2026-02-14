# Day 1066: Gale-Shapley men-proposing stable matching. Time: O(N^2), Space: O(N^2).
from collections import deque

def gale_shapley(guy_pref, gal_pref):
    men = list(guy_pref.keys())
    # gal_rank[w][m] = preference rank of man m for woman w (lower = more preferred)
    gal_rank = {w: {m: i for i, m in enumerate(gal_pref[w])} for w in gal_pref}
    next_proposal = {m: 0 for m in men}
    woman_partner = {}  # w -> m
    man_partner   = {}  # m -> w
    free_men = deque(men)
    while free_men:
        m = free_men.popleft()
        w = guy_pref[m][next_proposal[m]]
        next_proposal[m] += 1
        if w not in woman_partner:
            woman_partner[w] = m
            man_partner[m] = w
        else:
            m2 = woman_partner[w]
            if gal_rank[w][m] < gal_rank[w][m2]:
                woman_partner[w] = m
                man_partner[m] = w
                del man_partner[m2]
                free_men.append(m2)
            else:
                free_men.append(m)
    return man_partner

guy_preferences = {
    'andrew':  ['caroline', 'abigail', 'betty'],
    'bill':    ['caroline', 'betty',   'abigail'],
    'chester': ['betty',    'caroline','abigail'],
}
gal_preferences = {
    'abigail':  ['andrew', 'bill',    'chester'],
    'betty':    ['bill',   'andrew',  'chester'],
    'caroline': ['bill',   'chester', 'andrew'],
}

result = gale_shapley(guy_preferences, gal_preferences)
for man in sorted(result):
    print(f"{man} -> {result[man]}")
