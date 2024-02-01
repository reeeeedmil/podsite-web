import rsnet


def parse_address(address: str):
    if type(address) is not str:
        raise TypeError("Address in not in string form.")
    if len(address) > 32:
        raise Exception(f"Address too big ({address})")
    address = address.split(".")
    if len(address) != 4:
        if len(address) > 4:
            raise Exception(f"More than four bytes in address ({address})")
        else:
            raise Exception(f"Less than four bytes in address ({address})")
    address = [int(byte) for byte in address]
    for byte in address:
        if byte > 255 or byte < 0:
            raise Exception(f"Byte {byte} out of range 0-255 ({address})")
    return address


def validate_mask(address):
    if len(address) != 4:
        if len(address) > 4:
            raise Exception(f"More than four bytes in address ({address})")
        else:
            raise Exception(f"Less than four bytes in address ({address})")
    for i in range(0, 3):
        if address[i] < address[i+1]:
            raise Exception(
                f"Byte {address[i]} is smaller than {address[i+1]} ({address})"
            )
    return address
