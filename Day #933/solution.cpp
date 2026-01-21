// Day 933: Reconstruct a permutation of [0..N] consistent with +/- signs.
// Two-pointer: '+' takes the current low, '-' takes the current high. O(n) time, O(n) space.
// Many arrangements are valid; README shows [1,2,3,0,4], this prints another valid one.
#include <bits/stdc++.h>
using namespace std;

// signs[0] is the sentinel for None; signs[i] in {'+','-'} for i>=1.
vector<int> reconstruct(const vector<char>& signs) {
    int n = (int)signs.size();      // number of elements
    int lo = 0, hi = n - 1;
    vector<int> res;
    res.reserve(n);
    for (int i = 1; i < n; ++i) {
        if (signs[i] == '+') res.push_back(lo++);
        else                 res.push_back(hi--);
    }
    res.push_back(lo);              // lo == hi here
    return res;
}

int main() {
    vector<char> signs = {'?', '+', '+', '-', '+'}; // [None, +, +, -, +]
    vector<int> r = reconstruct(signs);
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // e.g. [0, 1, 4, 2, 3] (a valid reconstruction)
    return 0;
}
