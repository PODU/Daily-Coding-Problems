# Day 1837: Stable marriage via Gale-Shapley (men propose). Always yields a stable matching.
# Time O(N^2), Space O(N^2) for preference/ranking tables.
from collections import deque


def stable_match(guy_pref, gal_pref):
    gal_rank = {gal: {g: i for i, g in enumerate(pref)} for gal, pref in gal_pref.items()}
    nxt = {guy: 0 for guy in guy_pref}
    gal_partner = {}
    free = deque(guy_pref)

    while free:
        guy = free.popleft()
        gal = guy_pref[guy][nxt[guy]]
        nxt[guy] += 1
        cur = gal_partner.get(gal)
        if cur is None:
            gal_partner[gal] = guy
        elif gal_rank[gal][guy] < gal_rank[gal][cur]:
            gal_partner[gal] = guy
            free.append(cur)
        else:
            free.append(guy)

    return {guy: gal for gal, guy in gal_partner.items()}


def main():
    guy_preferences = {
        "andrew": ["caroline", "abigail", "betty"],
        "bill": ["caroline", "betty", "abigail"],
        "chester": ["betty", "caroline", "abigail"],
    }
    gal_preferences = {
        "abigail": ["andrew", "bill", "chester"],
        "betty": ["bill", "andrew", "chester"],
        "caroline": ["bill", "chester", "andrew"],
    }
    match = stable_match(guy_preferences, gal_preferences)
    for guy in sorted(match):
        print(f"{guy} -> {match[guy]}")


if __name__ == "__main__":
    main()
