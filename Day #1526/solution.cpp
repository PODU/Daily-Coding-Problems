// PageRank: iterative power method with damping d=0.85, dangling-node mass redistributed evenly.
// Time O(iters*(N+E)), Space O(N+E).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<string> nodes = {"A", "B", "C", "D"};
    map<string, vector<string>> edges = {
        {"A", {"B", "C"}}, {"B", {"C"}}, {"C", {"A"}}, {"D", {"C"}}};
    int N = (int)nodes.size();
    double d = 0.85;
    map<string, double> score;
    for (auto& nd : nodes) score[nd] = 1.0 / N;

    for (int it = 0; it < 100; it++) {
        map<string, double> next;
        double dangling = 0.0;
        for (auto& nd : nodes)
            if (edges[nd].empty()) dangling += score[nd];
        for (auto& nd : nodes)
            next[nd] = (1.0 - d) / N + d * dangling / N;
        for (auto& nd : nodes) {
            auto& outs = edges[nd];
            if (outs.empty()) continue;
            double share = d * score[nd] / outs.size();
            for (auto& t : outs) next[t] += share;
        }
        double diff = 0.0;
        for (auto& nd : nodes) diff += fabs(next[nd] - score[nd]);
        score = next;
        if (diff < 1e-9) break;
    }

    cout << fixed << setprecision(6);
    for (auto& nd : nodes) cout << nd << " " << score[nd] << "\n";
    return 0;
}
