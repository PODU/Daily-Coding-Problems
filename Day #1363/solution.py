# Day 1363: Stream voting: dict candidate->count, set of voters to detect fraud; top-3 via sort.
# Time: O(records) processing + O(C log C) reporting. Space: O(C + V).

def main():
    stream = [
        ("v1", "A"), ("v2", "B"), ("v3", "A"), ("v4", "C"),
        ("v5", "B"), ("v6", "A"), ("v7", "C"), ("v1", "B"),
    ]

    counts = {}
    seen = set()

    for voter, cand in stream:
        if voter in seen:
            print(f"Fraud detected: voter {voter}")
            continue
        seen.add(voter)
        counts[cand] = counts.get(cand, 0) + 1

    ordered = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))
    top3 = ", ".join(c for c, _ in ordered[:3])
    print(f"Top 3 candidates: {top3}")


if __name__ == "__main__":
    main()
