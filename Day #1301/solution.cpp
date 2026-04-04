// Day 1301: Fewest jumps from 0 to N where the ith jump moves exactly i (left/right).
// Find smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even (flipping a jump changes sum by 2*val).
#include <bits/stdc++.h>
using namespace std;

int minJumps(long long N) {
    N = llabs(N);
    long long k = 0, sum = 0;
    while (sum < N || (sum - N) % 2 != 0) {
        k++;
        sum += k;
    }
    return (int)k;
}

int main() {
    cout << minJumps(3) << endl; // 2  (1 + 2)
    cout << minJumps(2) << endl; // 3  (1 - 2 + 3)
    return 0;
}
