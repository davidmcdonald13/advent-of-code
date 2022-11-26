import sys


class Room:
    def __init__(self, raw_string):
        components = raw_string.strip().split("-")

        self.name = components[:-1]

        sector_id, self.checksum = components[-1][:-1].split("[")
        self.sector_id = int(sector_id)

    def is_real(self):
        letter_counts = dict()

        for word in self.name:
            for c in word:
                letter_counts[c] = letter_counts.get(c, 0) + 1

        if len(letter_counts) < 5 or len(self.checksum) != 5:
            return False

        common = sorted(letter_counts.keys(), key=lambda c: (-letter_counts[c], c))

        return "".join(common[:5]) == self.checksum

    def _decrypt(self, word):
        result = []
        for c in word:
            new_letter = chr(ord('a') + (ord(c) - ord('a') + self.sector_id) % 26)
            result.append(new_letter)
        return "".join(result)

    def decrypt(self):
        return " ".join([self._decrypt(word) for word in self.name])


def decryption_test():
    room = Room("qzmt-zixmtkozy-ivhz-343[decoy]")
    assert(room.decrypt() == "very encrypted name")


def room_sum(filename):
    result = 0
    with open(filename, "r") as f:
        for line in f.readlines():
            room = Room(line)

            if room.is_real():
                result += room.sector_id

            if room.decrypt() == "northpole object storage":
                print("northpole object store:", room.sector_id)

    return result


if __name__ == "__main__":
    decryption_test()
    print(room_sum(sys.argv[1]))
