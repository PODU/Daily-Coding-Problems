# Day 329: Gale-Shapley stable marriage, men propose; free man proposes down his list, women keep better partner.
# Time O(n^2), Space O(n^2).
from collections import deque


def main():
    men = ["andrew", "bill", "chester"]
    women = ["abigail", "betty", "caroline"]
    guy_pref = {
        "andrew": ["caroline", "abigail", "betty"],
        "bill": ["caroline", "betty", "abigail"],
        "chester": ["betty", "caroline", "abigail"],
    }
    gal_pref = {
        "abigail": ["andrew", "bill", "chester"],
        "betty": ["bill", "andrew", "chester"],
        "caroline": ["bill", "chester", "andrew"],
    }

    wrank = {w: {m: i for i, m in enumerate(gal_pref[w])} for w in women}
    nxt = {m: 0 for m in men}
    partner_of = {}  # woman -> man
    free_men = deque(men)

    while free_men:
        m = free_men.popleft()
        w = guy_pref[m][nxt[m]]
        nxt[m] += 1
        if w not in partner_of:
            partner_of[w] = m
        else:
            cur = partner_of[w]
            if wrank[w][m] < wrank[w][cur]:
                partner_of[w] = m
                free_men.append(cur)
            else:
                free_men.append(m)

    man_to_woman = {man: woman for woman, man in partner_of.items()}
    for m in men:  # already sorted
        print("{} - {}".format(m, man_to_woman[m]))


if __name__ == "__main__":
    main()
