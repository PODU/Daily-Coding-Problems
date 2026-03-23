// Two-sum existence: one-pass hash set, check (k-num) seen before insert.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;
bool hasPair(const vector<int>& a,int k){
    unordered_set<int> seen;
    for(int x:a){ if(seen.count(k-x))return true; seen.insert(x); }
    return false;
}
int main(){
    vector<int> v={10,15,3,7};
    cout<<(hasPair(v,17)?"true":"false")<<"\n";
    cout<<(hasPair(v,50)?"true":"false")<<"\n";
}
