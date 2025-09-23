// Min jumps on number line: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even.
// Time: O(sqrt N), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int minJumps(long long N) {
    long long target = llabs(N);
    long long k = 0, S = 0;
    while (S < target || (S - target) % 2 != 0) {
        k++;
        S += k;
    }
    return (int)k;
}

int main() {
    long long N = 10;
    cout << "Minimum jumps to reach " << N << ": " << minJumps(N) << "\n";
    return 0;
}
