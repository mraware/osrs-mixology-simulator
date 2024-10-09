import random
import copy

# Simulate the comparison of multiple inventories

inventories = [
    ["All Unlocks v1", ["MLL", "MMA", "MMA", "MML", "ALL", "MLL", "MAL", "MAL", "AAL"]],
    ["All Unlocks v2", ["AAL", "MAL", "MLL", "ALL", "MMA", "AAM", "MAL", "MLL", "MML"]],
    ["Useful Unlocks", ["LLL", "MLL", "ALL", "MMA", "MML", "MMM", "MAL", "AAL", "AAM"]],
    ["Aldarin Ratio 1", ["ALL", "MML", "MMA", "MAL", "AAM", "ALL", "MAL", "MML", "MLL"]],
    ["Aldarin Ratio 2", ['MAL', 'MLL', 'AAM', 'MMA', 'ALL', 'MAL', 'MLL', 'MAL', 'MML']],
    ["RedMare 1", ["MMM", "AAM", "ALL", "MML", "AAL", "MAL", "MAL", "MMA", "MLL"]],
    ["RedMare 2", ["MML", "AAM", "ALL", "LLL", "AAL", "MAL", "MAL", "MMA", "MLL"]],
    ["Mitchram v1 (Lye Heavy)", ["LLL", "MLL", "ALL", "MMA", "MML", "MMM", "MAL", "AAL", "AAM"]],
    ["Wizzy (MMM Only)", ["AAL", "ALL", "MMA", "MLL", "MMM", "MAL", "MAL", "AAM", "MML"]],
    ["Mox Heavy", ["MMA", "AAL", "MML", "ALL", "MMM", "MAL", "MAL", "AAM", "MLL"]],
    ["DockerContnr", ["MML", "MLL", "LLL", "MMA", "AAL", "AAM", "MAL", "MMM", "ALL"]],
]
number_of_runs = 50

# all unlocks
total_mox_needed = 61050
total_aga_needed = 52550
total_lye_needed = 70500

# 500 aldarium
# total_mox_needed = 40000
# total_aga_needed = 30000
# total_lye_needed = 45000

# useful unlocks
# total_mox_needed = 54300
# total_aga_needed = 44150
# total_lye_needed = 59400
 
class Potion:
    potion_no = 0
    def __init__(self, name, mox, aga, lye, multiplier, rarity):
        self.name = name
        self.input_mox = mox
        self.input_aga = aga
        self.input_lye = lye
        self.output_mox = mox * multiplier
        self.output_aga = aga * multiplier
        self.output_lye = lye * multiplier
        self.id_range = [Potion.potion_no, Potion.potion_no + 1, Potion.potion_no + 2]
        self.output_lye_fail = 0
        self.output_aga_fail = 0
        self.output_mox_fail = 0
        self.rarity = rarity
        if lye > 0:
            self.output_lye_fail = 10
        elif aga > 0:
            self.output_aga_fail = 10
        else:
            self.output_mox_fail = 10
        Potion.potion_no += 3
        
class Inventory:
    def __init__(self, mox_cost, aga_cost, lye_cost, mox_points, aga_points, lye_points, potions, name):
        self.mox_cost = mox_cost
        self.aga_cost = aga_cost
        self.lye_cost = lye_cost
        self.mox_points = mox_points
        self.aga_points = aga_points
        self.lye_points = lye_points
        self.potions = potions
        self.name = name
        self.num_inventories = 0
    
    def __str__(self) -> str:
        return f"Inventory({self.name} {self.potions} {self.num_inventories})"
    
    def __repr__(self) -> str:
        return self.__str__()
 
potions = [
    Potion('AAA', 0, 30, 0, 2/3, 0.119048),
    Potion('MMM', 30, 0, 0, 2/3, 0.119048),
    Potion('LLL', 0, 0, 30, 2/3, 0.119048),
    Potion('AAL', 0, 20, 10, 1, 0.095238),
    Potion('AAM', 10, 20, 0, 1, 0.095238),
    Potion('MMA', 20, 10, 0, 1, 0.095238),
    Potion('MML', 20, 0, 10, 1, 0.095238),
    Potion('ALL', 0, 10, 20, 1, 0.095238),
    Potion('MLL', 10, 0, 20, 1, 0.095238),
    Potion('MAL', 10, 10, 10, 2, 0.071429),
]
 
