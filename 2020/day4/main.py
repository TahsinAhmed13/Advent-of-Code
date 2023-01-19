from sys import stdin

def getFields(passport): 
    fields = {}
    for line in passport: 
        for token in line.split(): 
            [key, val] = token.split(':')
            fields[key] = val
    return fields

def valid(passports): 
    need = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}
    fields = getFields(passports).keys()
    return need.intersection(fields) == need
    
def part1(passports): 
    return len(list(filter(valid, passports)))

def valid2(passports): 
    fields = getFields(passports)
    ok_byr = 'byr' in fields.keys() and 1920 <= int(fields['byr']) and int(fields['byr']) <= 2002
    ok_iyr = 'iyr' in fields.keys() and 2010 <= int(fields['iyr']) and int(fields['iyr']) <= 2020 
    ok_eyr = 'eyr' in fields.keys() and 2020 <= int(fields['eyr']) and int(fields['eyr']) <= 2030
    ok_hgt = False
    if 'hgt' in fields.keys(): 
        if len(fields['hgt']) > 1 and fields['hgt'][-2:] == 'cm': 
            ok_hgt = 150 <= int(fields['hgt'][:-2]) and int(fields['hgt'][:-2]) <= 193
        elif len(fields['hgt']) > 1 and fields['hgt'][-2:] == 'in': 
            ok_hgt = 59 <= int(fields['hgt'][:-2]) and int(fields['hgt'][:-2]) <= 76
    ok_hcl = ('hcl' in fields.keys() and len(fields['hcl']) == 7 and fields['hcl'][0] == '#' and
        all(map(lambda x: x in 'abcdef0123456789', fields['hcl'][1:])))
    ok_ecl = 'ecl' in fields.keys() and fields['ecl'] in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}
    ok_pid = 'pid' in fields.keys() and len(fields['pid']) == 9 and all(map(str.isdigit, fields['pid']))
    return ok_byr and ok_iyr and ok_eyr and ok_hgt and ok_hcl and ok_ecl and ok_pid

def part2(passports): 
    return len(list(filter(valid2, passports)))

passports = [[]]
for line in stdin.read().splitlines(): 
    if len(line): 
        passports[-1].append(line)
    else: 
        passports.append([])
print(part1(passports))
print(part2(passports))
            
