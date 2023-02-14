# VXLans

## 1

Todays datacenters and cloud providers are testing the limits of the STP protocol. Datacenter networks use VLAN to provide broadcast isolation, where every stp port provides an isolated VLAN. But STP can only provide 4094 of these and that is too few for modern datacenters. The need for more isolated VLAN has grown due to increase in VM technologies.

## 2

An overlay network is a network that is built on top of another network. <https://en.wikipedia.org/wiki/Overlay_network>

https and Tor are a example of overlay networks.

## 3

yes, because if docker container has it's own MAC-address then it is shown in MAC-address table.

## 4

Broadcast isolation means segmenting network into clear segments, where only devices inside segment can communicate with each other. This has added benefit of reducing the amount of needed ip-addreses inside a segment.

## 5

Figure 1 describes different message headers. The most important part of figure 1 is VXLAN header. Other Headers are normal headers of their kind.

In the VXLAN header flag is assigned 8-bits, and it must always be set to 1 for valid VXLAN header. Rest of the flag bits are reserved bits. The VXLAN Network Identifier (VNI) states the VXLAN overlay network, where message is relayed. VNI uses 24 bits. Rest of the bits (24 and 8 bits) are reserved bits, and must be set to zero and ignored on receipt.

## 6

In figure 2 header uses ipv6 header instead of ipv4 header. Otherwise headers are identical.

## 7

Scope of the mac addresses refers to the ranges of group and locally administered addresses. Addresses can be administered gouped (multicast) or individually (unicast) and both of these can be either universally or locally adnministered. The way MAC-addresses are admisnistered can be infered from second bit of MAC-address.
