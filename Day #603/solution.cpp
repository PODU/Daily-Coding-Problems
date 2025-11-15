// Day 603: Final resting state of pushed dominoes.
// Approach: two force passes (R rightward, L leftward), compare. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string pushDominoes(const string& s) {
    int n = s.size();
    vector<int> fR(n), fL(n);   // force pushing right / left
    int f = 0;
    for (int i = 0; i < n; i++) {
        if (s[i] == 'R') f = n;
        else if (s[i] == 'L') f = 0;
        else f = max(f - 1, 0);
        fR[i] = f;
    }
    f = 0;
    for (int i = n - 1; i >= 0; i--) {
        if (s[i] == 'L') f = n;
        else if (s[i] == 'R') f = 0;
        else f = max(f - 1, 0);
        fL[i] = f;
    }
    string res(n, '.');
    for (int i = 0; i < n; i++) {
        if (fR[i] > fL[i]) res[i] = 'R';
        else if (fR[i] < fL[i]) res[i] = 'L';
    }
    return res;
}

int main() {
    cout << pushDominoes(".L.R....L") << "\n"; // LL.RRRLLL
    cout << pushDominoes("..R...L.L") << "\n"; // ..RR.LLLL
    return 0;
}
