# FITstone virtual machine compiler
# Robert Heine, rheine2019@my.fit.edu

# Input should be the name of the instructions file
# Compiles to a file with the same name as the instructions file, bit with a .bin extension.
files = input().rstrip().split(" ")
f_txt = open(files[0], "rt")
binfile_name = files[0].split(".")
f_bin = open(binfile_name[0] + ".bin", "wb")

# Reads each instruction line by line, then converts to bytes to write to the bin file
for line in f_txt:
    args = line.rstrip().split(" ")
    if args[0] == "LOAD":
        tmp = b'\x00' + (int(args[1])).to_bytes(1, byteorder='big') + (int(args[2])).to_bytes(4, byteorder='big')
        f_bin.write(tmp)
    elif args[0] == "RLOAD" or args[0] == "CMP":
        tmp = b'\x09'
        if args[0] == "RLOAD":
            tmp = b'\x01'
        tmp += (int(args[1])).to_bytes(1, byteorder='big') + (int(args[2])).to_bytes(1, byteorder='big')
        f_bin.write(tmp)
    elif args[0] == "PUSH" or args[0] == "POP":
        tmp = b'\x03'
        if args[0] == "PUSH":
            tmp = b'\x02'
        tmp += (int(args[1])).to_bytes(1, byteorder='big')
        f_bin.write(tmp)
    elif args[0] == "ADD" or args[0] == "MUL" or args[0] == "SUB" or args[0] == "DIV":
        tmp = b'\x04'
        if args[0] == "SUB":
            tmp = b'\x05'
        elif args[0] == "MUL":
            tmp = b'\x06'
        elif args[0] == "DIV":
            tmp == b'\x07'
        tmp += (int(args[1])).to_bytes(1, byteorder='big') + (int(args[2])).to_bytes(1, byteorder='big') + (int(args[3])).to_bytes(1, byteorder='big')
        f_bin.write(tmp)
    elif args[0] == "JMP" or args[0] == "BLT" or args[0] == "BEQ" or args[0] == "BGT" or args[0] == "BNE":
        tmp = b'\x08'
        if args[0] == "BLT":
            tmp = b'\x0a'
        elif args[0] == "BEQ":
            tmp = b'\x0b'
        elif args[0] == "BGT":
            tmp = b'\x0c'
        elif args[0] == "BNE":
            tmp = b'\x0d'
        tmp += (int(args[1])).to_bytes(4, byteorder='big')
        f_bin.write(tmp)

f_txt.close()
f_bin.close()