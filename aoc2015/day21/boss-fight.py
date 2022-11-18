import queue


class Item:
    def __init__(self, name, cost, damage, armor):
        self.name = name
        self.cost = cost
        self.damage = damage
        self.armor = armor

    def __lt__(self, other):
        return self.cost < other.cost


def get_all_combos():
    weapons = [
        Item("Dagger", 8, 4, 0),
        Item("Shortsword", 10, 5, 0),
        Item("Warhammer", 25, 6, 0),
        Item("Longsword", 40, 7, 0),
        Item("Greataxe", 74, 8, 0),
        ]

    armors = [
        Item("Empty", 0, 0, 0),
        Item("Leather", 13, 0, 1),
        Item("Chainmail", 31, 0, 2),
        Item("Splintmail", 53, 0, 3),
        Item("Bandedmail", 75, 0, 4),
        Item("Platemail", 102, 0, 5),
        ]

    rings = [
        Item("Empty1", 0, 0, 0),
        Item("Empty2", 0, 0, 0),
        Item("Damage +1", 25, 1, 0),
        Item("Damage +2", 50, 2, 0),
        Item("Damage +3", 100, 3, 0),
        Item("Defense +1", 20, 0, 1),
        Item("Defense +2", 40, 0, 2),
        Item("Defense +3", 80, 0, 3),
        ]

    combos = list()
    for weapon in weapons:
        for armor in armors:
            for i, ring1 in enumerate(rings):
                for ring2 in rings[i+1:]:
                    combos.append([weapon, armor, ring1, ring2])
    return combos


def hits_to_kill(damage, hp, armor):
    actual_damage = max(damage - armor, 1)
    return hp // actual_damage + int(bool(hp % actual_damage))


def player_wins(combo, enemy_hit_points, enemy_damage, enemy_armor):
    player_hit_points = 100
    player_damage = sum(x.damage for x in combo)
    player_armor = sum(x.armor for x in combo)

    # number of hits needed for player to kill enemy
    player_hits_needed = hits_to_kill(player_damage, enemy_hit_points, enemy_armor)

    # number of hits needed for enemy to kill player
    enemy_hits_needed = hits_to_kill(enemy_damage, player_hit_points, player_armor)

    return player_hits_needed <= enemy_hits_needed


def optimizer(enemy_hit_points, enemy_damage, enemy_armor, min_cost=True, player_win=True):
    combos = get_all_combos()
    pq = queue.PriorityQueue()
    for combo in combos:
        cost = sum(x.cost for x in combo)
        if not min_cost:
            cost = -cost
        pq.put((cost, combo))

    while not pq.empty():
        cost, combo = pq.get()

        if player_win == player_wins(combo, enemy_hit_points, enemy_damage, enemy_armor):
            if not min_cost:
                cost = -cost
            return cost


if __name__ == "__main__":
    hit_points = 100
    damage = 8
    armor = 2
    print(optimizer(hit_points, damage, armor))
    print(optimizer(hit_points, damage, armor, min_cost=False, player_win=False))
