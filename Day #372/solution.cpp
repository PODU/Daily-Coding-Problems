// Day 372: Count digits of a natural number without loops.
// Tail recursion: 1 digit for n<10, else 1 + digits(n/10). Time O(d), Space O(d).
#include <bits/stdc++.h>
using namespace std;

int numDigits(long n) {
    return n < 10 ? 1 : 1 + numDigits(n / 10);
}

int main() {
    cout << numDigits(0) << "\n";       // 1
    cout << numDigits(7) << "\n";       // 1
    cout << numDigits(42) << "\n";      // 2
    cout << numDigits(12345) << "\n";   // 5
    return 0;
}
