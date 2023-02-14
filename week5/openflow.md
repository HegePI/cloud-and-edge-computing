# Openflow - Heikki Pulli

## 1

The goal of the paper is to introsuce OpenFlow switch. OpenFlow switch is programmable network switch that provides needed features for network research. In the paper OpenFlow is aimed at the researchers at campus, who try to do research using campus networks. Earlier campus network research has been dificult, because there hasn't been guarantees that new protocol/device wouldn't crash the network. OpenFlow tries to minimize this risk.

## 2

Network ossification refers to unability to modify and upgrade current networks protocols. This is due to the middleboxes of the big telecom companies, who have designed boxes to work the certain way and it is not possible for outsiders to change tha way it works. The networks is delivered "as is" and it cannot be modified for research purposes.

## 3

Pros of the ossification is that everybody uses the same protocol to communicate. Ossification quarantees that messages on the net are not dropped due to differing protocols.

Cons of ossification on the other hand is that everybody is stuck to using the de facto protocols even though they might not be suitable for the use case. Basically on the internet TCP and UDP are only practical protocols to use. But if person tries to develop a new protocol on the internet person usually has to wrap the message inside the TCP/UDP packet for it to get sent where wanted. This slows the research of new protocols.

## 4

a) Line-rate processing refers to the processing of the network messages done in networks at the speed of current network. If netwrok speed is 10Gib/s, then messages are processed also at that speed.

b) Messages of the network.

c) Regular machines (ie. PCs) cannot process messages at the needed speed. Machines built specifically for this job are needed for the speed.

d) Iftop is a tool used to monitor the bandwidth and internet usage of system, <https://en.wikipedia.org/wiki/Iftop>. To get the bandwidth of wanted interface, run ```iftop -i <INTERFACE_NAME>```.

## 5

In the context of the article flows are broadly defined and it could mean many things. For example all packets from certain IP adress is a flow and a TCP connection can be flow.

## 6

a) Aggretion of flows means tracking all flows of the same type, ie. TCP flows or IP address flows.

b) Aggregation of flows gives information of certain type of flow, not just singular flow. Point is to measure ie. how UDP protocol behaves in certain situations or issues.

## 7

a) No, because controller manages all the flowtable of switches it is connected to. If the controller is connected to all switches, then it manages all the paths.

b) OpenFlow allows multiple controllers to manage single flowtable. If only one controller controls switches flowtable and it crashes, then switch cannot be used anymore. But if there multiple controllers managing the flowtable, then the other controller takes over.

## 8
