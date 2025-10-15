// Day 437: Shortest substring containing all chars of a set via sliding window.
// O(N) time, O(set) space.
#include <bits/stdc++.h>
using namespace std;

string shortestSubstring(const string& s, const set<char>& chars) {
    if (chars.empty()) return "";
    unordered_map<char,int> need;
    for (char c : chars) need[c] = 0;
    int required = chars.size(), formed = 0;
    int bestLen = INT_MAX, bestL = 0;
    int l = 0;
    for (int r = 0; r < (int)s.size(); ++r) {
        char c = s[r];
        if (need.count(c)) {
            if (need[c] == 0) formed++;
            need[c]++;
        }
        while (formed == required) {
            if (r - l + 1 < bestLen) { bestLen = r - l + 1; bestL = l; }
            char lc = s[l];
            if (need.count(lc)) {
                need[lc]--;
                if (need[lc] == 0) formed--;
            }
            l++;
        }
    }
    return bestLen == INT_MAX ? string() : s.substr(bestL, bestLen);
}

int main() {
    string s = "figehaeci";
    set<char> chars = {'a','e','i'};
    string res = shortestSubstring(s, chars);
    cout << (res.empty() ? "null" : "\"" + res + "\"") << "\n"; // "aeci"
    return 0;
}
