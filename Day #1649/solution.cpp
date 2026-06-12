// Bijective base-26: while n>0, n--, prepend 'A'+(n%26), n/=26. O(log n) time, O(log n) space.
#include <iostream>
#include <string>
#include <algorithm>

std::string columnTitle(long long n) {
    std::string s;
    while (n > 0) {
        --n;
        s += char('A' + (n % 26));
        n /= 26;
    }
    std::reverse(s.begin(), s.end());
    return s;
}

int main() {
    std::cout << columnTitle(1) << "\n";
    std::cout << columnTitle(27) << "\n";
    return 0;
}
