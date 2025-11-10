// Min jumps: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even. Time O(sqrt(N)), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minJumps(long long N) {
    long long n = llabs(N);
    long long k = 0, s = 0;
    while (s < n || (s - n) % 2 != 0) {
        k++;
        s += k;
    }
    return (int)k;
}

int main() {
    cout << "Minimum jumps to reach 10: " << minJumps(10) << "\n";
    return 0;
}
