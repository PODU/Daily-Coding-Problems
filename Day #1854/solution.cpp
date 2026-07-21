// Day 1854: Shortest substring containing all chars in a set.
// Sliding window with a need-counter; expand right, contract left while valid. O(n) time, O(|set|) space.
#include <bits/stdc++.h>
using namespace std;

// returns substring, or empty optional if none
optional<string> shortestSubstring(const string& s, const set<char>& need) {
    unordered_map<char, int> want;
    for (char c : need) want[c]++;
    int required = (int)want.size(), formed = 0;
    unordered_map<char, int> win;
    int bestLen = INT_MAX, bestL = 0, l = 0;
    for (int r = 0; r < (int)s.size(); r++) {
        char c = s[r];
        if (want.count(c)) {
            if (++win[c] == want[c]) formed++;
        }
        while (formed == required) {
            if (r - l + 1 < bestLen) { bestLen = r - l + 1; bestL = l; }
            char lc = s[l++];
            if (want.count(lc) && --win[lc] < want[lc]) formed--;
        }
    }
    if (bestLen == INT_MAX) return nullopt;
    return s.substr(bestL, bestLen);
}

int main() {
    auto res = shortestSubstring("figehaeci", {'a', 'e', 'i'});
    cout << (res ? *res : "null") << "\n"; // aeci
    return 0;
}
