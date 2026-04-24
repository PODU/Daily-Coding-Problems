// Day 1413: shortest substring of s containing all characters in a set.
// Approach: sliding window, expand right then shrink left while valid. O(n) time, O(k) space.
#include <bits/stdc++.h>
using namespace std;

string shortestSubstring(const string& s, const set<char>& want) {
    unordered_map<char,int> need;
    for (char c : want) need[c]++;
    int required = (int)need.size(), formed = 0;
    unordered_map<char,int> win;
    int bestLen = INT_MAX, bestL = 0;
    int l = 0;
    for (int r = 0; r < (int)s.size(); ++r) {
        char c = s[r];
        if (need.count(c)) {
            win[c]++;
            if (win[c] == need[c]) formed++;
        }
        while (formed == required) {
            if (r - l + 1 < bestLen) { bestLen = r - l + 1; bestL = l; }
            char lc = s[l];
            if (need.count(lc)) {
                win[lc]--;
                if (win[lc] < need[lc]) formed--;
            }
            l++;
        }
    }
    return bestLen == INT_MAX ? string("null") : s.substr(bestL, bestLen);
}

int main() {
    string s = "figehaeci";
    set<char> want = {'a','e','i'};
    cout << shortestSubstring(s, want) << "\n"; // aeci
    return 0;
}
