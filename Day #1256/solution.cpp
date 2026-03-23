// Max contiguous subarray sum, empty allowed: Kadane, clamp running sum at 0.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;
long long maxSub(const vector<int>& a){
    long long cur=0,best=0;
    for(int x:a){ cur+=x; if(cur<0)cur=0; best=max(best,cur); }
    return best;
}
int main(){
    cout<<maxSub({34,-50,42,14,-5,86})<<"\n";
    cout<<maxSub({-5,-1,-8,-9})<<"\n";
}
