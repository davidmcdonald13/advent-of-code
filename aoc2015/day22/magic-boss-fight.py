import copy
import queue


# represents the state of the game immediately before a turn
class GameState:
    def __init__(self, player_hp, player_mana, enemy_hp, enemy_damage, player_next, shield_turns_remaining, poison_turns_remaining, recharge_turns_remaining, log, hard_mode=False):
        self.player_hp = player_hp
        self.player_mana = player_mana
        self.enemy_hp = enemy_hp
        self.enemy_damage = enemy_damage
        self.player_next = player_next
        self.shield_turns_remaining = shield_turns_remaining
        self.poison_turns_remaining = poison_turns_remaining
        self.recharge_turns_remaining = recharge_turns_remaining
        self.log = log
        self.hard_mode = hard_mode

    def __lt__(self, other):
        return self.enemy_hp < other.enemy_hp

    def player_won(self):
        return self.enemy_hp <= 0

    def enemy_won(self):
        return self.player_hp <= 0

    def process_effects(self):
        if self.hard_mode and self.player_next:
            self.player_hp -= 1
            if self.player_hp <= 0:
                return

        armor = 0
        if self.shield_turns_remaining > 0:
            armor += 7
            self.shield_turns_remaining -= 1

        if self.poison_turns_remaining > 0:
            self.enemy_hp -= 3
            self.poison_turns_remaining -= 1

        if self.recharge_turns_remaining > 0:
            self.player_mana += 101
            self.recharge_turns_remaining -= 1

        if not self.player_next:
            self.player_hp -= (self.enemy_damage - armor)

        self.player_next = not self.player_next

    def copy(self):
        return GameState(self.player_hp, self.player_mana, self.enemy_hp, self.enemy_damage, self.player_next, self.shield_turns_remaining, self.poison_turns_remaining, self.recharge_turns_remaining, copy.deepcopy(self.log), self.hard_mode)


def min_cost(player_hp, mana, enemy_hp, enemy_damage, hard_mode=False):
    pq = queue.PriorityQueue()

    pq.put((0, GameState(player_hp, mana, enemy_hp, enemy_damage, True, 0, 0, 0, [], hard_mode)))

    while not pq.empty():
        cumulative_cost, game = pq.get()

        if 'nothing' in game.log:
            continue
        if game.player_mana < 0:
            continue

        if game.enemy_won():
            continue

        if game.player_won():
            print(f"[pre_turn]cost={cumulative_cost}, player_up={game.player_next}, player_hp={game.player_hp}, enemy_hp={game.enemy_hp}, log={game.log}")
            return cumulative_cost

        game.process_effects()

        if game.player_won():
            print(f"[post_turn]cost={cumulative_cost}, player_up={game.player_next}, player_hp={game.player_hp}, enemy_hp={game.enemy_hp}, log={game.log}")
            return cumulative_cost

        if game.enemy_won():
            continue

        # we've already toggled this property
        if not game.player_next:
            before = pq.qsize()

            # magic missile
            cum_cost = cumulative_cost + 53
            new_game = game.copy()
            new_game.player_mana -= 53
            new_game.enemy_hp -= 4
            new_game.log.append('missile')
            pq.put((cum_cost, new_game))

            # drain
            cum_cost = cumulative_cost + 73
            new_game = game.copy()
            new_game.player_mana -= 73
            new_game.enemy_hp -= 2
            new_game.player_hp += 2
            new_game.log.append('drain')
            pq.put((cum_cost, new_game))

            # shield
            cum_cost = cumulative_cost + 113
            new_game = game.copy()
            if new_game.shield_turns_remaining <= 0:
                new_game.player_mana -= 113
                new_game.shield_turns_remaining = 6
                new_game.log.append('shield')
                pq.put((cum_cost, new_game))

            # poison
            cum_cost = cumulative_cost + 173
            new_game = game.copy()
            if new_game.poison_turns_remaining <= 0:
                new_game.player_mana -= 173
                new_game.poison_turns_remaining = 6
                new_game.log.append('poison')
                pq.put((cum_cost, new_game))

            # recharge
            cum_cost = cumulative_cost + 229
            new_game = game.copy()
            if new_game.recharge_turns_remaining <= 0:
                new_game.player_mana -= 229
                new_game.recharge_turns_remaining = 5
                new_game.log.append('recharge')
                pq.put((cum_cost, new_game))

            after = pq.qsize()

            if before == after:
                # [no spell]
                cum_cost = cumulative_cost
                new_game = game.copy()
                new_game.log.append('nothing')
                pq.put((cum_cost, new_game))
        else:
            new_game = game.copy()
            pq.put((cumulative_cost, new_game))


if __name__ == "__main__":
    player_hp = 10
    mana = 250
    enemy_hp = 13
    enemy_damage = 8
    assert(min_cost(player_hp, mana, enemy_hp, enemy_damage) == 226)
    print()

    player_hp = 10
    mana = 250
    enemy_hp = 14
    enemy_damage = 8
    assert(min_cost(player_hp, mana, enemy_hp, enemy_damage) == 641)
    print()

    player_hp = 50
    mana = 500
    enemy_hp = 58
    enemy_damage = 9
    hard_mode = True
    print(min_cost(player_hp, mana, enemy_hp, enemy_damage, hard_mode))
