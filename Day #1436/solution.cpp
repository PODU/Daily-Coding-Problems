// Day 1436: Length of longest subarray with all distinct elements.
// Approach: Sliding window with last-seen index map; shrink left on repeat.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int longestDistinct(const vector<int>& arr) {
    unordered_map<int,int> last;
    int best = 0, left = 0;
    for (int right = 0; right < (int)arr.size(); ++right) {
        auto it = last.find(arr[right]);
        if (it != last.end() && it->second >= left)
            left = it->second + 1;
        last[arr[right]] = right;
        best = max(best, right - left + 1);
    }
    return best;
}

int main() {
    vector<int> arr = {5, 1, 3, 5, 2, 3, 4, 1};
    cout << longestDistinct(arr) << endl; // 5
    return 0;
}
