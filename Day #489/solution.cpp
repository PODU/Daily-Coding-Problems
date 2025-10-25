// Day 489: Longest subarray with all distinct elements.
// Sliding window + last-seen map; move left past previous occurrence. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int longestDistinctSubarray(const vector<int>& a) {
    unordered_map<int, int> last; // value -> last index seen
    int left = 0, best = 0;
    for (int right = 0; right < (int)a.size(); ++right) {
        auto it = last.find(a[right]);
        if (it != last.end() && it->second >= left) left = it->second + 1;
        last[a[right]] = right;
        best = max(best, right - left + 1);
    }
    return best;
}

int main() {
    vector<int> a = {5, 1, 3, 5, 2, 3, 4, 1};
    cout << longestDistinctSubarray(a) << "\n"; // 5
    return 0;
}
