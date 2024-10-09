use rand::Rng;
use std::collections::HashMap;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering, AtomicI32};


// start of configuration

// Target values for Mox, Aga, and Lye

// All unlocks
static MOX_TARGET: i32 = 61050;
static AGA_TARGET: i32 = 52550;
static LYE_TARGET: i32 = 70500;

// // 500 aldarium
// static MOX_TARGET: i32 = 40000;
// static AGA_TARGET: i32 = 30000;
// static LYE_TARGET: i32 = 45000;

// // Useful unlocks (excludes cosmetics)
// static MOX_TARGET: i32 = 54300;
// static AGA_TARGET: i32 = 44150;
// static LYE_TARGET: i32 = 59400;


// Number of simulations every layout will run once
static MINI_SIMS_PER_LAYOUT: i32 = 3; 

// Number of simulations to run for each layout if mini sim performs better than existing layouts
static DEEP_SIMS_PER_LAYOUT: i32 = 15; 



// end of configuration

#[derive(Clone)]
struct Potion {
    name: String,
    input_mox: i32,
    input_aga: i32,
    input_lye: i32,
    output_mox: i32,
    output_aga: i32,
    output_lye: i32,
    output_mox_fail: i32,
    output_aga_fail: i32,
    output_lye_fail: i32,
    rarity: f64,
    id_range: Vec<i32>,
}

impl Potion {
    fn new(name: &str, mox: i32, aga: i32, lye: i32, multiplier: f64, rarity: f64) -> Potion {
        let potion_no = POTION_COUNT.fetch_add(3, std::sync::atomic::Ordering::SeqCst); // Atomically increment potion_no
        let mut output_mox_fail = 0;
        let mut output_aga_fail = 0;
        let mut output_lye_fail = 0;

        if lye > 0 {
            output_lye_fail = 10;
        } else if aga > 0 {
            output_aga_fail = 10;
        } else {
            output_mox_fail = 10;
        }

        Potion {
            name: name.to_string(),
            input_mox: mox,
            input_aga: aga,
            input_lye: lye,
            output_mox: (mox as f64 * multiplier) as i32,
            output_aga: (aga as f64 * multiplier) as i32,
            output_lye: (lye as f64 * multiplier) as i32,
            output_mox_fail,
            output_aga_fail,
            output_lye_fail,
            rarity,
            id_range: vec![potion_no, potion_no + 1, potion_no + 2],
        }
    }
}

static POTION_COUNT: AtomicI32 = AtomicI32::new(0);

// Make these 1/20 of the original values for faster testing
static MOX_TARGET_SMALL: i32 = MOX_TARGET / 20;
static AGA_TARGET_SMALL: i32 = AGA_TARGET / 20;
static LYE_TARGET_SMALL: i32 = LYE_TARGET / 20;

fn get_order(potions: &[Potion]) -> i32 {
    let mut rng = rand::thread_rng();
    let mut target = rng.gen::<f64>();

    for potion in potions {
        let step = potion.rarity / 3.0;
        if target < potion.rarity {
            if target < step {
                return potion.id_range[0];
            } else if target < 2.0 * step {
                return potion.id_range[1];
            } else {
                return potion.id_range[2];
            }
        }
        target -= potion.rarity;
    }

    panic!("No potion selected!"); // If this is reached, the logic isn't working correctly
}

fn print_order(potions: &[Potion]) {
    let names: Vec<String> = potions.iter().map(|p| p.name.clone()).collect();
    println!("Potion names are {:?}", names);
}

