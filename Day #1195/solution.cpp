// Smallest window containing all distinct chars of the string. Sliding window:
// expand right, shrink left while all distinct kinds present. O(N) time, O(K) space.
#include <bits/stdc++.h>
using namespace std;

int smallestWindow(const string& s) {
    int distinct = set<char>(s.begin(), s.end()).size();
    unordered_map<char, int> cnt;
    int have = 0, left = 0, best = INT_MAX;
    for (int right = 0; right < (int)s.size(); ++right) {
        if (++cnt[s[right]] == 1) have++;
        while (have == distinct) {
            best = min(best, right - left + 1);
            if (--cnt[s[left]] == 0) have--;
            left++;
        }
    }
    return best == INT_MAX ? 0 : best;
}

int main() {
    cout << smallestWindow("jiujitsu") << '\n';
    return 0;
}
