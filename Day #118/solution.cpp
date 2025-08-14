// Day 118: Two-pointer merge from both ends into result back-to-front. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;
vector<int> sortedSquares(const vector<int>& a){
    int n = a.size(); vector<int> res(n);
    int lo = 0, hi = n - 1;
    for(int k = n - 1; k >= 0; --k){
        int sl = a[lo]*a[lo], sh = a[hi]*a[hi];
        if(sl > sh){ res[k] = sl; lo++; } else { res[k] = sh; hi--; }
    }
    return res;
}
int main(){
    auto r = sortedSquares({-9,-2,0,2,3});
    cout << "[";
    for(size_t i=0;i<r.size();++i) cout << r[i] << (i+1<r.size()?", ":"");
    cout << "]\n"; // [0, 4, 4, 9, 81]
    return 0;
}
