// Day 347: Lexicographically smallest string by moving one of first k letters to the end.
// k==1 -> best rotation; k>=2 -> any permutation reachable, so sorted. Time O(N^2)/O(N log N).
#include <bits/stdc++.h>
using namespace std;

string smallest(string s, int k) {
    if (k == 1) {
        string best = s;
        for (size_t i = 1; i < s.size(); ++i) {
            string rot = s.substr(i) + s.substr(0, i);
            best = min(best, rot);
        }
        return best;
    }
    sort(s.begin(), s.end());
    return s;
}

int main() {
    cout << smallest("daily", 1) << "\n";
    return 0;
}
