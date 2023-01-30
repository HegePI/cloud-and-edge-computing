import numpy as np
import matplotlib.pyplot as plt


def cdf():
    a = np.random.uniform(0, 1, (1000000, 1000))
    plt.plot(np.arange(1000000), np.cumsum(a.reshape(-1))[0:-1:1000])
    plt.savefig("cdf.png")


if __name__ == "__main__":
    cdf()
