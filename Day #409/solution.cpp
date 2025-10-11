// Day 409: PageRank via power iteration with damping d=0.85.
// Approach: iterate score(j)=(1-d)/N + d*sum(score(i)/Ci); dangling rank
// is spread over all nodes. Time: O(iters*(N+E)), Space: O(N+E).
#include <bits/stdc++.h>
using namespace std;

map<string,double> pageRank(const map<string,vector<string>>& graph,
                            double d = 0.85, int iters = 100, double eps = 1e-12) {
    int N = graph.size();
    map<string,double> rank;
    for (const auto& kv : graph) rank[kv.first] = 1.0 / N;
    for (int it = 0; it < iters; it++) {
        map<string,double> next;
        // Dangling nodes (no out-links) leak rank; redistribute it evenly.
        double dangling = 0.0;
        for (const auto& kv : graph)
            if (kv.second.empty()) dangling += rank[kv.first];
        for (const auto& kv : graph)
            next[kv.first] = (1.0 - d) / N + d * dangling / N;
        for (const auto& kv : graph) {
            const vector<string>& outs = kv.second;
            if (outs.empty()) continue;
            double share = rank[kv.first] / outs.size();
            for (const auto& nbr : outs) next[nbr] += d * share;
        }
        double diff = 0.0;
        for (const auto& kv : graph) diff += fabs(next[kv.first] - rank[kv.first]);
        rank = next;
        if (diff < eps) break;
    }
    return rank;
}

int main() {
    map<string,vector<string>> graph = {
        {"A", {"B", "C"}},
        {"B", {"C"}},
        {"C", {"A"}},
    };
    auto rank = pageRank(graph);
    cout << fixed << setprecision(4);
    for (const auto& kv : rank)
        cout << kv.first << ": " << kv.second << "\n";
    return 0;
}
