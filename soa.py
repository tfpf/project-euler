#! /usr/bin/env python3

import itertools
import math


def algorithm_31(delta):
    for (x, y, k) in algorithm_41_(delta):
        sieve[60 * k + delta] ^= True

def algorithm_41_(delta):
    for (f, g) in itertools.product(range(1, 16), range(1, 31)):
        if delta == (4 * f ** 2 + g ** 2) % 60:
            print(f'\033[1;31mFound {delta=} {f=} {g=}\033[m')
            for item in algorithm_41(delta, f, g):
                yield item

def algorithm_41(delta, f, g):
    (x, y0, k0) = (f, g, (4 * f ** 2 + g ** 2 - delta) // 60)
    print(f'  Starting with {x=} {y0=} {k0=}')
    while k0 < B:
        (k0, x) = (k0 + 2 * x + 15, x + 15)
    print(f'  Changed to {x=} {y0=} {k0=}')
    while True:
        (k0, x) = (k0 - 2 * x + 15, x - 15)
        print(f'    Moved left {x=} {y0=} {k0=}')
        if x <= 0:
            break
        while k0 < 0:
            (k0, y0) = (k0 + y0 + 15, y0 + 30)
            print(f'      Moved up {x=} {y0=} {k0=}')
        print(f'    Reached {x=} {y0=} {k0=}')
        (k, y) = (k0, y0)
        while k < B:
            print(f'      Got {x=} {y=} {k=}')
            yield (x, y, k)
            (k, y) = (k + y + 15, y + 30)


N = 130
N -= N % 60 + 1
B = math.ceil((N + 1) / 60)
sieve = [False] * (N + 1)
print(f'{N=} {B=}')
for delta in range(60):
    if delta in {1, 13, 17, 29, 37, 41, 49, 53}:
        algorithm_31(delta)
for (num, prime) in itertools.islice(enumerate(sieve), 10000000000):
    if prime:
        print(num)


# for (f, g) in itertools.product(range(1, 16), range(1, 31)):
#     if (4 * f ** 2 + g ** 2) % 60 == 1:
#         print(f, g, (4 * f ** 2 + g ** 2) // 60)
