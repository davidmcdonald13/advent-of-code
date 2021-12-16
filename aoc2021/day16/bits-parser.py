import sys


class Packet:
    def __init__(self, version, type_id):
        self.version = version
        self.type_id = type_id

    def version_sum(self):
        raise NotImplementedError

    def get_value(self):
        raise NotImplementedError


class Operator(Packet):
    def __init__(self, version, type_id, sub_packets):
        Packet.__init__(self, version, type_id)
        self.sub_packets = sub_packets

    def version_sum(self):
        return self.version + sum([packet.version_sum() for packet in self.sub_packets])

    def get_value(self):
        if self.type_id == 0:
            return sum([packet.get_value() for packet in self.sub_packets])
        elif self.type_id == 1:
            return product([packet.get_value() for packet in self.sub_packets])
        elif self.type_id == 2:
            return min([packet.get_value() for packet in self.sub_packets])
        elif self.type_id == 3:
            return max([packet.get_value() for packet in self.sub_packets])
        elif self.type_id == 5:
            return int(self.sub_packets[0].get_value() > self.sub_packets[1].get_value())
        elif self.type_id == 6:
            return int(self.sub_packets[0].get_value() < self.sub_packets[1].get_value())
        elif self.type_id == 7:
            return int(self.sub_packets[0].get_value() == self.sub_packets[1].get_value())


class LiteralValue(Packet):
    def __init__(self, version, type_id, value):
        Packet.__init__(self, version, type_id)
        self.value = value

    def version_sum(self):
        return self.version

    def get_value(self):
        return self.value


def product(arr):
    if not arr:
        return 0
    result = 1
    for x in arr:
        result *= x
    return result


def parse_literal_value_body(body):
    chunks = list()
    while True:
        group = body[:5]
        body = body[5:]

        chunks.append(group[1:])
        if group[0] == "0":
            break

    value = int("".join(chunks), 2)

    return value, body


# returns packet and remainder (which may be None or empty string)
def parse_packet(binary, padding=""):
    version = int(binary[:3], 2)
    type_id = int(binary[3:6], 2)

    if type_id == 4:
        value, remainder = parse_literal_value_body(binary[6:])
        #print(padding + "found literal with value: " + str(value))
        return LiteralValue(version, type_id, value), remainder
    else:
        if binary[6] == "0":
            # next 15 bits are a number that represents the total length in bits
            # of the sub-packets contained by this packet
            sub_packets_length = int(binary[7:22], 2)
            #print(padding + "found operator with " + str(sub_packets_length) + " sub-packet BITS")
            sub_packets_repr = binary[22:22+sub_packets_length]
            sub_packets = list()
            while len(sub_packets_repr) > 7:
                packet, sub_packets_repr = parse_packet(sub_packets_repr, padding=padding + "    ")
                sub_packets.append(packet)
            return Operator(version, type_id, sub_packets), binary[22+sub_packets_length:]
        else:
            # next 11 bits are a number that represents the number of sub-packets
            # immediately contained by this packet
            sub_packets_count = int(binary[7:18], 2)
            #print(padding + "found operator with " + str(sub_packets_count) + " sub-packets")
            remainder = binary[18:]
            sub_packets = list()
            while len(sub_packets) < sub_packets_count:
                packet, remainder = parse_packet(remainder, padding=padding + "    ")
                sub_packets.append(packet)
            return Operator(version, type_id, sub_packets), remainder


def hex_to_bin(s):
    result = ""
    for c in s:
        result += {
                "0": "0000",
                "1": "0001",
                "2": "0010",
                "3": "0011",
                "4": "0100",
                "5": "0101",
                "6": "0110",
                "7": "0111",
                "8": "1000",
                "9": "1001",
                "A": "1010",
                "B": "1011",
                "C": "1100",
                "D": "1101",
                "E": "1110",
                "F": "1111",
                }[c]

    return result


if __name__ == "__main__":
    try:
        hex_to_bin(sys.argv[1])
        lines = [sys.argv[1]]
    except:
        lines = list()
        with open(sys.argv[1], 'r') as f:
            for line in f.readlines():
                lines.append(line.strip())

    for line in lines:
        binary_repr = hex_to_bin(line.strip())

        print(line.strip())

        packet, _ = parse_packet(binary_repr)

        print("value:", packet.get_value())
        print("version sum:", packet.version_sum())
        print()
