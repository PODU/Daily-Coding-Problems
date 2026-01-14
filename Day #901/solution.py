# Day 901: Distributed Wikipedia crawler (concrete simulation).
# Approach: BFS over a page link graph with a shared visited set (dedup) and a
# work frontier; in production the frontier is a distributed queue, visited is a
# sharded/bloom-filtered store, clients are rate-limited & rotated to avoid bans,
# and re-crawl is driven by RecentChanges feed + last-modified timestamps.
# Time: O(V + E) over reachable pages, Space: O(V).

from collections import deque


class CrawlerSystem:
    """
    Design summary (the heart of the problem):
      * Coverage: seed from special pages (AllPages / category roots), then BFS
        every internal /wiki/ link, plus periodically diff against Wikipedia's
        full page-id dump so orphan pages are not missed.
      * Dedup: a central, sharded "visited" service (URL -> hash) backed by a
        Bloom filter front for cheap negative checks; canonicalize URLs first.
      * Blacklisting: a pool of client machines/proxies with per-client rate
        limiting and exponential backoff; when one is blocked it is parked and
        traffic shifts to healthy clients.
      * Updates: subscribe to the MediaWiki RecentChanges API and use the
        revision/last-modified timestamp to re-crawl only changed pages.

    Below is a runnable in-memory simulation of the distributed loop.
    """

    def __init__(self, link_graph):
        self.link_graph = link_graph          # page -> list of linked pages
        self.visited = set()                  # central dedup store
        self.db = {}                          # page -> stored content

    def crawl(self, seeds):
        frontier = deque(seeds)               # distributed work queue
        for s in seeds:
            self.visited.add(s)
        while frontier:
            page = frontier.popleft()         # a client pops & fetches this page
            self.db[page] = f"content of {page}"
            for nxt in self.link_graph.get(page, []):
                if nxt not in self.visited:   # dedup before enqueue
                    self.visited.add(nxt)
                    frontier.append(nxt)
        return self.db


if __name__ == "__main__":
    graph = {
        "Main_Page": ["Python", "Wikipedia"],
        "Python":    ["Programming", "Wikipedia"],
        "Wikipedia": ["Python"],
        "Programming": [],
    }
    sys = CrawlerSystem(graph)
    db = sys.crawl(["Main_Page"])
    print("Pages crawled:", len(db))
    print("Visited:", sorted(db.keys()))
