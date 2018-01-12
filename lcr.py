import sys
import math
from collections import namedtuple

Player = namedtuple('Person', ['num'])

def create_states(players):
    states = []
    chips = 3 * len(players)
    players_permut = powerset(players)
    players_permut.remove([])

    # As chips get placed in the center, the number of chips
    # in play decreases from '3 * players' to '1
    for num_chips in range(1, chips + 1):
        # Determine how many ways a number of chips
        # can be distributed among the players. Players
        # can have 0 chips without being out of the game,
        # so we need to count all of the ways the chips
        # can be distributed when anywhere from '0' to 'p - 1'
        # players don't have any chips (there's always at least
        # one player with chips)
        for combination in players_permut:
            permutes = permute_chips(combination, num_chips)
            for permute in permutes:
                for player in players:
                    if not player in permute:
                        permute[player] = 0

            for turn in map(lambda p: p.num, players):
                states.append((turn, permutes))
    return states


# For each playe:
#     for each number of chips from 0 to all
#         assign them that number of chips
#         run this again without that player, including the generated
#         assignment
def permute_chips(players, chips):
    final = []
    player = next(iter(players))

    if len(players) == 1:
        final.append({player: chips})
        return final

    for num_chips in range(chips + 1):
        other_players = set(players)
        other_players.remove(player)

        other_permuts = permute_chips(other_players, chips - num_chips)
        for permut in other_permuts:
            permut[player] = num_chips
            final.append(dict(permut))

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

    #return (k * stirling(n - 1, k)) + stirling(n, k - 1)

def main():
    players = []
    for i in range(int(sys.argv[1])):
        players.append(Player(i))


    num_players = len(players)
    num_chips = 3 * num_players

    permuts = permute_chips(players, num_chips)
    states = create_states(players)

    #print(permuts)
    #print("States: ", states)
    print("Num states: ", len(states))
    print("Computed permutes: ", len(permuts))

if __name__ == "__main__":
    main()
