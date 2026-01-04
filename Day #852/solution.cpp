// Day 852: maximum circular subarray sum (empty allowed -> 0).
// answer = max(maxKadane non-wrap clamped at 0, total - minKadane). O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

long long maxCircular(const vector<long long>& a){
    long long total = 0, maxK = LLONG_MIN, minK = LLONG_MAX;
    long long curMax = 0, curMin = 0;
    for(long long x : a){
        total += x;
        curMax = max(x, curMax + x); maxK = max(maxK, curMax);
        curMin = min(x, curMin + x); minK = min(minK, curMin);
    }
    long long nonWrap = max(0LL, maxK);            // empty subarray allowed
    long long wrap = total - minK;                 // remove the min subarray
    return max(nonWrap, wrap);
}

int main(){
    cout << maxCircular({8,-1,3,4}) << "\n";  // 15
    cout << maxCircular({-4,5,1,0}) << "\n";  // 6
    return 0;
}
