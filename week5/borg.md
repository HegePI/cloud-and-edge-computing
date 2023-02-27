# Borg - Heikki Pulli

## 1

The authors aim is to introduce reader to the Borg system. Borg is cluster manager system developed and used by google. Borg manages thousands of workloads of applications and manages multiples machines to allocate needed resources for these tasks.

## 2

In general diurnal pattern/cycle referes to things/events which occurance is based on the rotation of the earth. In the context of the article diurnal pattern would refer to cronjob, that is run everyday at the same time. Other example would be the usage of certain cloud provided software that is used by big audience, ie. google drive. This is used by many companies and other parties, and most of the usage occurs during the working hours.

## 3

The reason using static linking results in bigger executable size is that it combines all the dependency programs also to the executable. Good thing in compiling this way is that you can drop the executable in any runtime and run it there and it will work, because it has all the dependencies with it. The problem arises when there are multiple dependencies for the program. Then executable sixe might grow too big. Smarter compilation techniques could reduce the executable size.

![static-linking](/home/heikki/koulu/cloud-and-edge-computing/week5/a9-static-linking.png)

## 4

Borg cell is logical abstraction on which google developers run their tasks. Borg cell contains Borgmaster and its scheduler and borglets. Borglet is a machine that is provisioned to the cell and that runs a process of local borg agent. These borglets in the cell communicate with Borgmaster.

Each cell contains it's own Borgmaster process and it's separate scheduler attached to it. Borgmaster manages all the client workloads sent to it, manages all the state machines for all the objects in the system and it communicates with Borglets.

Scheduler ensures that all the jobs sent to borgmaster are treated equally. When job is submitted, Borgmaster appends it to the pending queue and scheduler asynchronously scans through the queue and assigns each job accordingly to a machine if there are sufficient resources that meet the jobs constraint's.

## 5

Figure 5 represents how many new cells (new machines) would be needed in relations of current amount old cells (old machines) when overhead caused by segregating non-prod and prod jobs increases to certain extent. Steps in the CDF is caused by the fact, that when adding new cells it can handle certain amount of new overhead without the need of other new cells. Error bars in the increases of the CDF represent the amount of error between different test runs. If error bar is small, then at that point need for more new cells almost always occured, but if error bar is big there has been more variance, when new cells are actually required.

## 6

Milli-core is a metric measurement that is used to measure CPU usage. If CPU has 8 cores, then it has 8000 milli-cores. When slicing CPU to millicores measuring CPU usage becomes much more precise.

## 7

Section 5 describes different ways of optimization of current machinery for the purpose of Borg. The most interesting part in the section is about fine-grained resource requests and resource reclamation. The idea behind fine-grained resource requesting is to make users to specify more accurately their needs for computing environment. Idea behind smaller units in resources requests is that users can still request the same CPU and memory, but resourse reclamation process can measure the actual usage more precisely and free un-used resources to other processes. This increases the usability of current machinery and therefore decreaeses the need to buy more new machines to accomodate for the need that actually is not present.

## 8

## 9
