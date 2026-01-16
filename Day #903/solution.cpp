// Sort, then fix arr[i] and two-pointer scan for the remaining pair summing to k.
// Time O(n^2), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

bool threeSum(vector<int> arr, int k) {
    sort(arr.begin(), arr.end());
    int n = arr.size();
    for (int i = 0; i < n - 2; i++) {
        int lo = i + 1, hi = n - 1;
        while (lo < hi) {
            int s = arr[i] + arr[lo] + arr[hi];
            if (s == k) return true;
            if (s < k) lo++; else hi--;
        }
    }
    return false;
}

int main() {
    vector<int> arr = {20, 303, 3, 4, 25};
    cout << (threeSum(arr, 49) ? "true" : "false") << endl;
    return 0;
}