potions_by_id = {}
potions_by_name = {}
for potion in potions:
    potions_by_name[potion.name] = potion
    for id in potion.id_range:
        potions_by_id[id] = potion
 
def get_order():
    target = random.random()
    for potion in potions:
        step = potion.rarity / 3.
        if target < potion.rarity:
            if target < step:
                return potion.id_range[0]
            elif target < 2 * step:
                return potion.id_range[1]
            else:
                return potion.id_range[2]
        target -= potion.rarity
        
    print("Error: get_order() failed")
 
def get_names(potions):
    names = []
    for potion in potions[:9]:
        names.append(potion.name)
    return ", ".join(names)
 
def print_order(potions):
    names = []
    for potion in potions[:9]:
        names.append(potion.name)
    print(f"Potion names are {names}")

def simulate_inventory_compare(potions, orders_megalist):
    inventory = []
 
    mox_cost = 0
    aga_cost = 0
    lye_cost = 0
    # Potions 1-7 are the ones that go down the left side of the screen
    for potion in potions[:7]:
        inventory.extend(potion.id_range)
        mox_cost += (3 * potion.input_mox)
        aga_cost += (3 * potion.input_aga)
        lye_cost += (3 * potion.input_lye)
 
    for i, potion in enumerate(potions[7:9]):
        mox_cost += (3 * potion.input_mox)
        aga_cost += (3 * potion.input_aga)
        lye_cost += (3 * potion.input_lye)
        # Potions 8 and 9 need to be spliced into the inventory array since they go down the right side
        for j, id in enumerate(potion.id_range):
            inventory.insert((12 * i) + 3 + (4 *j), id)
 
 
    mox_points = 0
    aga_points = 0
    lye_points = 0
    order_count = 0
    while len(inventory) > 7:
        # if we run out of orders, generate a new order
        orders = orders_megalist[order_count] if order_count <= 2000 else [get_order(), get_order(), get_order()]
        order_count += 1
        order_failed = True
        order_multiplier = 1
        order_mox_points = 0
        order_aga_points = 0
        order_lye_points = 0
        for order in orders:
            if order in inventory:
                order_failed = False
                if (order_mox_points > 0 or order_aga_points > 0 or order_lye_points > 0):
                    order_multiplier += 0.2
                order_mox_points += potions_by_id[order].output_mox
                order_aga_points += potions_by_id[order].output_aga
                order_lye_points += potions_by_id[order].output_lye
                inventory.remove(order)
        if order_failed:
            id = inventory.pop(0)
            potion = potions_by_id[id]
            order_mox_points += potion.output_mox_fail
            order_aga_points += potion.output_aga_fail
            order_lye_points += potion.output_lye_fail
            
        mox_points += order_mox_points * order_multiplier
        aga_points += order_aga_points * order_multiplier
        lye_points += order_lye_points * order_multiplier
 
    # Decrement cost from remaining potions
    for id in inventory:
        potion = potions_by_id[id]
        mox_cost -= potion.input_mox
        aga_cost -= potion.input_aga
        lye_cost -= potion.input_lye
 
    return mox_cost, aga_cost, lye_cost, mox_points, aga_points, lye_points

def simulate_potion_order_compare(inventories, print_results):
    inventory_list = []
    for inventory in inventories:
        inventory_list.append(Inventory(0, 0, 0, 0, 0, 0, inventory[1], inventory[0]))
    
    # while each inventory has not reached the goal
    while not all(inventory.mox_points > total_mox_needed and inventory.aga_points > total_aga_needed and inventory.lye_points > total_lye_needed for inventory in inventory_list):
        # generate orders megalist that is 750 long
        # we want to use the same order list for each simulation to make it fair
        orders_megalist = []
        for i in range(750):
            orders = [get_order(), get_order(), get_order()]
            orders_megalist.append(orders)
        
        for inventory in inventory_list:
            if inventory.mox_points > total_mox_needed and inventory.aga_points > total_aga_needed and inventory.lye_points > total_lye_needed:
                continue
            run_mox_cost, run_aga_cost, run_lye_cost, run_mox_points, run_aga_points, run_lye_points = simulate_inventory_compare([potions_by_name[name] for name in inventory.potions], orders_megalist)
            inventory.mox_cost += run_mox_cost
            inventory.aga_cost += run_aga_cost
            inventory.lye_cost += run_lye_cost
            inventory.mox_points += run_mox_points
            inventory.aga_points += run_aga_points
            inventory.lye_points += run_lye_points
            inventory.num_inventories += 1
            
        
    # sort inventory list by num_inventories
    inventory_list.sort(key=lambda x: x.num_inventories)
        
    for inventory in inventory_list:
        efficiency = (inventory.mox_points + inventory.aga_points + inventory.lye_points + (inventory.num_inventories * 60)) / (inventory.mox_cost + inventory.aga_cost + inventory.lye_cost)
        if print_results :
            print(f"Name: {inventory.name}")
            print_order([potions_by_name[name] for name in inventory.potions])
            print(f"Mox used: {inventory.mox_cost}, Aga used: {inventory.aga_cost}, Lye used: {inventory.lye_cost}")
            print(f"Mox earned: {inventory.mox_points}, Aga earned: {inventory.aga_points}, Lye earned: {inventory.lye_points}")
            print(f"Ratio: ({inventory.mox_points / inventory.lye_points}:{inventory.aga_points / inventory.lye_points}:{inventory.lye_points / inventory.lye_points}")
            print(f"Efficiency:{efficiency}")
            print(f"Took {inventory.num_inventories} inventories to complete")
            
    return inventory_list

