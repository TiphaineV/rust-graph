# Input : social network + ratings network
# Output : u v r_u r_v, where r_u and r_v is the number of ratings of u and v
import sys

ratings_file = open("./data/ratings.txt")
sn_file = open("./data/links.txt")

ratings = dict()
users = set()
users2 = set()

for line in ratings_file.readlines():
	contents = line.split()
	user = int(contents[0])

	try:
		ratings[user] += 1
	except KeyError:
		ratings[user] = 1

for line in sn_file.readlines():
	pr = False
	contents = line.split()
	u = int(contents[0])
	v = int(contents[1])


	try:
		r_u = ratings[u]
		pr = True
		users.add(u)
	except KeyError:
		r_u = 0
	
	pr = False

	try:
		r_v = ratings[v]
		pr = True
		users.add(v)
	except KeyError:
		r_v = 0

	if r_u > 0 and r_v > 0:
		users2.add(u)
		users2.add(v) 

	# if pr:
		# sys.stdout.write("%d\t%d\t%d\t%d\n" % (u, v, r_u, r_v))
print(len(users))
print(len(users2))
print(users2.difference(users))