// Day 1146: Dominoes - two-pass force accumulation.
// L->R pass adds rightward force, R->L pass adds leftward; sign decides. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

string pushDominoes(const string& s) {
    int n = s.size();
    vector<int> forces(n, 0);
    int force = 0;
    for (int i = 0; i < n; ++i) {
        if (s[i] == 'R') force = n; else if (s[i] == 'L') force = 0; else force = max(force - 1, 0);
        forces[i] += force;
    }
    force = 0;
    for (int i = n - 1; i >= 0; --i) {
        if (s[i] == 'L') force = n; else if (s[i] == 'R') force = 0; else force = max(force - 1, 0);
        forces[i] -= force;
    }
    string res(n, '.');
    for (int i = 0; i < n; ++i) res[i] = forces[i] > 0 ? 'R' : forces[i] < 0 ? 'L' : '.';
    return res;
}

int main() {
    cout << pushDominoes(".L.R....L") << "\n"; // LL.RRRLLL
    cout << pushDominoes("..R...L.L") << "\n"; // ..RR.LLLL
    return 0;
}
