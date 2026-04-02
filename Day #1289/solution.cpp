// Day 1289: PageRank via power iteration with damping factor d.
// Iterate score vector until convergence; dangling nodes spread mass uniformly.
// Time O(iters * (V + E)), Space O(V + E).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<string> nodes = {"A", "B", "C", "D"};
    map<string, vector<string>> links = {
        {"A", {"B", "C"}}, {"B", {"C"}}, {"C", {"A"}}, {"D", {"C"}}};
    double d = 0.85;
    int iters = 100;
    int n = nodes.size();

    map<string, int> outCount;
    for (auto& node : nodes) outCount[node] = links.count(node) ? links[node].size() : 0;
    map<string, double> score;
    for (auto& node : nodes) score[node] = 1.0 / n;

    for (int it = 0; it < iters; ++it) {
        map<string, double> nw;
        double danglingSum = 0;
        for (auto& node : nodes) if (outCount[node] == 0) danglingSum += score[node];
        for (auto& node : nodes) nw[node] = (1 - d) / n + d * danglingSum / n;
        for (auto& src : nodes) {
            if (outCount[src] == 0) continue;
            double share = d * score[src] / outCount[src];
            for (auto& dst : links[src]) nw[dst] += share;
        }
        score = nw;
    }
    for (auto& node : nodes)
        cout << node << ": " << fixed << setprecision(4) << score[node] << "\n";
    return 0;
}
