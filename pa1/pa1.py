import numpy as np
import matplotlib.pyplot as plt


def matmul():
    # Setting seed, so that data is same for any run
    np.random.seed(42)
    x = 1000000
    y = 1000

    bc = np.random.uniform(0.000001, 1, (y, x)) @ np.random.uniform(0.000001, 1, (x, 1))
    a = np.random.uniform(0.000001, 1, (x, y))
    d = a @ bc

    print(f"mean: {np.mean(d)}")

if __name__ == "__main__":
    matmul()
