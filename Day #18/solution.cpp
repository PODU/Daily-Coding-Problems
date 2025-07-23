// Approach: Monotonic deque of indices; front always holds the window max. O(n) time, O(k) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> maxSlidingWindow(const vector<int>& nums, int k) {
    deque<int> dq; // indices, decreasing values
    vector<int> res;
    for (int i = 0; i < (int)nums.size(); i++) {
        if (!dq.empty() && dq.front() <= i - k) dq.pop_front();
        while (!dq.empty() && nums[dq.back()] <= nums[i]) dq.pop_back();
        dq.push_back(i);
        if (i >= k - 1) res.push_back(nums[dq.front()]);
    }
    return res;
}

int main() {
    vector<int> nums = {10, 5, 2, 7, 8, 7};
    int k = 3;
    vector<int> res = maxSlidingWindow(nums, k);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
