// Egg drop: min trials t s.t. floors(t,eggs)=sum_{i=1..eggs} C(t,i) >= k.
// O(eggs * answer). For N=1,k=5 -> 5.
#include <bits/stdc++.h>
using namespace std;

long long floorsCovered(long long t, long long eggs, long long cap){
    long long total = 0, c = 1;
    for(long long i = 1; i <= eggs; i++){
        c = c * (t - i + 1) / i;   // C(t,i) incrementally
        total += c;
        if(total >= cap) return cap; // avoid overflow
    }
    return total;
}

long long minDrops(long long eggs, long long k){
    long long t = 0;
    while(floorsCovered(t, eggs, k) < k) t++;
    return t;
}

int main(){
    cout << minDrops(1, 5) << "\n"; // 5
    return 0;
}
