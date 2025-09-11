# Day 251: External/bucket sort demo for sorting ~1M ints in [0,1e9] with bounded memory.
# Real answer for data exceeding RAM: external merge sort (split into chunks ->
# sort each chunk on disk -> k-way merge). Here we bucket by range, sort each
# bucket, and concatenate. Time: O(n log(n/k)); Space: O(n) bounded per bucket.

def bucket_sort(data, max_val, num_buckets):
    width = max_val // num_buckets + 1
    buckets = [[] for _ in range(num_buckets)]
    for x in data:
        b = min(x // width, num_buckets - 1)
        buckets[b].append(x)
    out = []
    for bk in buckets:
        bk.sort()  # each bucket fits in memory
        out.extend(bk)
    return out


if __name__ == "__main__":
    input_data = [5, 1, 4, 2, 8, 1000000000, 0]
    result = bucket_sort(input_data, 1000000000, 16)
    print(" ".join(str(x) for x in result))
