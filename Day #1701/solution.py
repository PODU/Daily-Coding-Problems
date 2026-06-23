# Day 1701: Stream voting: dict vote counts + set of seen voters; duplicate voter = fraud.
# Top 3 by count (ties by candidate id). Time O(n + k log k), Space O(k+v).
def main():
    records = [(1,'A'),(2,'B'),(3,'A'),(4,'C'),(2,'A'),(5,'B'),(6,'A')]
    counts = {}
    seen = set()
    for voter, cand in records:
        if voter in seen:
            print(f"Fraud detected: voter {voter} voted more than once")
            continue
        seen.add(voter)
        counts[cand] = counts.get(cand, 0) + 1
    top = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))[:3]
    print("Top 3 candidates: " + ", ".join(f"{c}({n})" for c, n in top))

if __name__ == "__main__":
    main()
