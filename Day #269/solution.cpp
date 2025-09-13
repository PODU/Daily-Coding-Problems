// Day 269: Push dominoes simulation via force/two-pointer scan.
// Left-to-right add +force from R, right-to-left add -force from L, sign decides. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string pushDominoes(const string& s) {
    int n = s.size();
    vector<int> force(n, 0);
    int f = 0;
    for (int i = 0; i < n; ++i) {                 // rightward push
        if (s[i] == 'R') f = n;
        else if (s[i] == 'L') f = 0;
        else f = max(f - 1, 0);
        force[i] += f;
    }
    f = 0;
    for (int i = n - 1; i >= 0; --i) {            // leftward push
        if (s[i] == 'L') f = n;
        else if (s[i] == 'R') f = 0;
        else f = max(f - 1, 0);
        force[i] -= f;
    }
    string res(n, '.');
    for (int i = 0; i < n; ++i)
        res[i] = force[i] > 0 ? 'R' : force[i] < 0 ? 'L' : '.';
    return res;
}

int main() {
    cout << pushDominoes(".L.R....L") << "\n";
    cout << pushDominoes("..R...L.L") << "\n";
    return 0;
}
