import sys
import networkx as nx

g = nx.Graph()

for line in sys.stdin:
	contents = line.split()
	u = int(contents[0])
	v = int(contents[1])
	g.add_edge(u,v)

for c in nx.connected_components(g):
	print(len(c))