# Ramcloud

## Q1

The problem in the article is the unsufdficient use of memory in computer systemms. Softwares, that use memory as storage, ie. redis, could benefit drastically from more efficient memory utilization.

## Q2

In both workloads W2 and W8 memory is first allocated with n sized objects, then some percentage of those are deleted and finally memory is re-allocatedwith different sized objects.

In W2 the storage is filled with 100 bytes sized objects, then none of them are deleted and finally all of them are replaced with 130 byte sized objects.

In W8 the memory is allocated with uniformly with 50 to 150 byte sized objects, then 90% of the objects are deleted and finally memory is filled again with uniformly distributed objects with size 5000 to 15000 bytes.

The performance difference is shown in figure 1, and besides the "hoard 3.9", which doesn't show W8, W8 and W2 perform quite similarly. Only in "glibc 2.12 malloc" W2 doesn't take as much memory as W8.

## Q3

There is a big difference between different workloads with different allocators and even with same allocator with different workloads. It is quite dependent on the use target, that should define the proper memory allocator.

## Q4

Without knowing the pointers, garbage collector (gc) coudn't free the memory from heap from correct place.

## Q5

Yes, because the application would need to wait for the gc to finish the work.

## Q6

A data blob is a fixed size memory area that contains data. RAMCloud is optimized for small blobs, but it can support up to 1mb blobs as well. Uninterpreted blobs mean, that RAMCloud doesn't check what kind of data is stored. Not checking the data and possibly optimizing for it would decrease the performance of the RAMCloud. Also it adheres to End-to-End (E2E) principle.

## Q7

## Q8

## Q9

## Q10

## Q11

## Q12

## Q13

## Q14
