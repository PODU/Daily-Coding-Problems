# Day 1114: Day 1114 - Voting stream: top 3 candidates + fraud detection
# Approach: stream records, hash-map vote counts, set of seen voters for O(1)
# duplicate detection; top-3 via partial selection. Time: O(R + Q*M), Space: O(V+M).

def process_stream(stream):
    counts = {}
    seen = set()
    fraud = []
    for voter, cand in stream:
        if voter in seen:
            fraud.append(voter)
            print(f"Fraud detected: voter {voter} voted more than once")
            continue
        seen.add(voter)
        counts[cand] = counts.get(cand, 0) + 1
    return counts, fraud


def top3(counts):
    items = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))
    return items[:3]


if __name__ == "__main__":
    stream = [
        (1, 'A'), (2, 'B'), (3, 'A'), (4, 'C'), (5, 'B'),
        (6, 'A'), (2, 'C'), (7, 'D'), (8, 'A'),
    ]
    counts, fraud = process_stream(stream)
    print("Top 3 candidates:", top3(counts))
    # Fraud detected: voter 2 voted more than once
    # Top 3 candidates: [('A', 4), ('B', 2), ('C', 1)]
