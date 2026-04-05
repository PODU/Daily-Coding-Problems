// Day 1304: Uniformly sample an integer in [0, n) not in list l.
// Precompute sorted excluded; pick r in [0, n-|excl|) and map to the r-th allowed value.
// Preprocess O(m log m); each draw O(m). Uniform over all allowed values.
#include <bits/stdc++.h>
using namespace std;

class RandExcluder {
    int n;
    vector<int> excl; // sorted, within [0,n), deduped
    mt19937 rng;
public:
    RandExcluder(int n_, vector<int> l, unsigned seed = 42) : n(n_), rng(seed) {
        set<int> s;
        for (int x : l) if (x >= 0 && x < n) s.insert(x);
        excl.assign(s.begin(), s.end());
    }
    int next() {
        int count = n - (int)excl.size();
        uniform_int_distribution<int> dist(0, count - 1);
        int res = dist(rng);
        for (int e : excl) { if (e <= res) res++; else break; }
        return res;
    }
};

int main() {
    RandExcluder r(5, {1, 3});
    set<int> seen;
    for (int i = 0; i < 1000; i++) seen.insert(r.next());
    cout << "{"; bool f = true;
    for (int v : seen) { cout << (f ? "" : ", ") << v; f = false; }
    cout << "}" << endl; // {0, 2, 4} -- excludes 1 and 3
    return 0;
}
