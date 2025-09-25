# Day 328: Simplified Elo: expected = 1/(1+10^((Rb-Ra)/400)); delta = round(K*(s-expected)); zero-sum transfer.
# Time O(1), Space O(1).

def main():
    Ra, Rb, K = 1200, 2000, 32
    expected_a = 1.0 / (1.0 + 10.0 ** ((Rb - Ra) / 400.0))
    delta = round(K * (1 - expected_a))  # A wins, s=1
    new_a = Ra + delta
    new_b = Rb - delta
    print("Winner ({}) -> {}, Loser ({}) -> {}".format(Ra, new_a, Rb, new_b))


if __name__ == "__main__":
    main()
