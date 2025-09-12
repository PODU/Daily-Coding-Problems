// Day 260: Reconstruct a permutation of [0..N] consistent with +/- sign array.
// Grow a list: on '+' append (currentMax+1), on '-' append (currentMin-1); then
// shift everything by -min into the range [0..N]. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> reconstruct(const vector<int>& signs) { // signs[0] is sentinel (None)
    vector<int> res{0};
    int curMax = 0, curMin = 0;
    for (size_t i = 1; i < signs.size(); ++i) {
        if (signs[i] > 0) res.push_back(++curMax);
        else              res.push_back(--curMin);
    }
    int off = -curMin;
    for (int& x : res) x += off;
    return res;
}

int main() {
    // Input: [None, +, +, -, +]  (encode None=0, +=1, -=-1)
    vector<int> signs = {0, 1, 1, -1, 1};
    vector<int> r = reconstruct(signs);
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) { if (i) cout << ", "; cout << r[i]; }
    cout << "]" << endl;
}
