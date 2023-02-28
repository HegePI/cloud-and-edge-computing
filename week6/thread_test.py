import threading
import numpy as np
import time

def thread_function(numArray, N):
    np.sum(numArray[:N:])

def main():
    maxN = 1000
    numArray = np.random.uniform(1,1000,maxN) 
    time_samples = np.zeros(1000)
    for i in range(1000): 
        start_time = time.time()
        N  = np.random.randint(1,1000)
        thread_handle = threading.Thread(target=thread_function, args=(numArray, N))
        thread_handle.start()
        thread_handle.join()
        end_time = time.time()
        time_samples[i] = end_time - start_time

    print(f"Min time: {np.min(time_samples)}s")
    print(f"Max time: {np.max(time_samples)}s")
    print(f"Avg time: {np.average(time_samples)}s")
    print(f"STD: {np.std(time_samples)}s")  


if __name__ == "__main__":
    main()


