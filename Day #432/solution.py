# Day 432: Distributed Wikipedia crawler — concrete single-process simulation.
# Design: a central coordinator holds a URL frontier (BFS queue) + visited set for dedup.
# Worker machines pull URLs, "fetch" pages from an in-memory graph, parse links, store page
# content keyed by a content hash + last-crawled timestamp, and push new URLs back to the
# frontier. Politeness delay per domain + worker blacklisting/rotation prevent overload
# (a worker that hits its rate limit is rotated out). Incremental re-crawl compares the
# stored content hash to detect updates. O(V+E) over the page graph.

from collections import deque


def fnv1a(s):
    h = 2166136261
    for b in s.encode('utf-8'):
        h ^= b
        h = (h * 16777619) & 0xFFFFFFFF
    return h


class Coordinator:
    def __init__(self, wiki):
        self.wiki = wiki
        self.frontier = deque()
        self.visited = set()
        self.db = {}                 # url -> {hash, content, ts}
        self.order = []
        self.workers = ["w0", "w1", "w2"]
        self.req_count = {w: 0 for w in self.workers}
        self.blacklisted = set()
        self.last_access = {}        # domain -> tick (politeness)
        self.wi = 0

    def pick_worker(self):
        for _ in range(len(self.workers)):       # rotate, skip blacklisted
            w = self.workers[self.wi % len(self.workers)]
            self.wi += 1
            if w not in self.blacklisted:
                return w
        self.blacklisted.clear()                 # all out: refresh the pool
        return self.workers[0]

    def fetch(self, worker, url):
        self.last_access["wiki"] = self.last_access.get("wiki", 0) + 1   # politeness tick
        self.req_count[worker] += 1
        if self.req_count[worker] >= 2:          # rate-limited -> blacklist + rotate out
            self.blacklisted.add(worker)
        return self.wiki[url]

    def crawl(self, seed):
        self.frontier.append(seed)
        while self.frontier:
            url = self.frontier.popleft()
            if url in self.visited:
                continue
            self.visited.add(url)
            worker = self.pick_worker()
            content, links = self.fetch(worker, url)
            self.db[url] = {"hash": fnv1a(content), "content": content, "ts": len(self.order)}
            self.order.append(url)
            for l in links:
                if l not in self.visited:
                    self.frontier.append(l)
        return self.order

    def recrawl(self, url):
        content, _ = self.wiki[url]
        new_h = fnv1a(content)
        old_h = self.db[url]["hash"]
        if new_h != old_h:
            self.db[url] = {"hash": new_h, "content": content, "ts": self.db[url]["ts"]}
            return old_h, new_h
        return None


def main():
    wiki = {
        "Main": ("Welcome to the wiki", ["A", "B"]),
        "A": ("Page A content", ["C"]),
        "B": ("Page B content", ["C"]),
        "C": ("Page C content", ["Main"]),
    }
    c = Coordinator(wiki)
    order = c.crawl("Main")
    print("Crawled %d pages: [%s]" % (len(order), ", ".join(order)))

    # external edit to page C, then incremental re-crawl detects the change via content hash
    wiki["C"] = ("Page C content (edited 2026)", ["Main"])
    res = c.recrawl("C")
    if res:
        print("Re-crawl detected update on 'C': hash %08x -> %08x, re-stored." % res)


if __name__ == "__main__":
    main()
