// Sliding window with count map: longest subarray with at most 2 distinct values. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int longestAtMost2Distinct(vector<int>& nums) {
    unordered_map<int,int> cnt;
    int left = 0, best = 0;
    for (int right = 0; right < (int)nums.size(); right++) {
        cnt[nums[right]]++;
        while ((int)cnt.size() > 2) {
            cnt[nums[left]]--;
            if (cnt[nums[left]] == 0) cnt.erase(nums[left]);
            left++;
        }
        best = max(best, right - left + 1);
    }
    return best;
}

int main() {
    vector<int> nums = {2,1,2,3,3,1,3,5};
    cout << longestAtMost2Distinct(nums) << "\n";
    return 0;
}
