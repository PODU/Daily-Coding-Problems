# Day 1558: Count connected components (friend groups) in an undirected graph via DFS.
# Time O(V+E), Space O(V).


def main():
    adj = {0: [1, 2], 1: [0, 5], 2: [0], 3: [6], 4: [], 5: [1], 6: [3]}

    visited = set()
    groups = 0
    for start in adj:
        if start in visited:
            continue
        groups += 1
        stack = [start]
        visited.add(start)
        while stack:
            u = stack.pop()
            for v in adj[u]:
                if v not in visited:
                    visited.add(v)
                    stack.append(v)
    print(groups)


if __name__ == "__main__":
    main()
