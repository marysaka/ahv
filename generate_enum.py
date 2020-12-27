import sys

with open("raw.txt") as f:
    lines = f.readlines()

for i in range(0, len(lines) / 3):
    documentation_line = lines[i * 3].replace('\n', '')
    declaration_line = lines[i * 3 + 1].replace('\n', '')

    documentation_line = documentation_line.replace('.', '').replace('The value that identifies register ', '')
    reg_name = documentation_line.replace('.', '').replace('/// ', '').replace('The value that identifies register ', '')
    documentation_line = '    ' + documentation_line + ' register.'

    declaration_line = declaration_line.replace('pub const HV_SYS_REG_', '    ').replace(': hv_sys_reg_t', '').replace(';', ',')



    print(documentation_line)
    print('    ' + reg_name + ',\n')