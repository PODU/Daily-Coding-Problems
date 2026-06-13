# Day 1654: Simplified Lesk WSD: score each candidate meaning of an ambiguous word by overlap
# with the union of other in-dict context words and their meaning texts. O(W*M*L) time, O(V) space.
meanings = {
    "bank": ["place where people deposit and withdraw money", "land beside a river or lake"],
    "money": ["currency coins and cash used to buy things"],
    "river": ["a large natural stream of flowing water"],
}
sentence = "I went to get money from the bank"


def words(s):
    return s.lower().split()


toks = words(sentence)
for w in toks:
    if w in meanings and len(meanings[w]) > 1:
        ctx = set()
        for o in toks:
            if o != w and o in meanings:
                ctx.add(o)
                for m in meanings[o]:
                    ctx.update(words(m))
        best, best_score = meanings[w][0], -1
        for cand in meanings[w]:
            score = sum(1 for t in words(cand) if t in ctx)
            if score > best_score:
                best_score, best = score, cand
        print(f"{w}: {best}")
