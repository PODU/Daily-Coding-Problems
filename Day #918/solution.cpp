// PageRank via power iteration. Dangling nodes distribute rank evenly.
// Time O(iters * (N+E)), Space O(N+E).
#include <iostream>
#include <vector>
#include <cmath>
#include <iomanip>
using namespace std;

int main() {
    int N = 4;
    vector<vector<int>> out = {{1,2}, {2}, {0,1}, {0,1,2}};
    double d = 0.85;
    vector<double> rank(N, 1.0/N);

    for (int iter = 0; iter < 1000; ++iter) {
        vector<double> next(N, (1.0 - d) / N);
        double dangling = 0.0;
        for (int i = 0; i < N; ++i)
            if (out[i].empty()) dangling += rank[i];
        for (int i = 0; i < N; ++i) {
            if (out[i].empty()) continue;
            double share = rank[i] / out[i].size();
            for (int j : out[i]) next[j] += d * share;
        }
        // distribute dangling rank evenly
        for (int j = 0; j < N; ++j) next[j] += d * dangling / N;
        double diff = 0.0;
        for (int j = 0; j < N; ++j) diff += fabs(next[j] - rank[j]);
        rank = next;
        if (diff < 1e-9) break;
    }
    for (int i = 0; i < N; ++i)
        cout << i << ": " << fixed << setprecision(4) << rank[i] << "\n";
    return 0;
}
