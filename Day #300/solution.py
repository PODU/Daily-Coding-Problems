# Day 300: Stream voting: track seen voters + candidate counts; report top-3 (count desc, id asc) or FRAUD on repeat.
# Time: O(n * c log c) over stream, Space: O(voters + candidates).
def process_stream(stream):
    seen = set()
    counts = {}
    for voter, cand in stream:
        if voter in seen:
            print(f"Fraud: voter {voter} voted more than once")
            continue
        seen.add(voter)
        counts[cand] = counts.get(cand, 0) + 1
        ranked = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))
        top3 = [cid for cid, _ in ranked[:3]]
        print(f"Top 3: {top3}")


if __name__ == "__main__":
    stream = [(1, 'A'), (2, 'B'), (3, 'A'), (4, 'C'), (5, 'B'), (1, 'A'), (6, 'A')]
    process_stream(stream)
