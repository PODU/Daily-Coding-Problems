// Sliding window median via two balanced multisets (lower/upper halves).
// Time: O(n log k), Space: O(k).
#include <bits/stdc++.h>
using namespace std;

static string fmtNum(double d){
    if (fabs(d - llround(d)) < 1e-9) return to_string((long long)llround(d));
    ostringstream os; os << d; return os.str();
}

int main(){
    vector<int> arr = {-1,5,13,8,2,3,3,1};
    int k = 3;
    multiset<int> lo, hi;          // lo = lower half (holds extra when k odd)
    auto rebalance = [&](){
        while((int)lo.size() > (int)hi.size()+1){ hi.insert(*lo.rbegin()); lo.erase(prev(lo.end())); }
        while(lo.size() < hi.size()){ lo.insert(*hi.begin()); hi.erase(hi.begin()); }
    };
    auto insert = [&](int x){
        if(lo.empty() || x <= *lo.rbegin()) lo.insert(x); else hi.insert(x);
        rebalance();
    };
    auto erase = [&](int x){
        if(x <= *lo.rbegin()) lo.erase(lo.find(x)); else hi.erase(hi.find(x));
        rebalance();
    };
    auto median = [&]() -> double {
        if(k % 2 == 1) return *lo.rbegin();
        return (*lo.rbegin() + (double)*hi.begin()) / 2.0;
    };

    for(int i=0;i<k;i++) insert(arr[i]);
    int n = arr.size();
    for(int i=k;i<=n;i++){
        string win = "[";
        for(int j=i-k;j<i;j++){ if(j>i-k) win += ", "; win += to_string(arr[j]); }
        win += "]";
        cout << fmtNum(median()) << " <- median of " << win << "\n";
        if(i < n){ insert(arr[i]); erase(arr[i-k]); }
    }
    return 0;
}
