// n-th perfect number (digits sum to 10): count up from 1 checking digit sums. Time O(answer * digits), Space O(1).
#include <iostream>
using namespace std;

int digitSum(long long x) {
    int s = 0;
    while (x) { s += x % 10; x /= 10; }
    return s;
}

long long nthPerfect(int n) {
    int count = 0;
    long long num = 0;
    while (count < n) {
        ++num;
        if (digitSum(num) == 10) ++count;
    }
    return num;
}

int main() {
    cout << nthPerfect(1) << endl;
    cout << nthPerfect(2) << endl;
    return 0;
}