fn simulate_inventory(
  potions_subset: &[Potion], // This is the subset of 9 potions used for the simulation
  potions_full_list: &[Potion], // This is the full list of all potions for random order generation
  potions_by_id: &HashMap<i32, Potion>,
) -> (i32, i32, i32, i32, i32, i32) {
  let mut inventory: Vec<i32> = Vec::new();
  let mut mox_cost = 0;
  let mut aga_cost = 0;
  let mut lye_cost = 0;

  // Add the first 7 potions in the subset to the inventory
  for potion in &potions_subset[0..7] {
      inventory.extend(&potion.id_range);
      mox_cost += 3 * potion.input_mox;
      aga_cost += 3 * potion.input_aga;
      lye_cost += 3 * potion.input_lye;
  }

  // Potions 8 and 9 (going down the right side) are added separately
  for (i, potion) in potions_subset[7..9].iter().enumerate() {
      mox_cost += 3 * potion.input_mox;
      aga_cost += 3 * potion.input_aga;
      lye_cost += 3 * potion.input_lye;
      for (j, id) in potion.id_range.iter().enumerate() {
          inventory.insert((12 * i) + 3 + (4 * j), *id);
      }
  }

  let mut mox_points = 0;
  let mut aga_points = 0;
  let mut lye_points = 0;

  // Main loop: continue processing the inventory until it has 7 or fewer items
  while inventory.len() > 7 {
      // IMPORTANT: Use the full list of potions to get random orders
      let orders = vec![
          get_order(&potions_full_list), // Full potion list for randomness
          get_order(&potions_full_list),
          get_order(&potions_full_list),
      ];

      let mut order_failed = true;
      let mut order_multiplier = 1.0;
      let mut order_mox_points = 0;
      let mut order_aga_points = 0;
      let mut order_lye_points = 0;

      // Check each order and process it if it exists in the inventory
      for order in orders {
          if let Some(index) = inventory.iter().position(|&id| id == order) {
              order_failed = false;

              // update order multiplier if successful
              if (order_mox_points > 0 || order_aga_points > 0 || order_lye_points > 0) {
                  order_multiplier += 0.2;
              }

              let potion = &potions_by_id[&order];

              // Update points for all resources (Mox, Aga, Lye)
              order_mox_points += potion.output_mox;
              order_aga_points += potion.output_aga;
              order_lye_points += potion.output_lye;

              // Remove the potion from the inventory once used
              inventory.remove(index);
          }
      }

      // If no order matched, remove the first potion in the inventory and apply fail points
      if order_failed {
          let id = inventory.remove(0);
          let potion = &potions_by_id[&id];
          order_mox_points += potion.output_mox_fail;
          order_aga_points += potion.output_aga_fail;
          order_lye_points += potion.output_lye_fail;
      }

      mox_points += (order_mox_points as f64 * order_multiplier) as i32;
      aga_points += (order_aga_points as f64 * order_multiplier) as i32;
      lye_points += (order_lye_points as f64 * order_multiplier) as i32;
  }

  // Decrement costs based on remaining potions in the inventory
  for id in inventory {
      let potion = &potions_by_id[&id];
      mox_cost -= potion.input_mox;
      aga_cost -= potion.input_aga;
      lye_cost -= potion.input_lye;
  }

  // Return costs and points for Mox, Aga, and Lye
  (mox_cost, aga_cost, lye_cost, mox_points, aga_points, lye_points)
}

fn simulate_potion_order(
  potions_subset: &[Potion], // Subset of 9 potions
  potions_full_list: &[Potion], // Full list of all potions
  potions_by_id: &HashMap<i32, Potion>,
  print_results: bool,
  max_mox_points: i32,
  max_aga_points: i32,
  max_lye_points: i32,
) -> (i32, f64) {
  let mut mox_cost = 0;
  let mut aga_cost = 0;
  let mut lye_cost = 0;
  let mut mox_points = 0;
  let mut aga_points = 0;
  let mut lye_points = 0;
  let mut num_inventories = 0;

  while !(mox_points > max_mox_points && aga_points > max_aga_points && lye_points > max_lye_points) {
      let (run_mox_cost, run_aga_cost, run_lye_cost, run_mox_points, run_aga_points, run_lye_points) =
          simulate_inventory(potions_subset, potions_full_list, &potions_by_id); // Pass both subset and full list
      mox_cost += run_mox_cost;
      aga_cost += run_aga_cost;
      lye_cost += run_lye_cost;
      mox_points += run_mox_points;
      aga_points += run_aga_points;
      lye_points += run_lye_points;
      num_inventories += 1;
  }

  let efficiency = (mox_points + aga_points + lye_points + (num_inventories * 60)) as f64
      / (mox_cost + aga_cost + lye_cost) as f64;

  if print_results {
      print_order(potions_subset);
      println!(
          "Mox used: {}, Aga used: {}, Lye used: {}",
          mox_cost, aga_cost, lye_cost
      );
      println!(
          "Mox earned: {}, Aga earned: {}, Lye earned: {}",
          mox_points, aga_points, lye_points
      );
      println!(
          "Ratio: {:.2}:{:.2}:1",
          mox_points as f64 / lye_points as f64,
          aga_points as f64 / lye_points as f64
      );
      println!("Efficiency: {:.2}", efficiency);
      println!("Took {} inventories to green log", num_inventories);
  }

  (num_inventories, efficiency)
}

