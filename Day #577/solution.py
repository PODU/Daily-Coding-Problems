# Day 577: Word circle: model words as directed edges first->last char; Eulerian circuit via Hierholzer. Time O(V+E), Space O(V+E).
def word_circle(words):
    adj = {}  # char -> list of (nextChar, word) in input order
    for w in words:
        adj.setdefault(w[0], []).append((w[-1], w))
    ptr = {c: 0 for c in adj}

    start = words[0][0]
    path = []
    stack = [(start, None)]  # (char, word used to enter)
    while stack:
        v = stack[-1][0]
        lst = adj.get(v, [])
        p = ptr.get(v, 0)
        if p < len(lst):
            ptr[v] = p + 1
            nxt, w = lst[p]
            stack.append((nxt, w))
        else:
            _, w = stack.pop()
            if w is not None:
                path.append(w)
    path.reverse()
    return path


if __name__ == "__main__":
    words = ['chair', 'height', 'racket', 'touch', 'tunic']
    path = word_circle(words)
    print(" --> ".join(path) + " --> " + path[0])
