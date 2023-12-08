class IPv4Network():
    def __init__(self):
        network_address = [0, 0, 0, 0]
        prefix = 32
        mask = [255, 255, 255, 255]

    def update_network_address(self, network_address):
        self.network_address = network_address

    def update_mask(self, mask):
        self.mask = mask

    def update_broadcast(self, broadcast):
        self.broadcast = broadcast

    def update_wildcard(self, wildcard):
        self.wildcard = wildcard

    def update_prefix(self, prefix):
        self.prefix = prefix

def parse_for_calculations(unparsed_data):
    data = unparsed_data.split('.') # turns '000.000.000.000' into ['000','000','000','000']
    data = [int(number) for number in data] # turns the list of strings into a list of numbers - this also removes leading zeros

    print(data)

parse_for_calculations('000.000.000.000')

def broadcast_calculation(network_address, mask):
    broadcast = []

    for i in range (0, 4):
        broadcast.append( network_address[i] | (~mask[i]&255) )
    return broadcast

def wildcard_calculation(mask):
    wildcard = []
    for i in range(0, 4):
        wildcard.append( ~mask[i] & 255 )
    return wildcard

def mask_from_prefix(prefix):
    decimal_mask = 2**32 - 2**(32-prefix)
    binary_mask = format(decimal_mask, 'b')
    mask_binary_bytes = [(binary_mask[i:i + 8]) for i in range (0, len(binary_mask), 8)]
    #
    # list comprehension vysvetleni
    # [i:i + 8] => split stringu od n do n+8 znaku (napr. 0,8); +8 protoze posledni znak se nepocita
    # for i in range => vytvori cisla znaku na split
    # 0, len(binarni_maska), 8 => zacne indexovat od 0, jde az do konce masky(32-1), skace po 8 znacich => 0,7; 8,15; 16,23; 24,31
    # stejny jako:
    #
    # maska_bytes = []
    # for i in range(0, len(binarni_maska), 8):
    #       maska_bytes.append(binarni_maska[i:i+8])
    #
    #
    print(decimal_mask)
    mask = []
    for i in range(0, 4):
        mask.append( int(mask_binary_bytes[i], 2) )
    return mask

def prefix_from_mask(mask):
    bits = 0
    zeroes = 0
    for i in range(0, 4):
        mask[i] = format(mask[i], 'b')
        ones = mask[i].replace("0", "")
        bits += len(ones)
        mask[i] = int(mask[i], 2)
    return bits

def network_size_normalization(number_of_hosts):
    number_of_hosts += 1  #1, ne 2, protože tím změním jenom poslední bit -> podsíť nebude mít nechtěné zvětšení se
                       #například 2 by potom bylo 4 -> to už je 100, ne 11, tudíž by se to zařadilo do velikosti podsítě 8 adres
    number_of_hosts = bin(number_of_hosts)
    power = len(number_of_hosts)-2 #toto je protože bin() dá do stringu 0b před číslo -> musí se odečíst od délky
    number_of_addresses = 2**power
    return number_of_addresses