fn main() {
  let potions = vec![
      Potion::new("AAA", 0, 30, 0, 2.0 / 3.0, 0.119048),
      Potion::new("MMM", 30, 0, 0, 2.0 / 3.0, 0.119048),
      Potion::new("LLL", 0, 0, 30, 2.0 / 3.0, 0.119048),
      Potion::new("AAL", 0, 20, 10, 1.0, 0.095238),
      Potion::new("AAM", 10, 20, 0, 1.0, 0.095238),
      Potion::new("MMA", 20, 10, 0, 1.0, 0.095238),
      Potion::new("MML", 20, 0, 10, 1.0, 0.095238),
      Potion::new("ALL", 0, 10, 20, 1.0, 0.095238),
      Potion::new("MLL", 10, 0, 20, 1.0, 0.095238),
      Potion::new("MAL", 10, 10, 10, 2.0, 0.071429),
  ];

  let mut potions_by_id = HashMap::new();
  for potion in &potions {
      for &id in &potion.id_range {
          potions_by_id.insert(id, potion.clone());
      }
  }

  // store the top 10 orders
  let top_orders = Arc::new(Mutex::new(Vec::<(Vec<Potion>, f64, f64)>::new()));


  let count = Arc::new(AtomicUsize::new(0));

  // Start timer
  let start = std::time::Instant::now();

  // Use parallel iterators with rayon
  potions.par_iter().for_each(|potion1| {
      potions.par_iter().for_each(|potion2| {
          potions.par_iter().for_each(|potion3| {
              potions.par_iter().for_each(|potion4| {
                  potions.par_iter().for_each(|potion5| {
                      potions.par_iter().for_each(|potion6| {
                          // potion 7 should always me MAL
                          let potion7 = &potions[9];
                          // potions.par_iter().for_each(|potion7| {
                          potions.par_iter().for_each(|potion8| {
                              potions.par_iter().for_each(|potion9| {

                                  // Increment the local count
                                  let local_count = count.fetch_add(1, Ordering::SeqCst) + 1;

                                  // Only print progress every 10,000 iterations
                                  if local_count % 10_000 == 0 {
                                      let elapsed = start.elapsed();
                                      println!("Count: {} / 100000000", local_count);
                                      println!("Percentage: {:.2}%", (local_count as f64 / 100_000_000.0) * 100.0);
                                      println!("Time elapsed: {:?}", elapsed);
                                      let elapsed_secs = elapsed.as_secs_f64();
                                      let estimated_time = (elapsed_secs / (local_count as f64 / 100_000_000.0)) - elapsed_secs;
                                      println!("Estimated time to completion: {:.2} seconds", estimated_time);
                                      // // print best effeciency
                                      // let best_efficiency = best_efficiency.lock().unwrap();
                                      // println!("Best Efficiency: {:.2}", best_efficiency);
                                      // // print best order
                                      // let best_order = best_order.lock().unwrap();
                                      // print_order(&best_order);

                                      // print top orders
                                      let top_orders = top_orders.lock().unwrap();
                                      for (i, (order, inventories, efficiency)) in top_orders.iter().enumerate() {
                                          println!("Top Order {}: Inventories: {} Efficiency: {:.2}", i + 1, inventories, efficiency);
                                          print_order(&order);
                                      }
                                  }

                                  let current_order = vec![
                                      potion1.clone(),
                                      potion2.clone(),
                                      potion3.clone(),
                                      potion4.clone(),
                                      potion5.clone(),
                                      potion6.clone(),
                                      potion7.clone(),
                                      potion8.clone(),
                                      potion9.clone(),
                                  ];

                                  let mut inventory_list = vec![];
                                  let mut efficiency_list = vec![];
                                  for _ in 0..MINI_SIMS_PER_LAYOUT {
                                      let (number_inventories, efficiency) = simulate_potion_order(&current_order, &potions, &potions_by_id, false, MOX_TARGET_SMALL, AGA_TARGET_SMALL, LYE_TARGET_SMALL);
                                      inventory_list.push(number_inventories as f64);
                                      efficiency_list.push(efficiency as f64);
                                  }

                                  let inventories: f64 = inventory_list.iter().sum::<f64>()
                                      / inventory_list.len() as f64 * 20.0;
                                  let efficiency: f64 = efficiency_list.iter().sum::<f64>()
                                      / efficiency_list.len() as f64;

                                  // Lock the top_orders mutex to update it safely
                                  let mut top_orders_guard = top_orders.lock().unwrap();
                                  let mut max_index = 0;
                                  let mut max_inventories = std::f64::NEG_INFINITY;
                                  let mut max_efficiency = std::f64::NEG_INFINITY;
                              
                                  for (i, (_order, inner_inventories, inner_efficiency)) in top_orders_guard.iter().enumerate() {
                                      if *inner_inventories > max_inventories || (*inner_inventories == max_inventories && *inner_efficiency > max_efficiency) {
                                          max_inventories = *inner_inventories;
                                          max_efficiency = *inner_efficiency;
                                          max_index = i;
                                      }
                                  }
                              
                                  if inventories < max_inventories || (inventories == max_inventories && efficiency < max_efficiency) || top_orders_guard.len() < 10 {
                                      // Do some extra simulations to make sure that this isn't an outlier situation
                                      let mut inventory_list = vec![];
                                      let mut efficiency_list = vec![];
                                      for _ in 0..DEEP_SIMS_PER_LAYOUT {
                                          let (number_inventories, efficiency) = simulate_potion_order(&current_order, &potions, &potions_by_id, false, MOX_TARGET, AGA_TARGET, LYE_TARGET);
                                          inventory_list.push(number_inventories as f64);
                                          efficiency_list.push(efficiency as f64);
                                      }
    
                                      let inventories: f64 = inventory_list.iter().sum::<f64>()
                                          / inventory_list.len() as f64;
                                      let efficiency: f64 = efficiency_list.iter().sum::<f64>()
                                          / efficiency_list.len() as f64;
                                      
                                      if inventories < max_inventories || (inventories == max_inventories && efficiency < max_efficiency) || top_orders_guard.len() < 10 {
                                        top_orders_guard.insert(max_index, (current_order.clone(), inventories, efficiency));
                                        top_orders_guard.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().then_with(|| b.2.partial_cmp(&a.2).unwrap()));
                                        if top_orders_guard.len() > 10 {
                                            top_orders_guard.remove(10);
                                        }
                                      }
                                  }
                              });
                          });
                      // });
                    });
                  });
              });
          });
      });
  });
}