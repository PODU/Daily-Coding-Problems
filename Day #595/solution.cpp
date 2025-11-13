// Kaprekar's routine: repeatedly subtract ascending-digit number from
// descending-digit number (4-digit, zero-padded) until reaching 6174.
// Bounded to <=7 steps. Time O(1), Space O(1).
#include <iostream>
#include <algorithm>
#include <string>
#include <cstdio>
using namespace std;

int main() {
    int n = 1234;
    int steps = 0;
    while (n != 6174) {
        char buf[5];
        snprintf(buf, sizeof(buf), "%04d", n);
        string s(buf);
        string asc = s, desc = s;
        sort(asc.begin(), asc.end());
        sort(desc.begin(), desc.end(), greater<char>());
        int hi = stoi(desc);
        int lo = stoi(asc);
        n = hi - lo;
        steps++;
        printf("%04d - %04d = %04d\n", hi, lo, n);
    }
    printf("Steps: %d\n", steps);
    return 0;
}
