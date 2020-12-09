use smol_str::SmolStr;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type MutRc<T> = Rc<RefCell<T>>;

#[derive(Default, Eq)]
struct Bag {
    name: SmolStr,
    contained_by: HashMap<SmolStr, usize>,
    contains: HashMap<SmolStr, usize>,
}

impl Bag {
    pub fn new(name: SmolStr) -> MutRc<Bag> {
        Rc::new(RefCell::new(Bag {
            name,
            ..Default::default()
        }))
    }

    fn get_parent_bags(&self, all_bags: &HashMap<SmolStr, MutRc<Bag>>) -> usize {
        self.get_parent_bags_(all_bags, &mut HashSet::with_capacity(600))
    }

    fn get_parent_bags_(
        &self,
        all_bags: &HashMap<SmolStr, MutRc<Bag>>,
        known: &mut HashSet<SmolStr>,
    ) -> usize {
        let mut count = 0;
        for bag in self.contained_by.keys() {
            if known.contains(bag) {
                continue;
            }
            // +1 to account for the bag itself
            count += all_bags
                .get(bag)
                .unwrap()
                .borrow()
                .get_parent_bags_(all_bags, known)
                + 1;
            known.insert(bag.clone());
        }
        count
    }

    fn get_child_bags(&self, all_bags: &HashMap<SmolStr, MutRc<Bag>>) -> usize {
        let mut total = 0;
        for (bag, count) in &self.contains {
            let bag_children = all_bags.get(bag).unwrap().borrow().get_child_bags(all_bags);
            total += count + count * bag_children;
        }
        total
    }
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn a7_1() {
    let bags = construct_tree();
    let target = Rc::clone(bags.get("shiny gold").unwrap());
    println!("{}", target.borrow().get_parent_bags(&bags))
}

pub fn a7_2() {
    let bags = construct_tree();
    let target = Rc::clone(bags.get("shiny gold").unwrap());
    println!("{}", target.borrow().get_child_bags(&bags))
}

fn construct_tree<'t>() -> HashMap<SmolStr, MutRc<Bag>> {
    let mut bags_map = HashMap::with_capacity(600);

    for rule in read_to_string("inputs/input-7")
        .expect("Failed to read seats")
        .lines()
    {
        let rule = &rule[0..(rule.len() - 1)]; // Trim dot
        let mut split = rule.split(" bags contain ");

        let container = SmolStr::new(split.next().unwrap());
        let container_bag = Rc::clone(
            bags_map
                .entry(container.clone())
                .or_insert_with(|| Bag::new(container.clone())),
        );

        let bags = split.next().unwrap();
        if bags == "no other bags" {
            continue;
        }

        for bag in bags.split(", ") {
            let count = bag.chars().next().unwrap().to_digit(10).unwrap();
            let bag = &bag[2..bag.len()]; // Trim count
            let bag_name = SmolStr::new(bag.split(" bag").next().unwrap());

            let bag = bags_map
                .entry(bag_name.clone())
                .or_insert_with(|| Bag::new(bag_name.clone()));
            bag.borrow_mut()
                .contained_by
                .insert(container.clone(), count as usize);
            container_bag
                .borrow_mut()
                .contains
                .insert(bag_name, count as usize);
        }
    }

    bags_map
}
