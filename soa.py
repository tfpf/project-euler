#! /usr/bin/env python3

import itertools
import math


def algorithm_31(delta):
    for (x, y, k) in algorithm_41(delta):
        sieve[60 * k + delta] ^= True

def algorithm_41(delta):
    for (f, g) in itertools.product(range(1, 16), range(1, 31)):
        if (4 * f ** 2 + g ** 2) % 60 != delta:
            continue
        print('Found fg:', delta, f, g)
        (x, y0, k0) = (f, g, (4 * f ** 2 + g ** 2 - delta) // 60)
        while k0 < B:
            (k0, x) = (k0 + 2 * x + 15, x + 15)
        print(x, y0, k0)
        while True:
            (x, k0) = (x - 15, k0 - 2 * x - 15)
            if x <= 0:
                break
            while k0 < 0:
                (k0, y0) = (k0 + y0 + 15, y0 + 30)
            (k, y) = (k0, y0)
            while k < B:
                yield (x, y, k)
                (k, y) = (k + y + 15, y + 30)


N = 1199
B = math.ceil((N + 1) / 60)
sieve = [False] * (N + 1)
print(f'{N=} {B=}')
for delta in range(60):
    if delta in {1, 13, 17, 29, 37, 41, 49, 53}:
        algorithm_31(delta)

for item in itertools.islice(enumerate(sieve), 10000000000):
    print(item)


# for (f, g) in itertools.product(range(1, 16), range(1, 31)):
#     if (4 * f ** 2 + g ** 2) % 60 == 1:
#         print(f, g, (4 * f ** 2 + g ** 2) // 60)
