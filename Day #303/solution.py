# Day 303: Angle between clock hands. O(1).
# Bonus: hands overlap (angle 0) every 12/11 hours (~65.45 min apart).
def clock_angle(h, m):
    hr = (h % 12) * 30.0 + m * 0.5
    mn = m * 6.0
    diff = abs(hr - mn)
    diff = min(diff, 360.0 - diff)
    return round(diff)


if __name__ == "__main__":
    print(clock_angle(3, 30))  # 75
