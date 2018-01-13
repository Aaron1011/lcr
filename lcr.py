import sys
import math
from collections import namedtuple
import time
import numpy

Player = namedtuple('Person', ['num'])

def create_states(players):
    states = []
    chips = 3 * len(players)
    #players_permut = powerset(players)
    #players_permut.remove([])

    # As chips get placed in the center, the number of chips
    # in play decreases from '3 * players' to '1
    for num_chips in range(1, chips + 1):
        permutes = permute_chips(players, num_chips)
        for turn in players:
            for permute in permutes:
                states.append((turn, permute))

    return states


# For each playe:
#     for each number of chips from 0 to all
#         assign them that number of chips
#         run this again without that player, including the generated
#         assignment
def permute_chips(players, chips):
    final = []
    player = players[0]

    if len(players) == 1:
        final.append([(player, chips)])
        return final

    for num_chips in range(chips + 1):
        #other_players = set(players)
        #other_players.remove(player)
        #other_players = iter(players)
        #next(other_players)

        other_permuts = permute_chips(players[1:], chips - num_chips)
        for permut in other_permuts:
            permut.append((player, num_chips))
            final.append(list(permut))

    return final
   

def permute_chips_numpy(players, chips):
    final = None
    player = players[0]

    if len(players) == 1:
        #final.append([(player, chips)])
        return numpy.array([[[player, chips]]])

    for num_chips in range(chips + 1):
        other_permuts = permute_chips_numpy(numpy.delete(players, 0), chips - num_chips)

        #other_permuts = permute_chips(other_players, chips - num_chips)
        for permut in other_permuts:
            permut = numpy.append(permut, [[player, num_chips]], axis=0)
            if final is None:
                final = numpy.array([permut])
            else:
                final = numpy.append(final, [permut], axis=0)

    return final



def powerset(s):
    result = [[]]

    for elem in s:
        result.extend([x + [elem] for x in result])
    return result


def binom(n, k):
    return int(math.factorial(n) / (math.factorial(n-k) * math.factorial(k)))

def stirling(n, k):
    if k == 0:
        if n == 0:
            return 1
        return 0

    total = 0
    for i in range(k + 1):
        total += ((-1)**(k-i)) * binom(k, i) * (i**n)

    return (1.0 / math.factorial(k)) * total

def time_fn(fn, *args, **kwargs):
    before = time.time()
    result = fn(*args, **kwargs)
    after = time.time()

    return (after - before, result)

    #return (k * stirling(n - 1, k)) + stirling(n, k - 1)

def main():
    players = []
    #for i in range(int(sys.argv[1])):
    #    players.append(Player(i))
    players = [i for i in range(int(sys.argv[1]))]


    num_players = len(players)
    num_chips = 3 * num_players

    
    (orig_time, permuts) = time_fn(permute_chips, players, num_chips)
    #(numpy_time, numpy_permuts) = time_fn(permute_chips_numpy, numpy.array(players), num_chips)
    states = create_states(players)

    #print(permuts)
    #print("States: ", states)
    print("Num states: ", len(states))
    print("Computed permutes: ", len(permuts))
    #print(permuts)
    #print(numpy_permuts)
    #print("Regular time: ", orig_time)
    #print("Numpy time: ", numpy_time)
    print("Calculated permutes: ", binom(num_chips + num_players - 1,
        num_chips))

if __name__ == "__main__":
    main()
