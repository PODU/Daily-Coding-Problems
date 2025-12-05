// Day 695: Uniform random in [0, n-1] excluding values in list l.
// Approach: precompute the allowed values once, then pick a uniform index.
// Preprocess O(n), each draw O(1). (Rejection sampling is an O(1)-space alt.)
#include <bits/stdc++.h>
using namespace std;

struct RandExcluder {
    vector<int> allowed;
    mt19937 rng;
    RandExcluder(int n, const vector<int>& l, unsigned seed = 42) : rng(seed) {
        unordered_set<int> bad(l.begin(), l.end());
        for (int x = 0; x < n; ++x) if (!bad.count(x)) allowed.push_back(x);
    }
    int next() {
        uniform_int_distribution<int> d(0, (int)allowed.size() - 1);
        return allowed[d(rng)];
    }
};

int main() {
    RandExcluder r(10, {2, 4, 6, 8});
    cout << "sample: ";
    for (int i = 0; i < 8; ++i) cout << r.next() << " ";
    cout << "\n(all values are in [0,9] and never 2,4,6,8)\n";
    return 0;
}
