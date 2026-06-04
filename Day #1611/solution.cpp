// Orderly Queue: move one of the first k letters to the end repeatedly; find lexicographically smallest.
// k==1 -> smallest rotation; k>=2 -> sorted ascending. Time O(n^2) (k==1) / O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string smallest(const string& s, int k) {
    if (k >= 2) {
        string t = s;
        sort(t.begin(), t.end());
        return t;
    }
    // k == 1: smallest rotation
    string best = s;
    int n = s.size();
    for (int i = 1; i < n; ++i) {
        string rot = s.substr(i) + s.substr(0, i);
        if (rot < best) best = rot;
    }
    return best;
}

int main() {
    cout << smallest("daily", 1) << "\n";
    return 0;
}
