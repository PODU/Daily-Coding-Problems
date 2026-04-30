# Day 1443: Word sense disambiguation (simplified Lesk algorithm).
# For each ambiguous word pick the meaning whose words overlap most with the
# rest of the sentence's words. Time O(W * M * L), Space O(vocab).
import re
from typing import Dict, List


def tokenize(s: str) -> List[str]:
    return [t for t in re.split(r"[^a-z0-9]+", s.lower()) if t]


def disambiguate(meanings: Dict[str, List[str]], sentence: str) -> Dict[str, str]:
    words = tokenize(sentence)
    context = set(words)
    result = {}
    for w in words:
        senses = meanings.get(w)
        if not senses or len(senses) <= 1:
            continue
        best_score, best_meaning = -1, None
        for m in senses:
            mt = set(tokenize(m))
            score = sum(1 for t in mt if t != w and t in context)
            if score > best_score:
                best_score, best_meaning = score, m
        result[w] = best_meaning
    return result


if __name__ == "__main__":
    meanings = {
        "bank": [
            "financial institution where people deposit money",
            "sloping land beside a river or lake",
        ]
    }
    sentence = "I went to the bank to deposit money"
    res = disambiguate(meanings, sentence)
    print("bank:", res["bank"])