def find_inventory_in_list(inventory_list, name):
    for inventory in inventory_list:
        if inventory.name == name:
            return inventory
    return None

runs = []
for i in range(number_of_runs):
    print(f"Simulation {i + 1}")
    run = simulate_potion_order_compare(inventories, False)
    runs.append(run)
    
# average all the inventories in the runs
average_inventories = []
for inventory in inventories:
    average_inventories.append(Inventory(0, 0, 0, 0, 0, 0, inventory[1], inventory[0]))
    
for run in runs:
    for i, inventory in enumerate(run):
        # get inventory by inventory.name
        # add mox_cost, aga_cost, lye_cost, mox_points, aga_points, lye_points, num_inventories
        average_inventory = find_inventory_in_list(average_inventories, inventory.name)
        
        average_inventory.mox_cost += inventory.mox_cost
        average_inventory.aga_cost += inventory.aga_cost
        average_inventory.lye_cost += inventory.lye_cost
        average_inventory.mox_points += inventory.mox_points
        average_inventory.aga_points += inventory.aga_points
        average_inventory.lye_points += inventory.lye_points
        average_inventory.num_inventories += inventory.num_inventories

for inventory in average_inventories:
    inventory.mox_cost /= number_of_runs
    inventory.aga_cost /= number_of_runs
    inventory.lye_cost /= number_of_runs
    inventory.mox_points /= number_of_runs
    inventory.aga_points /= number_of_runs
    inventory.lye_points /= number_of_runs
    inventory.num_inventories /= number_of_runs
    
# sort average inventories by num_inventories
average_inventories.sort(key=lambda x: x.num_inventories)
    
for inventory in average_inventories:
    efficiency = (inventory.mox_points + inventory.aga_points + inventory.lye_points + (inventory.num_inventories * 60)) / (inventory.mox_cost + inventory.aga_cost + inventory.lye_cost)
    print(f"Name: {inventory.name}")
    print_order([potions_by_name[name] for name in inventory.potions])
    print(f"Mox used: {inventory.mox_cost}, Aga used: {inventory.aga_cost}, Lye used: {inventory.lye_cost}")
    print(f"Mox earned: {inventory.mox_points}, Aga earned: {inventory.aga_points}, Lye earned: {inventory.lye_points}")
    print(f"Ratio: ({inventory.mox_points / inventory.lye_points}:{inventory.aga_points / inventory.lye_points}:{inventory.lye_points / inventory.lye_points}")
    print(f"Efficiency:{efficiency}")
    print(f"Took {inventory.num_inventories} inventories to green log")
    print("\n")
    

    

#test_order = ['LLL', 'MLL', 'ALL', 'MMA', 'MML', 'MMM', 'MAL', 'AAL', 'AAM']
# test_order = ["MLL", "MMA", "MMA", "MML", "ALL", "MLL", "MAL", "MAL", "AAL"]
# simulate_potion_order([potions_by_name[name] for name in test_order], True)
# test_order = ["AAL", "ALL", "MMA", "MLL", "MMM", "MAL", "MAL", "AAM", "MML"]
# simulate_potion_order([potions_by_name[name] for name in test_order], True)

