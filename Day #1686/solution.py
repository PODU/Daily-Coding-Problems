# Day 1686: Selection sort using only reverse(lst,i,j). For each i, find min in [i..n-1],
# reverse [i..m] to bring it to front. Time O(n^2), Space O(1).

def reverse(lst, i, j):
    while i < j:
        lst[i], lst[j] = lst[j], lst[i]
        i += 1
        j -= 1

def sort_with_reverse(lst):
    n = len(lst)
    for i in range(n):
        m = i
        for k in range(i + 1, n):
            if lst[k] < lst[m]:
                m = k
        reverse(lst, i, m)

if __name__ == "__main__":
    data = [3, 1, 2, 5, 4]
    sort_with_reverse(data)
    print(" ".join(str(x) for x in data))
