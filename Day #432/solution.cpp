// Day 432: Distributed Wikipedia crawler — concrete single-process simulation.
// Design: a central coordinator holds a URL frontier (BFS queue) + visited set for dedup.
// Workers pull URLs, "fetch" pages from an in-memory graph, parse links, store content keyed
// by a content hash + last-crawled timestamp, and push new URLs back. Politeness delay per
// domain + worker blacklist/rotation prevent overload. Incremental re-crawl compares the
// stored content hash to detect updates. O(V+E) over the page graph.
#include <bits/stdc++.h>
using namespace std;

struct Page { string content; vector<string> links; };

uint32_t fnv1a(const string& s) {
    uint32_t h = 2166136261u;
    for (unsigned char b : s) { h ^= b; h *= 16777619u; }
    return h;
}

struct Coordinator {
    map<string, Page> wiki;
    deque<string> frontier;
    set<string> visited;
    map<string, pair<uint32_t,int>> db;   // url -> {hash, ts}
    vector<string> order;
    vector<string> workers{"w0", "w1", "w2"};
    map<string,int> reqCount;
    set<string> blacklisted;
    long long lastAccess = 0;
    int wi = 0;

    string pickWorker() {
        for (size_t k = 0; k < workers.size(); k++) {
            string w = workers[wi % workers.size()];
            wi++;
            if (!blacklisted.count(w)) return w;
        }
        blacklisted.clear();
        return workers[0];
    }
    Page fetch(const string& worker, const string& url) {
        lastAccess++;                                   // politeness tick
        if (++reqCount[worker] >= 2) blacklisted.insert(worker);  // rate-limit -> rotate out
        return wiki[url];
    }
    vector<string> crawl(const string& seed) {
        frontier.push_back(seed);
        while (!frontier.empty()) {
            string url = frontier.front(); frontier.pop_front();
            if (visited.count(url)) continue;
            visited.insert(url);
            string worker = pickWorker();
            Page p = fetch(worker, url);
            db[url] = {fnv1a(p.content), (int)order.size()};
            order.push_back(url);
            for (auto& l : p.links) if (!visited.count(l)) frontier.push_back(l);
        }
        return order;
    }
    bool recrawl(const string& url, uint32_t& oh, uint32_t& nh) {
        nh = fnv1a(wiki[url].content);
        oh = db[url].first;
        if (nh != oh) { db[url] = {nh, db[url].second}; return true; }
        return false;
    }
};

int main() {
    Coordinator c;
    c.wiki["Main"] = {"Welcome to the wiki", {"A", "B"}};
    c.wiki["A"]    = {"Page A content", {"C"}};
    c.wiki["B"]    = {"Page B content", {"C"}};
    c.wiki["C"]    = {"Page C content", {"Main"}};

    auto order = c.crawl("Main");
    cout << "Crawled " << order.size() << " pages: [";
    for (size_t i = 0; i < order.size(); i++) cout << (i ? ", " : "") << order[i];
    cout << "]\n";

    c.wiki["C"] = {"Page C content (edited 2026)", {"Main"}};
    uint32_t oh, nh;
    if (c.recrawl("C", oh, nh))
        printf("Re-crawl detected update on 'C': hash %08x -> %08x, re-stored.\n", oh, nh);
}
