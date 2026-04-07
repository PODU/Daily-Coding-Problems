// Move one of first k chars to the end, unlimited times.
// k==1: only rotations reachable -> smallest rotation. k>=2: any order -> sort.
// Time O(n^2) for k==1 (rotation scan), O(n log n) for k>=2.
#include <bits/stdc++.h>
using namespace std;

string smallestString(const string& s, int k) {
    if (k >= 2) { string t = s; sort(t.begin(), t.end()); return t; }
    string best = s;
    for (size_t i = 1; i < s.size(); i++) {
        string rot = s.substr(i) + s.substr(0, i);
        best = min(best, rot);
    }
    return best;
}

int main() {
    cout << smallestString("daily", 1) << "\n"; // ailyd
    return 0;
}
