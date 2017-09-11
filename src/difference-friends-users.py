import sys

friends = set()
users = set()

degree = dict()

# f1 = open("./results/links-degrees.dat")
# f2 = open("./results/users-degrees")
f1 = open("./data/links.txt")
f2 = open("./data/ratings.txt")

for line in f1.readlines():
	contents = line.split()
	friends.add(int(contents[0]))
	friends.add(int(contents[1]))

	# u = int(contents[0])
	# d = int(contents[1])

	# degree[u] = dict()
	# degree[u]["sn"] = d
	# degree[u]["ratings"] = None

f1.close()

for line in f2.readlines():
	contents = line.split()
	u = int(contents[0])
	users.add(u)
	# d = int(contents[1])

	# try:
	# 	degree[u]["ratings"] = d
	# except KeyError:
	# 	degree[u] = dict()
	# 	degree[u]["sn"] = None
	# 	degree[u]["ratings"] = d

# for node in degree:
# 	if degree[node]["ratings"] is not None and degree[node]["sn"] is not None:
# 		print(str(node) + " " + str(degree[node]["sn"]) + " " + str(degree[node]["ratings"]))


print("# of users in social network : " + str(len(friends))) # 786936
print("# of users in ratings : " + str(len(users))) # 147612
print("# of users in friends and ratings : " + str(len(friends.intersection(users)))) # 147335