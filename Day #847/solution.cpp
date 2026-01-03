// Day 847: jump game - can we reach the last index? Greedy furthest-reach. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool canReach(const vector<int>& a){
    int reach = 0, n = a.size();
    for(int i = 0; i < n; ++i){
        if(i > reach) return false;
        reach = max(reach, i + a[i]);
    }
    return true;
}

int main(){
    cout << (canReach({2,0,1,0}) ? "True" : "False") << "\n"; // True
    cout << (canReach({1,1,0,1}) ? "True" : "False") << "\n"; // False
    return 0;
}
