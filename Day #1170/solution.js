// Distributed Wikipedia crawler simulation: master holds a FIFO URL frontier +
// visited set (dedup); workers "fetch" mock pages, parse links, report back; BFS.
// Real design: shard frontier by URL hash (load-balance), distributed visited set
// (bloom filter / consistent hashing), rotating IPs + politeness/backoff to avoid
// blacklisting, recrawl via Last-Modified timestamps. Time/Space: O(V + E).
'use strict';

// Mock in-memory "Wikipedia" link graph (adjacency, ordered).
const GRAPH = {
  Wikipedia: ['Computer_Science', 'Mathematics'],
  Computer_Science: ['Algorithms', 'Mathematics'],
  Mathematics: ['Algorithms'],
  Algorithms: [],
};

function fetch(url) { // worker: fetch mock page + parse links
  return GRAPH[url] || [];
}

function crawl(seed) {
  const frontier = [seed];      // master FIFO frontier
  let head = 0;
  const seen = new Set([seed]); // dedup set
  const db = {};                // crawled database
  while (head < frontier.length) {
    const url = frontier[head++];
    const links = fetch(url);   // worker reports content + links
    db[url] = links;
    console.log('Crawled: ' + url);
    for (const nxt of links) {
      if (!seen.has(nxt)) { seen.add(nxt); frontier.push(nxt); }
    }
  }
  return db;
}

crawl('Wikipedia');
