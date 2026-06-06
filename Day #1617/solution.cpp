// Falling dominoes: two-pass force accumulation (R adds +, L adds -, decay between).
// Sign of net force decides L/R/.; equal force stays '.'. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string pushDominoes(const string &d) {
    int n = d.size();
    vector<int> force(n, 0);
    int f = 0;
    for (int i = 0; i < n; ++i) {            // left-to-right: R pushes right
        if (d[i] == 'R') f = n;
        else if (d[i] == 'L') f = 0;
        else f = max(f - 1, 0);
        force[i] += f;
    }
    f = 0;
    for (int i = n - 1; i >= 0; --i) {       // right-to-left: L pushes left
        if (d[i] == 'L') f = n;
        else if (d[i] == 'R') f = 0;
        else f = max(f - 1, 0);
        force[i] -= f;
    }
    string res(n, '.');
    for (int i = 0; i < n; ++i)
        res[i] = force[i] > 0 ? 'R' : (force[i] < 0 ? 'L' : '.');
    return res;
}

int main() {
    cout << pushDominoes(".L.R....L") << "\n";
    cout << pushDominoes("..R...L.L") << "\n";
    return 0;
}
