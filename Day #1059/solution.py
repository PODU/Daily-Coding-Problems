# Day 1059: Word sense disambiguation: for each ambiguous word pick the meaning whose words
# overlap most with the sentence context (other words). Tie-break -> first meaning.
# Time: O(W * M * L), Space: O(L) for the context set.
import re


def tokenize(s):
    return [t for t in re.split(r"[^a-z0-9]+", s.lower()) if t]


def disambiguate(dictionary, sentence):
    words = tokenize(sentence)
    results = {}
    for i, w in enumerate(words):
        meanings = dictionary.get(w)
        if not meanings or len(meanings) <= 1:
            continue
        context = set(words[:i] + words[i + 1:])
        best, best_score = 0, -1
        for m, meaning in enumerate(meanings):
            score = sum(1 for t in tokenize(meaning) if t in context)
            if score > best_score:
                best_score, best = score, m
        results[w] = meanings[best]
    return results


if __name__ == "__main__":
    dictionary = {
        "bank": [
            "financial institution where people deposit money",
            "land beside a river or lake",
        ]
    }
    sentence = "I went to get money from the bank"
    for word, meaning in disambiguate(dictionary, sentence).items():
        print(f"{word}: {meaning}")
