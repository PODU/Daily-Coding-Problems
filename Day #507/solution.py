# Day 507: Streaming vote tally: set of seen voters (duplicate -> fraud, vote dropped),
# dict candidate->count, top 3 computed on demand. Time O(n + k log k), Space O(n+k).


def main():
    stream = [
        ("v1", "A"), ("v2", "A"), ("v3", "B"), ("v4", "C"),
        ("v5", "B"), ("v6", "B"), ("v7", "C"), ("v2", "D"),
    ]

    seen = set()
    tally = {}

    for voter, cand in stream:
        if voter in seen:
            print(f"Fraud detected: voter {voter} voted more than once")
            continue  # do not count fraudulent vote
        seen.add(voter)
        tally[cand] = tally.get(cand, 0) + 1

    # sort by votes desc, then candidate id ascending
    ordered = sorted(tally.items(), key=lambda kv: (-kv[1], kv[0]))
    top = ordered[:3]
    print("Top 3: " + ", ".join(f"{c}({n})" for c, n in top))


if __name__ == "__main__":
    main()
