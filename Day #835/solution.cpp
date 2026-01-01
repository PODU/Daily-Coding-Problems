// Day 835: Shortest substring containing all chars in a set.
// Sliding-window min-window: expand right, contract left while valid. O(N) time, O(K) space.
#include <bits/stdc++.h>
using namespace std;

string shortestSubstring(const string& s, const set<char>& charset) {
    unordered_map<char, int> need, have;
    for (char c : charset) need[c] = 1;
    int required = (int)need.size();
    int formed = 0, left = 0;
    bool found = false;
    int bestL = 0, bestLen = INT_MAX;
    for (int right = 0; right < (int)s.size(); ++right) {
        char ch = s[right];
        if (need.count(ch)) {
            if (++have[ch] == need[ch]) ++formed;
        }
        while (formed == required) {
            if (right - left + 1 < bestLen) {
                bestLen = right - left + 1;
                bestL = left;
                found = true;
            }
            char lc = s[left];
            if (need.count(lc)) {
                if (--have[lc] < need[lc]) --formed;
            }
            ++left;
        }
    }
    return found ? s.substr(bestL, bestLen) : string();
}

int main() {
    string s = "figehaeci";
    set<char> charset = {'a', 'e', 'i'};
    string res = shortestSubstring(s, charset);
    cout << (res.empty() ? "null" : res) << "\n";
    return 0;
}
