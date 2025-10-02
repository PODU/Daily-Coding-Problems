# Day 354: Distributed crawler design (simulation): Coordinator holds a FIFO URL frontier + central visited set.
# Workers pull a URL, "download" (map lookup), record it, push unvisited links. BFS over a mock graph.
# Architecture: reach pages = BFS from seeds; visited = central set/bloom filter on coordinator;
# blacklisting = rotate IPs/user-agents, rate-limit + backoff; updates = recrawl by last-modified priority queue.
# Time O(V+E), Space O(V).
from collections import deque


def crawl(wiki, seeds):
    frontier = deque()
    visited = set()
    order = []
    for s in seeds:
        if s not in visited:
            visited.add(s)
            frontier.append(s)
    while frontier:
        url = frontier.popleft()
        order.append(url)  # "download" + record into results DB
        for link in wiki.get(url, []):
            if link not in visited:
                visited.add(link)
                frontier.append(link)
    return order


def main():
    wiki = {
        "/Main":   ["/Apple", "/Banana"],
        "/Apple":  ["/Banana", "/Fruit"],
        "/Banana": ["/Fruit"],
        "/Fruit":  ["/Main"],
    }
    order = crawl(wiki, ["/Main"])
    print("Crawled {} pages:".format(len(order)))
    for p in order:
        print(p)


if __name__ == "__main__":
    main()
