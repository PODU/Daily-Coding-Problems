// Monte-Carlo simulation of two stop conditions plus exact theory.
// E[rolls until 5->6] = 36 (distinct faces); E[rolls until 5->5] = 42 (same face).
// Time O(trials * rolls), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long play(int second, mt19937& gen, uniform_int_distribution<int>& d) {
    long long rolls = 0; int prev = 0;
    while (true) {
        int cur = d(gen);
        rolls++;
        if (prev == 5 && cur == second) return rolls;
        prev = cur;
    }
}

int main() {
    mt19937 gen(42);
    uniform_int_distribution<int> d(1, 6);
    long long trials = 200000, s56 = 0, s55 = 0;
    for (long long i = 0; i < trials; i++) s56 += play(6, gen, d);
    for (long long i = 0; i < trials; i++) s55 += play(5, gen, d);
    printf("Game 1 (five then six): simulated avg = %.2f, theoretical = 36\n",
           (double)s56 / trials);
    printf("Game 2 (five then five): simulated avg = %.2f, theoretical = 42\n",
           (double)s55 / trials);
    printf("Alice should play Game 1 (five-then-six): fewer expected rolls, less pay.\n");
    return 0;
}
