// Day 674: Longest contiguous subarray with at most 2 distinct values. Sliding window.
// Time O(n), Space O(1) (at most 3 keys in the map).
#include <bits/stdc++.h>
using namespace std;

int longestTwoTypes(const vector<int>& a) {
    unordered_map<int,int> cnt;
    int best = 0, l = 0;
    for (int r = 0; r < (int)a.size(); r++) {
        cnt[a[r]]++;
        while ((int)cnt.size() > 2) {
            if (--cnt[a[l]] == 0) cnt.erase(a[l]);
            l++;
        }
        best = max(best, r - l + 1);
    }
    return best;
}

int main() {
    vector<int> a = {2, 1, 2, 3, 3, 1, 3, 5};
    cout << longestTwoTypes(a) << "\n"; // 4
    return 0;
}
