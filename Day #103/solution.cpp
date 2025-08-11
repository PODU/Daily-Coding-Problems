// Day 103: Shortest window containing all chars of a set via sliding window with
// a required-count and a "have all" tracker. O(n) time, O(set) space.
#include <bits/stdc++.h>
using namespace std;

string minWindow(const string& s, const set<char>& need) {
    if (need.empty()) return "";
    unordered_map<char, int> count;
    int have = 0, left = 0, bestLen = INT_MAX, bestStart = 0;
    for (int right = 0; right < (int)s.size(); right++) {
        char ch = s[right];
        if (need.count(ch) && ++count[ch] == 1) have++;
        while (have == (int)need.size()) {
            if (right - left + 1 < bestLen) { bestLen = right - left + 1; bestStart = left; }
            char lc = s[left];
            if (need.count(lc) && --count[lc] == 0) have--;
            left++;
        }
    }
    return bestLen == INT_MAX ? "" : s.substr(bestStart, bestLen);
}

int main() {
    cout << minWindow("figehaeci", {'a', 'e', 'i'}) << "\n"; // aeci
    return 0;
}
