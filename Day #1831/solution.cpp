// Smallest positive int not a subset sum of a sorted array. Greedy O(N).
// Keep reachable range [1..res-1]; if a[i] <= res extend, else res is the answer.
#include <bits/stdc++.h>
using namespace std;

long long smallestMissing(vector<long long>& a){
    long long res = 1;
    for(long long x : a){
        if(x > res) break;
        res += x;
    }
    return res;
}

int main(){
    vector<long long> a = {1, 2, 3, 10};
    cout << smallestMissing(a) << "\n"; // 7
    return 0;
}
