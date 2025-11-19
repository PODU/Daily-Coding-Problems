// Longest consecutive run of 1s in binary repr of n.
// Bit trick: n &= (n<<1) collapses runs; iterations = longest run. O(bits) time, O(1) space.
#include <iostream>

int longestRun(unsigned int n) {
    int count = 0;
    while (n) { n &= (n << 1); ++count; }
    return count;
}

int main() {
    std::cout << longestRun(156) << "\n"; // 10011100 -> 3
    return 0;
}
