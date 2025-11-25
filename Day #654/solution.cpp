// Smallest window containing every distinct char: sliding window with two pointers.
// Expand right until all distinct chars present, shrink left to minimize. Time O(n), space O(k).
#include <bits/stdc++.h>
using namespace std;

int smallestWindow(const string& s) {
    unordered_set<char> distinct(s.begin(), s.end());
    int need = (int)distinct.size();
    unordered_map<char, int> cnt;
    int have = 0, best = INT_MAX, left = 0;
    for (int right = 0; right < (int)s.size(); ++right) {
        if (++cnt[s[right]] == 1) have++;
        while (have == need) {
            best = min(best, right - left + 1);
            if (--cnt[s[left]] == 0) have--;
            left++;
        }
    }
    return best == INT_MAX ? 0 : best;
}

int main() {
    cout << smallestWindow("jiujitsu") << "\n";
    return 0;
}
