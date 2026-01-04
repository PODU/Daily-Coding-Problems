// Day 849: Collatz - verify each n reaches 1; bonus: longest chain for n <= 1,000,000.
// Memoized chain lengths in an array. ~O(limit) amortized.
#include <bits/stdc++.h>
using namespace std;

int steps(long long n){            // number of steps to reach 1
    int c = 0;
    while(n != 1){ n = (n & 1) ? 3*n + 1 : n/2; c++; }
    return c;
}

int main(){
    const int LIMIT = 1000000;
    cout << "27 reaches 1 in " << steps(27) << " steps\n"; // 111

    vector<int> cache(LIMIT + 1, 0);
    cache[1] = 1;                   // chain length (count of terms)
    int bestN = 1, bestLen = 1;
    for(int i = 2; i <= LIMIT; ++i){
        long long n = i; int len = 0;
        while(n >= i || cache[n] == 0){       // walk until a cached value
            n = (n & 1) ? 3*n + 1 : n/2;
            len++;
            if(n == 1) break;
        }
        int total = len + (n <= LIMIT ? cache[n] : 1);
        if(i <= LIMIT) cache[i] = total;
        if(total > bestLen){ bestLen = total; bestN = i; }
    }
    cout << "Longest chain for n <= 1000000: n = " << bestN
         << " (length " << bestLen << ")\n"; // n = 837799
    return 0;
}
