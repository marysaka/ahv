import sys

with open("raw.txt") as f:
    lines = f.readlines()

for i in range(0, len(lines) / 3):
    documentation_line = lines[i * 3].replace('\n', '')
    declaration_line = lines[i * 3 + 1].replace('\n', '')

    reg_name = documentation_line.replace('.', '').replace('/// ', '').replace('The value that identifies register ', '')

    print("SystemRegister::{} => HV_SYS_REG_{},".format(reg_name, reg_name))