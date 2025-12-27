// Day 807: Longest substring with at most k distinct characters.
// Sliding window + char count map; shrink left when distinct > k. Time O(N), Space O(k).
#include <bits/stdc++.h>
using namespace std;

int longestKDistinct(const string& s, int k) {
    if (k == 0) return 0;
    unordered_map<char, int> cnt;
    int left = 0, best = 0;
    for (int right = 0; right < (int)s.size(); right++) {
        cnt[s[right]]++;
        while ((int)cnt.size() > k) {
            if (--cnt[s[left]] == 0) cnt.erase(s[left]);
            left++;
        }
        best = max(best, right - left + 1);
    }
    return best;
}

int main() {
    cout << longestKDistinct("abcba", 2) << "\n"; // 3
    return 0;
}
