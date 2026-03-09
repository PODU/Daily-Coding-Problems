// 3-sum existence: sort, fix each i, two-pointer scan remaining pair. Time O(N^2), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool threeSum(vector<int> a, long long k) {
    sort(a.begin(), a.end());
    int n = a.size();
    for (int i = 0; i < n - 2; i++) {
        int lo = i + 1, hi = n - 1;
        while (lo < hi) {
            long long s = (long long)a[i] + a[lo] + a[hi];
            if (s == k) return true;
            else if (s < k) lo++;
            else hi--;
        }
    }
    return false;
}

int main() {
    vector<int> arr = {20, 303, 3, 4, 25};
    cout << (threeSum(arr, 49) ? "true" : "false") << endl;
    return 0;
}
