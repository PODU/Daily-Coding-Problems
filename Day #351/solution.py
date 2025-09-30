# Day 351: Simplified Lesk: score each gloss by word overlap with sentence context; pick max (ties->first).
# Time O(words * meanings * glossLen), Space O(vocab).

def disambiguate(meanings, sentence):
    tokens = sentence.lower().split()
    results = []
    for i, w in enumerate(tokens):
        senses = meanings.get(w)
        if not senses or len(senses) < 2:
            continue  # not ambiguous
        context = set(tokens[:i] + tokens[i+1:])
        best_idx, best_score = 0, -1
        for idx, gloss in enumerate(senses):
            score = len(set(gloss.lower().split()) & context)
            if score > best_score:
                best_score, best_idx = score, idx
        results.append((w, senses[best_idx]))
    return results


def main():
    meanings = {
        "bank": ["place where people deposit and withdraw money",
                 "sloping land beside a river or lake of water"],
        "money": ["currency cash that people deposit"],
        "river": ["large natural stream of water"],
    }
    sentence = "I went to get money from the bank"
    for word, sense in disambiguate(meanings, sentence):
        print(f"{word}: {sense}")


if __name__ == "__main__":
    main()
