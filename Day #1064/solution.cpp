// Day 1064: Implement rand7() from rand5() via rejection sampling.
// (rand5()-1)*5 + rand5() -> uniform 1..25; reject >21; ((v-1)%7)+1. Expected O(1) calls, O(1) space.
#include <iostream>
#include <random>

std::mt19937 rng(42);

int rand5() {
    std::uniform_int_distribution<int> d(1, 5);
    return d(rng);
}

int rand7() {
    while (true) {
        int v = (rand5() - 1) * 5 + rand5(); // uniform 1..25
        if (v <= 21) return ((v - 1) % 7) + 1;
    }
}

int main() {
    int counts[8] = {0};
    for (int i = 0; i < 70000; ++i) counts[rand7()]++;
    for (int i = 1; i <= 7; ++i)
        std::cout << i << ": " << counts[i] << "\n";
    return 0;
}
