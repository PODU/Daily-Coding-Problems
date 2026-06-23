# Day 1702: Largest rectangle of 1s: per-row histogram + largest-rectangle-in-histogram via stack.
# Time O(N*M), Space O(M).
def largest_in_hist(h):
    best = 0
    st = []
    for i in range(len(h) + 1):
        cur = 0 if i == len(h) else h[i]
        while st and h[st[-1]] >= cur:
            height = h[st.pop()]
            width = i if not st else i - st[-1] - 1
            best = max(best, height * width)
        st.append(i)
    return best

def main():
    mat = [[1,0,0,0],[1,0,1,1],[1,0,1,1],[0,1,0,0]]
    m = len(mat[0])
    h = [0] * m
    best = 0
    for row in mat:
        for j in range(m):
            h[j] = h[j] + 1 if row[j] else 0
        best = max(best, largest_in_hist(h))
    print(best)

if __name__ == "__main__":
    main()
