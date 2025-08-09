// Day 90: Uniform random in [0,n) excluding l. Pick r in [0, n-k), then offset past
// sorted exclusions so every valid value is equally likely. Time O(k log k), Space O(k).
#include <bits/stdc++.h>
using namespace std;

int randomExcluding(int n, vector<int> l, mt19937& rng) {
    set<int> ex;
    for (int v : l) if (v >= 0 && v < n) ex.insert(v);
    int m = n - (int)ex.size();
    if (m <= 0) throw runtime_error("no valid number");
    uniform_int_distribution<int> dist(0, m - 1);
    int r = dist(rng);
    for (int e : ex) { if (e <= r) r++; else break; }
    return r;
}

int main() {
    mt19937 rng(42);
    int n = 5; vector<int> l = {1, 3};
    int x = randomExcluding(n, l, rng);
    cout << x << "\n"; // a uniform sample from {0,2,4}
    return 0;
}
