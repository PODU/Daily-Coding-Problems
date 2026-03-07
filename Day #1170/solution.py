# Day 1170: Distributed Wikipedia crawler simulation: master holds a FIFO URL frontier +
# visited set (dedup); workers "fetch" mock pages, parse links, report back; BFS.
# Real design: shard frontier by URL hash (load-balance), distributed visited set
# (bloom filter / consistent hashing), rotating IPs + politeness/backoff to avoid
# blacklisting, recrawl via Last-Modified timestamps. Time/Space: O(V + E).
from collections import deque

# Mock in-memory "Wikipedia" link graph (adjacency, ordered).
GRAPH = {
    "Wikipedia": ["Computer_Science", "Mathematics"],
    "Computer_Science": ["Algorithms", "Mathematics"],
    "Mathematics": ["Algorithms"],
    "Algorithms": [],
}


def fetch(url):
    """Worker: fetch a mock page and parse out its links."""
    return GRAPH.get(url, [])


def crawl(seed):
    frontier = deque([seed])   # master FIFO frontier
    seen = {seed}              # visited/enqueued dedup set
    db = {}                    # crawled "database"
    while frontier:
        url = frontier.popleft()
        links = fetch(url)     # worker reports content + links
        db[url] = links
        print("Crawled: " + url)
        for nxt in links:
            if nxt not in seen:
                seen.add(nxt)
                frontier.append(nxt)
    return db


if __name__ == "__main__":
    crawl("Wikipedia")
