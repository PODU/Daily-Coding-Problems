// Day 1772: Min jumps to reach N. Grow k until triangular sum >= |N| and (sum-|N|) even.
// Flipping any jump j changes parity of (sum-N) by 2j, so even surplus is reachable. O(sqrt(N)).
#include <bits/stdc++.h>
using namespace std;

int minJumps(long long n) {
    long long s = llabs(n);
    long long sum = 0;
    int k = 0;
    while (sum < s || (sum - s) % 2 != 0) {
        k++;
        sum += k;
    }
    return k;
}

int main() {
    cout << minJumps(10) << '\n';
    return 0;
}
