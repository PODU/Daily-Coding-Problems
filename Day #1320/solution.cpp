// Longest substring with at most k distinct characters via a sliding window
// with a char-count map. Time O(n), Space O(k).
#include <bits/stdc++.h>
using namespace std;

string longestKDistinct(const string& s, int k) {
    unordered_map<char,int> cnt;
    int left = 0, bestStart = 0, bestLen = 0;
    for (int right = 0; right < (int)s.size(); right++) {
        cnt[s[right]]++;
        while ((int)cnt.size() > k) {
            if (--cnt[s[left]] == 0) cnt.erase(s[left]);
            left++;
        }
        if (right - left + 1 > bestLen) { bestLen = right - left + 1; bestStart = left; }
    }
    return s.substr(bestStart, bestLen);
}

int main() {
    string sub = longestKDistinct("abcba", 2);
    cout << "The longest substring with k distinct characters is \"" << sub << "\".\n";
    return 0;
}
