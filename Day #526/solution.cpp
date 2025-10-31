// k==1: smallest rotation (try all n rotations). k>=2: sorted string (any pair
// of front letters can be reordered). Time O(n^2) for k==1, O(n log n) k>=2.
#include <bits/stdc++.h>
using namespace std;

string smallest(const string& s, int k) {
    if (k >= 2) {
        string t = s;
        sort(t.begin(), t.end());
        return t;
    }
    int n = (int)s.size();
    string best = s;
    for (int i = 1; i < n; ++i) {
        string rot = s.substr(i) + s.substr(0, i);
        best = min(best, rot);
    }
    return best;
}

int main() {
    cout << smallest("daily", 1) << "\n";
    return 0;
}
