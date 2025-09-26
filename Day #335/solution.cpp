// PageRank via iterative power method, d=0.85. Iterate until convergence.
// Time: O(iters * (N + E)). Space: O(N + E).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<string> names = {"A", "B", "C", "D"};
    int N = names.size();
    map<string,int> idx; for (int i = 0; i < N; ++i) idx[names[i]] = i;
    vector<pair<string,string>> edges = {{"A","B"},{"A","C"},{"B","C"},{"C","A"},{"D","C"}};
    vector<vector<int>> in(N);
    vector<int> out(N, 0);
    for (auto& e : edges) { in[idx[e.second]].push_back(idx[e.first]); out[idx[e.first]]++; }

    double d = 0.85;
    vector<double> score(N, 1.0 / N);
    for (int it = 0; it < 1000; ++it) {
        vector<double> nxt(N, (1.0 - d) / N);
        for (int j = 0; j < N; ++j)
            for (int i : in[j])
                nxt[j] += d * score[i] / out[i];
        double diff = 0;
        for (int k = 0; k < N; ++k) diff += fabs(nxt[k] - score[k]);
        score = nxt;
        if (diff < 1e-9) break;
    }
    for (int i = 0; i < N; ++i)
        cout << names[i] << ": " << fixed << setprecision(4) << score[i] << "\n";
    return 0;
}
