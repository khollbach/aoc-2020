import re

passports = []
for paragraph in open('../inputs/4').read().split('\n\n'):
    fields = {}
    for word in paragraph.split():
        k, v = word.split(':')
        fields[k] = v
    passports.append(fields)

req_fields = 'byr iyr eyr hgt hcl ecl pid'.split()
def is_valid(p: dict) -> bool:
    in_p = lambda k: k in p
    return all(map(in_p, req_fields))

count = len(list(filter(is_valid, passports)))
print(count)

def is_valid_2(p: dict) -> bool:
    valid = lambda kv: field_valid(*kv)
    return is_valid(p) and all(map(valid, p.items()))

def field_valid(k: str, v: str) -> bool:
    def in_range(s: str, low, high) -> bool:
        try:
            return low <= int(s) <= high
        except:
            return False

    def range_4(s: str, low, high) -> bool:
        return len(s) == 4 and in_range(s, low, high)

    if k == 'byr':
        return range_4(v, 1920, 2002)
    elif k == 'iyr':
        return range_4(v, 2010, 2020)
    elif k == 'eyr':
        return range_4(v, 2020, 2030)
    elif k == 'hgt':
        if v.endswith('cm'):
            return in_range(v[:-2], 150, 193)
        elif v.endswith('in'):
            return in_range(v[:-2], 59, 76)
        else:
            return False
    elif k == 'hcl':
        return re.match(r'#[0-9a-f]{6}', v)
    elif k == 'ecl':
        return v in 'amb blu brn gry grn hzl oth'.split()
    elif k == 'pid':
        return len(v) == 9 and all(map(str.isdigit, v))
    elif k == 'cid':
        return True
    else:
        return False

count = len(list(filter(is_valid_2, passports)))
print(count)
