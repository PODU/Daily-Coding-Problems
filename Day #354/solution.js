// Distributed crawler design (simulation): Coordinator holds a FIFO URL frontier + central visited set.
// Workers pull a URL, "download" (map lookup), record it, push unvisited links. BFS over a mock graph.
// Architecture: reach pages = BFS from seeds; visited = central set/bloom filter on coordinator;
// blacklisting = rotate IPs/user-agents, rate-limit + backoff; updates = recrawl by last-modified queue.
// Time O(V+E), Space O(V).
"use strict";

function crawl(wiki, seeds) {
  const frontier = [];
  const visited = new Set();
  const order = [];
  for (const s of seeds) {
    if (!visited.has(s)) { visited.add(s); frontier.push(s); }
  }
  let head = 0;
  while (head < frontier.length) {
    const url = frontier[head++];
    order.push(url); // "download" + record into results DB
    for (const link of (wiki[url] || [])) {
      if (!visited.has(link)) { visited.add(link); frontier.push(link); }
    }
  }
  return order;
}

function main() {
  const wiki = {
    "/Main":   ["/Apple", "/Banana"],
    "/Apple":  ["/Banana", "/Fruit"],
    "/Banana": ["/Fruit"],
    "/Fruit":  ["/Main"],
  };
  const order = crawl(wiki, ["/Main"]);
  console.log(`Crawled ${order.length} pages:`);
  for (const p of order) console.log(p);
}

main();
