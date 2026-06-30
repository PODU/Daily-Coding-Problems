// Approach: Single-process simulation of a distributed BFS web crawler. Central frontier (queue) +
// visited set for dedup; N round-robin workers fetch outlinks from a mock graph, store to a mock DB,
// requeue work from a blacklisted worker. Time O(V+E), Space O(V+E).
//
// Real distributed design: a master holds the frontier and shards the "URL-seen" set across nodes via
// consistent hashing or a bloom filter; workers crawl politely (robots.txt, rate limits, rotating IPs)
// to avoid blacklisting; recrawl uses last-modified/ETag to detect and update changed pages.

const graph = {
  Main: ["A", "B", "C"],
  A: ["B", "D"],
  B: ["C"],
  C: ["A", "E"],
  D: ["E"],
  E: [],
};

function crawl(seed, numWorkers) {
  const frontier = [seed];
  const visited = new Set([seed]);
  const db = new Map();
  const blacklisted = new Set();
  let worker = 0;
  let processedAny = false;

  while (frontier.length > 0) {
    const url = frontier.shift();
    const w = worker % numWorkers;
    worker++;

    // Blacklist worker #1 after at least one page processed; requeue its task.
    if (w === 1 && processedAny && !blacklisted.has(1)) {
      blacklisted.add(1);
      frontier.push(url); // in-flight task requeued, no page lost
      continue;
    }

    db.set(url, graph[url] || []); // "fetch" + store to mock DB
    processedAny = true;

    for (const link of graph[url] || []) {
      if (!visited.has(link)) {
        visited.add(link);
        frontier.push(link);
      }
    }
  }
  return db;
}

function main() {
  const db = crawl("Main", 3);
  console.log(`Crawled ${db.size} pages`);
  console.log([...db.keys()].sort().join(" "));
}

main();
