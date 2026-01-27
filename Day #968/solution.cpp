// Day 968: Fewest jumps from 0 to N where i-th jump moves +/- i.
// Approach: smallest k with sum 1..k >= |N| and (sum-|N|) even. Time O(sqrt(N)), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minJumps(long long N) {
    N = llabs(N);
    long long k = 0, sum = 0;
    while (sum < N || (sum - N) % 2 != 0) { k++; sum += k; }
    return (int)k;
}

int main() {
    cout << minJumps(10) << endl; // 4  (1+2+3+4=10)
    return 0;
}
