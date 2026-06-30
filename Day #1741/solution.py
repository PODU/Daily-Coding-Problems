# Day 1741: Approach: Single-process simulation of a distributed BFS web crawler. Central frontier (queue) +
# visited set for dedup; N round-robin workers fetch outlinks from a mock graph, store to a mock DB,
# requeue work from a blacklisted worker. Time O(V+E), Space O(V+E).
#
# Real distributed design: a master holds the frontier and shards the "URL-seen" set across nodes via
# consistent hashing or a bloom filter; workers crawl politely (robots.txt, rate limits, rotating IPs)
# to avoid blacklisting; recrawl uses last-modified/ETag to detect and update changed pages.

from collections import deque

GRAPH = {
    "Main": ["A", "B", "C"],
    "A": ["B", "D"],
    "B": ["C"],
    "C": ["A", "E"],
    "D": ["E"],
    "E": [],
}


def crawl(seed, num_workers):
    frontier = deque([seed])
    visited = {seed}
    db = {}
    blacklisted = set()
    worker = 0
    processed_any = False

    while frontier:
        url = frontier.popleft()
        w = worker % num_workers
        worker += 1

        # Blacklist worker #1 after at least one page has been processed; requeue its task.
        if w == 1 and processed_any and 1 not in blacklisted:
            blacklisted.add(1)
            frontier.append(url)  # in-flight task requeued, no page lost
            continue

        # "Fetch" page: look up outlinks, store content to mock DB.
        db[url] = list(GRAPH.get(url, []))
        processed_any = True

        for link in GRAPH.get(url, []):
            if link not in visited:
                visited.add(link)
                frontier.append(link)

    return db


def main():
    db = crawl("Main", 3)
    print(f"Crawled {len(db)} pages")
    print(" ".join(sorted(db.keys())))


if __name__ == "__main__":
    main()
