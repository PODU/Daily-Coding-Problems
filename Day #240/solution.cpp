// Couples holding hands: union the two couples occupying each seat-pair. Minimum swaps =
// N - (number of connected components among couples). Time: O(N alpha), Space: O(N).
#include <bits/stdc++.h>
using namespace std;

struct DSU {
    vector<int> p;
    int comps;
    DSU(int n) : p(n), comps(n) { iota(p.begin(), p.end(), 0); }
    int find(int x) { return p[x] == x ? x : p[x] = find(p[x]); }
    void unite(int a, int b) {
        a = find(a); b = find(b);
        if (a != b) { p[a] = b; comps--; }
    }
};

int minSwaps(const vector<int>& row) {
    int n = row.size() / 2;
    DSU dsu(n);
    for (int i = 0; i < (int)row.size(); i += 2)
        dsu.unite(row[i] / 2, row[i + 1] / 2);
    return n - dsu.comps;
}

int main() {
    vector<int> row = {0, 2, 1, 3};
    cout << minSwaps(row) << "\n"; // 1
}
