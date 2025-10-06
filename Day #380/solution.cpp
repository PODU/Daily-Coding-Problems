// Integer division without '/': repeated doubling/subtraction.
// Time: O(log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

pair<long long,long long> divide(long long a, long long b){
    bool neg = (a < 0) != (b < 0);
    long long x = llabs(a), y = llabs(b);
    long long q = 0;
    while(x >= y){
        long long temp = y, mult = 1;
        while(x >= (temp << 1)){ temp <<= 1; mult <<= 1; }
        x -= temp; q += mult;
    }
    return { neg ? -q : q, x }; // remainder keeps dividend sign convention (here non-negative)
}

int main(){
    auto r = divide(10, 3);
    cout << "(" << r.first << ", " << r.second << ")\n";
    return 0;
}
