# Day 1552: Max profit single buy-then-sell: track running min price and best (price - min). Time O(n), Space O(1).

def max_profit(prices):
    if not prices:
        return 0
    min_price = prices[0]
    best = 0
    for p in prices:
        if p < min_price:
            min_price = p
        elif p - min_price > best:
            best = p - min_price
    return best


def main():
    prices = [9, 11, 8, 5, 7, 10]
    print(max_profit(prices))


if __name__ == "__main__":
    main()
