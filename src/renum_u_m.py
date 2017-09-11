# Renum users with even integers, movie with odd ones
import sys

user_cpt = 0
movie_cpt = 1

users = {}
movies = {}

for line in sys.stdin:
	contents = line.split()
	u = int(contents[0])
	m = int(contents[1])
	r = float(contents[2])

	try:
		id_u = users[u]
	except KeyError:
		users[u] = user_cpt
		user_cpt += 2
		id_u = users[u]

	try:
		id_m = movies[m]
	except KeyError:
		movies[m] = movie_cpt
		movie_cpt += 2
		id_m = movies[m]

	sys.stdout.write("%d %d %f\n" % (id_u, id_m, r))
