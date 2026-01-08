// Two numbers summing to k via one-pass hash set. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

bool twoSum(const vector<int>& nums, int k){
    unordered_set<int> seen;
    for(int x : nums){
        if(seen.count(k - x)) return true;
        seen.insert(x);
    }
    return false;
}

int main(){
    vector<int> nums = {10, 15, 3, 7};
    cout << (twoSum(nums, 17) ? "true" : "false") << "\n";
    return 0;
}
