from sys import stdin

def get_ingredients(food): 
    ingredients = food
    if '(' in food: 
        ingredients = food[:food.index('(')-1]
    return ingredients.split()

def get_allergens(food): 
    allergens = ''
    if '(' in food: 
        allergens = food[food.index('(')+10:-1]
    return allergens.split(', ')
   
def get_inert(foods): 
    contains = dict()
    for f in foods: 
        ingredients = get_ingredients(f)
        allergens = get_allergens(f)
        for a in allergens: 
            contains[a] = (contains[a].intersection(set(ingredients)) 
                if a in contains else set(ingredients))
    ingredients, allergens = list(), list()
    while contains: 
        for a, i in contains.items(): 
            if len(i) == 1: 
                break
        i = i.pop()
        ingredients.append(i)
        allergens.append(a)
        contains.pop(a) 
        for k in contains.keys(): 
            if i in contains[k]: 
                contains[k].remove(i)
    return ingredients, allergens

def part1(foods): 
    inert = set(get_inert(foods)[0])
    res = 0
    for f in foods: 
        res += len(list(filter(lambda i: i not in inert, get_ingredients(f))))
    return res

def part2(foods): 
    ingredients, allergens = get_inert(foods)
    inert = [v[1] for v in sorted(zip(allergens, ingredients))]
    return ','.join(inert)

foods = stdin.read().splitlines()
print(part1(foods))
print(part2(foods))
