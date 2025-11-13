# Day 591: Word sense disambiguation via simplified Lesk.
# Score each candidate gloss by content-word overlap with the sentence
# context (other words + their glosses); pick the highest.
# Time O(words * meanings * glossLen). Space O(vocab).

STOP = {
    "i", "to", "the", "a", "an", "of", "by", "and", "or", "where", "people",
    "that", "is", "are", "in", "on", "at", "with", "went", "sat", "this"
}


def tokens(text):
    out = set()
    for w in text.lower().split():
        lw = "".join(c for c in w if c.isalpha())
        if lw and lw not in STOP:
            out.add(lw)
    return out


def main():
    meanings = {
        "bank": ["a financial institution where people deposit and withdraw money",
                 "the land alongside a river or lake"],
        "money": ["currency that people deposit and withdraw"],
        "river": ["a large natural stream of water"],
    }

    sentences = [
        "I went to the bank to deposit money",
        "I sat by the bank of the river",
    ]

    for sentence in sentences:
        words = sentence.split()
        for w in words:
            lw = w.lower()
            cands = meanings.get(lw)
            if not cands or len(cands) <= 1:
                continue

            context = set()
            for other in words:
                ol = other.lower()
                if ol == lw:
                    continue
                context |= tokens(other)
                for g in meanings.get(ol, []):
                    context |= tokens(g)

            best = -1
            best_gloss = ""
            for gloss in cands:
                overlap = len(tokens(gloss) & context)
                if overlap > best:
                    best = overlap
                    best_gloss = gloss
            print(f"{lw}: {best_gloss}")


if __name__ == "__main__":
    main()
