// Dominoes final state via two-pass force summation. Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

string dominoes(const string& s) {
    int n = s.size();
    vector<int> forces(n, 0);
    // Left to right: accumulate R forces
    int f = 0;
    for (int i = 0; i < n; i++) {
        if      (s[i]=='R') f = n;
        else if (s[i]=='L') f = 0;
        else f = max(0, f-1);
        forces[i] += f;
    }
    // Right to left: subtract L forces
    f = 0;
    for (int i = n-1; i >= 0; i--) {
        if      (s[i]=='L') f = n;
        else if (s[i]=='R') f = 0;
        else f = max(0, f-1);
        forces[i] -= f;
    }
    string res(n, '.');
    for (int i = 0; i < n; i++) {
        if      (forces[i] > 0) res[i] = 'R';
        else if (forces[i] < 0) res[i] = 'L';
    }
    return res;
}

int main() {
    cout << dominoes(".L.R....L") << "\n";
    cout << dominoes("..R...L.L") << "\n";
}
