# Ramcloud

## Q1

The problem in the article is the unsufficient use of memory in computer systems. For example softwares, that use memory as storage, ie. redis, could benefit drastically from more efficient memory utilization.

## Q2

In both workloads W2 and W8 memory is first allocated with n sized objects. Then objects are randomly deleted so that memeory usage doesn't go under 10gb. Then some percentage of those are deleted randomly and finally memory is re-filled with different sized objects.

In W2 the storage is filled with 100 bytes sized objects, then none of them are deleted and finally all of them are replaced with 130 byte sized objects.

In W8 the memory is allocated with uniformly with 50 to 150 byte sized objects, then 90% of the objects are deleted and finally memory is filled again with uniformly distributed objects with size 5000 to 15000 bytes.

The performance difference is shown in figure 1, and besides the "hoard 3.9", which doesn't show W8, W8 and W2 perform quite similarly. Only in "glibc 2.12 malloc" W2 doesn't take as much memory as W8.

## Q3

There is a big difference between different workloads with different allocators and even with same allocator with different workloads. It is quite dependent on the use target, that should define the proper memory allocator.

## Q4

Without knowing the pointers, garbage collector (gc) coudn't free the memory from heap in the correct place.

## Q5

Yes, because the application would need to wait for the gc to finish the work. With many objects in the memory the pause time could take a lot of time to update their references.

## Q6

A data blob is a fixed size memory area that contains data. RAMCloud is optimized for small blobs, but it can support up to 1mb blobs as well.Uninterpreted blobs mean, that RAMCloud doesn't check what kind of data is stored. Not checking the data and possibly optimizing for it would decrease the performance of the RAMCloud. Also it adheres to End-to-End (E2E) principle.

## Q7

Cleaning is important so that deleted blobs don't take space from new data.

When cleaning disks and memory together, RAMCloud couldn't achieve both high memory utilization and write throughput. If memory usage was high, the write throughput would be limited by cleaners usage of disks bandwith.

## Q8

Because in order to cleaner to work there must be free memory for use. Thats because before freeing memory it first copies data. So if there is no memory, where copied data can be allocated, then the system is in deadlock.

## Q9

1. The workload was descibed in section 8 before figures 6 and 7. Each master was given 16gb of memory. Workload for testing was to write data to memory, until wanted memory utilization was reached. After that it overwrote the objects until the overhead for cleaning converged to stable value. Each workload was run three times and results were averaged.

2. Authors use both write speed(Mb/s) and the object count(x1000) written to memory.

3. Figure shows that as memory utilization grows with the size of the objects written to memory two-level cleaning strategy outperforms one-level strategy. Even sequential strategy performs better than One-level strategy with larger object sizes. In any case two-level strategy performs better than one-level.

## Q10

Figure 7 shows, that as objects size grows two-level cleaning strategy takes less overhead than one-level strategy. Overhead is measured as ratio between cleaner bandwidth and regular log write bandwidth. For one-level strategy the overhead seems to be almost constant.

## Q11

Looking at the figure 9 it would be reasonable to not use cleaner, because the percentage of sudden long time consuming writes is smaller than with cleaner. This would be optimal for application that doesn't need to write and read much data into to memory.

Cleaner option is of course crucial for applications which perform lots of writes and stores lots of data into the disk. Without cleaner memory would be filled with garbage data that is not needed by the application, ie. application that processes stream data and stores the results.

## Q12

## Q13

## Q14
