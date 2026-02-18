// Day 1097: Smallest string by moving one of first k letters to the end.
// If k==1 only rotations reachable -> min rotation; if k>=2 any order -> sorted.
// Time: O(N^2) for k==1 (N rotations), O(N log N) for k>=2. Space: O(N).
#include <bits/stdc++.h>
using namespace std;

string smallest(const string& s, int k) {
    if (k >= 2) { string t = s; sort(t.begin(), t.end()); return t; }
    string best = s;
    string cur = s;
    for (int i = 0; i < (int)s.size(); i++) {        // try all rotations
        cur = cur.substr(1) + cur[0];
        best = min(best, cur);
    }
    return best;
}

int main() {
    cout << smallest("daily", 1) << "\n"; // ailyd
    return 0;
}
