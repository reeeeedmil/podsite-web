import rsnet

net = rsnet.Net(rsnet.Address(192, 168, 0, 0), rsnet.Address(255, 255, 255, 0))
podsite = rsnet.scaffold_prefixes(net, [30, 25, 20, 24, 30])
print(podsite)
