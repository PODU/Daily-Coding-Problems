// Longest consecutive sequence: hash set, extend only from sequence starts. O(n) time/space.
#include <bits/stdc++.h>
using namespace std;

int longestConsecutive(const vector<int>& a) {
    unordered_set<int> s(a.begin(), a.end());
    int best = 0;
    for (int x : s) {
        if (s.count(x - 1)) continue; // not a start
        int len = 1, cur = x;
        while (s.count(cur + 1)) { cur++; len++; }
        best = max(best, len);
    }
    return best;
}

int main() {
    cout << longestConsecutive({100, 4, 200, 1, 3, 2}) << "\n";
    return 0;
}
