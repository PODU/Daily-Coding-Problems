// Day 901: Distributed Wikipedia crawler (concrete simulation).
// Approach: BFS over a page link graph with a shared visited set (dedup) and a
// frontier queue. In production: distributed frontier queue, sharded/bloom
// visited store, rotating rate-limited clients to dodge bans, RecentChanges +
// last-modified driven re-crawl. Time: O(V+E), Space: O(V).

class CrawlerSystem {
  constructor(linkGraph) {
    this.linkGraph = linkGraph;   // page -> array of linked pages
    this.visited = new Set();     // central dedup store
    this.db = new Map();          // page -> stored content
  }

  crawl(seeds) {
    const frontier = [...seeds];  // distributed work queue
    let head = 0;
    seeds.forEach((s) => this.visited.add(s));
    while (head < frontier.length) {
      const page = frontier[head++];          // a client fetches this page
      this.db.set(page, `content of ${page}`);
      for (const nxt of this.linkGraph[page] || []) {
        if (!this.visited.has(nxt)) {         // dedup before enqueue
          this.visited.add(nxt);
          frontier.push(nxt);
        }
      }
    }
    return this.db;
  }
}

const graph = {
  Main_Page: ["Python", "Wikipedia"],
  Python: ["Programming", "Wikipedia"],
  Wikipedia: ["Python"],
  Programming: [],
};
const sys = new CrawlerSystem(graph);
const db = sys.crawl(["Main_Page"]);
console.log("Pages crawled:", db.size);
console.log("Visited:", [...db.keys()].sort());
