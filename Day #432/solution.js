// Day 432: Distributed Wikipedia crawler — concrete single-process simulation.
// Design: a central coordinator holds a URL frontier (BFS queue) + visited set for dedup.
// Workers pull URLs, "fetch" pages from an in-memory graph, parse links, store content keyed
// by a content hash + last-crawled timestamp, and push new URLs back. Politeness delay per
// domain + worker blacklist/rotation prevent overload. Incremental re-crawl compares the
// stored content hash to detect updates. O(V+E) over the page graph.

function fnv1a(s) {
    let h = 2166136261 >>> 0;
    for (const b of Buffer.from(s, 'utf-8')) {
        h ^= b;
        h = Math.imul(h, 16777619) >>> 0;
    }
    return h >>> 0;
}
function hex8(x) { return x.toString(16).padStart(8, '0'); }

class Coordinator {
    constructor(wiki) {
        this.wiki = wiki;
        this.frontier = [];
        this.visited = new Set();
        this.db = new Map();
        this.order = [];
        this.workers = ["w0", "w1", "w2"];
        this.reqCount = new Map(this.workers.map(w => [w, 0]));
        this.blacklisted = new Set();
        this.lastAccess = 0;
        this.wi = 0;
    }
    pickWorker() {
        for (let k = 0; k < this.workers.length; k++) {
            const w = this.workers[this.wi % this.workers.length];
            this.wi++;
            if (!this.blacklisted.has(w)) return w;
        }
        this.blacklisted.clear();
        return this.workers[0];
    }
    fetch(worker, url) {
        this.lastAccess++;                                  // politeness tick
        this.reqCount.set(worker, this.reqCount.get(worker) + 1);
        if (this.reqCount.get(worker) >= 2) this.blacklisted.add(worker); // rate-limit
        return this.wiki.get(url);
    }
    crawl(seed) {
        this.frontier.push(seed);
        while (this.frontier.length) {
            const url = this.frontier.shift();
            if (this.visited.has(url)) continue;
            this.visited.add(url);
            const worker = this.pickWorker();
            const [content, links] = this.fetch(worker, url);
            this.db.set(url, { hash: fnv1a(content), ts: this.order.length });
            this.order.push(url);
            for (const l of links) if (!this.visited.has(l)) this.frontier.push(l);
        }
        return this.order;
    }
    recrawl(url) {
        const [content] = this.wiki.get(url);
        const nh = fnv1a(content);
        const oh = this.db.get(url).hash;
        if (nh !== oh) {
            this.db.set(url, { hash: nh, ts: this.db.get(url).ts });
            return [oh, nh];
        }
        return null;
    }
}

const wiki = new Map([
    ["Main", ["Welcome to the wiki", ["A", "B"]]],
    ["A", ["Page A content", ["C"]]],
    ["B", ["Page B content", ["C"]]],
    ["C", ["Page C content", ["Main"]]],
]);
const c = new Coordinator(wiki);
const order = c.crawl("Main");
console.log(`Crawled ${order.length} pages: [${order.join(", ")}]`);

wiki.set("C", ["Page C content (edited 2026)", ["Main"]]);
const res = c.recrawl("C");
if (res) console.log(`Re-crawl detected update on 'C': hash ${hex8(res[0])} -> ${hex8(res[1])}, re-stored.`);
