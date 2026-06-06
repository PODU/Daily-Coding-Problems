// Day 1622: Longest substring with at most k distinct characters.
// Sliding window with a count map. Time O(n), Space O(k).
#include <bits/stdc++.h>
using namespace std;

string longestKDistinct(const string& s, int k) {
    if (k <= 0) return "";
    unordered_map<char, int> cnt;
    int left = 0, bestL = 0, bestLen = 0;
    for (int right = 0; right < (int)s.size(); right++) {
        cnt[s[right]]++;
        while ((int)cnt.size() > k) {
            if (--cnt[s[left]] == 0) cnt.erase(s[left]);
            left++;
        }
        if (right - left + 1 > bestLen) { bestLen = right - left + 1; bestL = left; }
    }
    return s.substr(bestL, bestLen);
}

int main() {
    cout << "\"" << longestKDistinct("abcba", 2) << "\"" << endl;
    return 0;
}
