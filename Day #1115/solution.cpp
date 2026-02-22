// Day 1115 - Uniform random in [0, n) excluding list l
// Approach: remap |E| excluded slots below m=n-|E| to available high slots,
// then sample once in [0, m). Time: O(|E|) prep, O(1)/sample.
#include <bits/stdc++.h>
using namespace std;

struct Sampler {
    int n, m;
    unordered_map<int,int> mapping;
    mt19937 rng;
    Sampler(int n_, const vector<int>& l) : n(n_), rng(random_device{}()) {
        set<int> excluded;
        for (int x : l) if (x >= 0 && x < n) excluded.insert(x);
        m = n - (int)excluded.size();
        vector<int> available;
        for (int v = m; v < n; ++v) if (!excluded.count(v)) available.push_back(v);
        int ai = 0;
        for (int e : excluded) if (e < m) mapping[e] = available[ai++];
    }
    int sample() {
        uniform_int_distribution<int> dist(0, m - 1);
        int r = dist(rng);
        auto it = mapping.find(r);
        return it == mapping.end() ? r : it->second;
    }
};

int main() {
    int n = 10; vector<int> l = {2, 5, 7};
    Sampler s(n, l);
    set<int> seen;
    for (int i = 0; i < 2000; ++i) seen.insert(s.sample());
    cout << "n=" << n << ", excluded=[2, 5, 7]\n";
    cout << "Sampled valid numbers: [";
    int i = 0;
    for (int v : seen) cout << v << (++i < (int)seen.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
