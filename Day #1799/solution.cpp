// Longest contiguous subarray with at most 2 distinct values via sliding window + hashmap. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int longestTwoDistinct(const vector<int>& a) {
    unordered_map<int,int> cnt;
    int left = 0, best = 0;
    for (int right = 0; right < (int)a.size(); right++) {
        cnt[a[right]]++;
        while ((int)cnt.size() > 2) {
            if (--cnt[a[left]] == 0) cnt.erase(a[left]);
            left++;
        }
        best = max(best, right - left + 1);
    }
    return best;
}

int main() {
    vector<int> a = {2, 1, 2, 3, 3, 1, 3, 5};
    cout << longestTwoDistinct(a) << endl; // 4
    return 0;
}
