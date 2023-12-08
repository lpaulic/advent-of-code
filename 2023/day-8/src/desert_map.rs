use num::integer::lcm;
use scanf::sscanf;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DesertMap {
    instructions: Vec<char>,
    node_network: HashMap<String, Destination>,
}

impl DesertMap {
    pub fn parse(map: &str) -> Self {
        let mut map_iter = map.trim().split('\n').filter(|line| !line.is_empty());

        let mut instructions: Vec<char> = Vec::new();
        let direction = &map_iter.next().unwrap();
        direction
            .trim()
            .chars()
            .for_each(|character| instructions.push(character));

        let mut node_network: HashMap<String, Destination> = HashMap::new();
        map_iter.for_each(|line| {
            let mut node: String = String::new();
            let mut left: String = String::new();
            let mut right: String = String::new();

            _ = sscanf!(line.trim(), "{} = ({}, {})", node, left, right);
            node_network.insert(node, Destination { left, right });
        });

        DesertMap {
            instructions,
            node_network,
        }
    }

    pub fn number_of_steps(&self, start_node: &str, end_node: &str) -> u64 {
        let mut current_node: &str = start_node;
        let mut steps_counter = 0_u64;

        'traversal: loop {
            for instruction in &self.instructions {
                let destination = &self.node_network.get(current_node).unwrap();

                current_node = if instruction == &'L' || instruction == &'l' {
                    &destination.left
                } else {
                    // instruction == &'R' || instruction == &'r'
                    &destination.right
                };

                steps_counter += 1;
                if current_node.cmp(end_node).is_eq() {
                    break 'traversal;
                }
            }
        }

        steps_counter
    }

    pub fn number_of_ghost_steps(&self, start_node_ending: char, end_node_ending: char) -> u64 {
        let current_nodes: Vec<&str> = self
            .node_network
            .keys()
            .filter(|key| key.ends_with(start_node_ending))
            .map(|key| key.as_str())
            .collect();
        let mut step_counters: Vec<u64> = Vec::new();

        for mut current_node in current_nodes {
            let mut steps_counter = 0_u64;
            'traversal: loop {
                for instruction in &self.instructions {
                    let destination = &self.node_network.get(current_node).unwrap();

                    current_node = if instruction == &'L' || instruction == &'l' {
                        &destination.left
                    } else {
                        // instruction == &'R' || instruction == &'r'
                        &destination.right
                    };

                    steps_counter += 1;
                    if current_node.ends_with(end_node_ending) {
                        break 'traversal;
                    }
                }
            }

            step_counters.push(steps_counter);
        }

        DesertMap::lcm_extended(step_counters)
    }

    fn lcm_extended(numbers: Vec<u64>) -> u64 {
        let mut numbers_iterator = numbers.iter();
        let first_number = *numbers_iterator.next().unwrap();
        numbers_iterator.fold(first_number, |last_lcm, step_counter| {
            lcm(last_lcm, *step_counter)
        })
    }
}

#[derive(Debug, PartialEq)]
struct Destination {
    left: String,
    right: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asset_desert_map_steps_to_end(expected_number_of_steps: u64, desert_map: &str) {
        assert_eq!(
            expected_number_of_steps,
            DesertMap::parse(desert_map).number_of_steps("AAA", "ZZZ")
        );
    }

    fn asset_desert_map_ghost_steps_to_end(expected_number_of_steps: u64, desert_map: &str) {
        assert_eq!(
            expected_number_of_steps,
            DesertMap::parse(desert_map).number_of_ghost_steps('A', 'Z')
        );
    }

    #[test]
    fn desert_map_instruction_right_left() {
        assert_eq!(vec!['R', 'L'], DesertMap::parse("RL\n").instructions);
        assert_eq!(vec!['L', 'R', 'L'], DesertMap::parse("LRL\n").instructions);
    }

    #[test]
    fn desert_map_node_network_one_element() {
        assert_eq!(
            vec!['L', 'R', 'L'],
            DesertMap::parse("LRL\n\nAAA = (BBB, CCC)").instructions
        );
        assert_eq!(
            vec!['L', 'R', 'L'],
            DesertMap::parse("LRL\n\nAAA = (BBB, CCC)").instructions
        );
        assert_eq!(
            HashMap::from([(
                "AAA".to_string(),
                Destination {
                    left: "BBB".to_string(),
                    right: "CCC".to_string()
                }
            )]),
            DesertMap::parse("LRL\n\nAAA = (BBB, CCC)").node_network
        );
    }

    #[test]
    fn desert_map_node_network_multiple_elements() {
        assert_eq!(
            HashMap::from([
                (
                    "AAA".to_string(),
                    Destination {
                        left: "BBB".to_string(),
                        right: "CCC".to_string()
                    }
                ),
                (
                    "BBB".to_string(),
                    Destination {
                        left: "CCC".to_string(),
                        right: "ZZZ".to_string()
                    }
                )
            ]),
            DesertMap::parse("LRL\n\nAAA = (BBB, CCC)\nBBB = (CCC, ZZZ)").node_network
        );
    }

    #[test]
    fn desert_map_one_node_network_element() {
        asset_desert_map_steps_to_end(1, "RL\n\nAAA = (BBB, ZZZ)\n");
    }

    #[test]
    fn desert_map_two_nodes_network_elements() {
        asset_desert_map_steps_to_end(2, "LR\n\nAAA = (BBB, ZZZ)\nBBB = (AAA, ZZZ)\n");
    }

    #[test]
    fn desert_map_multiple_nodes_network_elements() {
        asset_desert_map_steps_to_end(2, "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)");
    }

    #[test]
    fn desert_map_multiple_nodes_network_elements_repeat_instruction() {
        asset_desert_map_ghost_steps_to_end(
            6,
            "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)\n",
        );
    }

    #[test]
    fn desert_map_multiple_nodes_network_elements_multiple_start() {
        asset_desert_map_ghost_steps_to_end(
            6,
            "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)",
        );
    }
}
