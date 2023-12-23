#[cfg(feature = "performance")]
use ahash::AHashMap as HashMap;
#[cfg(not(feature = "performance"))]
use std::collections::HashMap;
use std::{cell::RefCell, collections::VecDeque, rc::Rc, str::FromStr};

use super::Solution;
use common::Answer;

use super::day_08::get_lcm;

#[derive(Default)]
pub struct Puzzle;

type ModuleRef = Rc<RefCell<Module>>;
type ModuleMap = HashMap<String, ModuleRef>;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let modules = parse_input(input);
        let mut pulses = (0, 0);
        for _ in 0..1000 {
            let (low, high) = run_pulse(&modules);
            pulses = (pulses.0 + low, pulses.1 + high)
        }
        Answer::from(pulses.0 * pulses.1).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let modules = parse_input(input);
        let rx = modules.get("rx").expect("Input needs rx output");
        let rx_collector = modules
            .values()
            .find(|m| m.borrow().str_outputs.contains(&"rx".to_string()))
            .expect("Input needs rx collector");
        let rx_collector_name = rx_collector.borrow().name.clone();

        // Inject recorders where the first level Conjunction resides
        let rx_recorders = modules
            .values()
            .filter(|m| m.borrow().str_outputs.contains(&rx_collector_name))
            .map(|i| {
                let recorder_name = format!("rec_{}", i.borrow().name);
                let recorder = Rc::new(RefCell::new(Module {
                    name: recorder_name.clone(),
                    module_type: ModuleType::Recorder {
                        seen_high: false,
                        seen_low: false,
                    },
                    outputs: vec![],
                    str_outputs: vec![],
                }));
                i.borrow_mut().outputs.push(recorder.clone());
                i.borrow_mut().str_outputs.push(recorder_name);
                recorder
            })
            .collect::<Vec<_>>();

        let mut c: usize = 0;
        let mut cycles: HashMap<String, usize> =
            HashMap::from_iter(rx_recorders.iter().map(|i| (i.borrow().name.clone(), 0)));

        while rx.borrow().module_type != ModuleType::Output(Some(false)) {
            run_pulse(&modules);
            c += 1;

            if c > 10 {
                for rx_recorder in &rx_recorders {
                    let rx_recorder = rx_recorder.borrow();
                    if let ModuleType::Recorder { seen_high, .. } = &rx_recorder.module_type {
                        if *seen_high && cycles.get(&rx_recorder.name).unwrap() == &0 {
                            cycles.insert(rx_recorder.name.clone(), c);
                        }
                    }
                }
            }

            // Reset recorders every cycle
            rx_recorders.iter().for_each(|i| {
                let mut recorder = i.borrow_mut();
                if let ModuleType::Recorder { .. } = recorder.module_type {
                    recorder.module_type = ModuleType::Recorder {
                        seen_high: false,
                        seen_low: false,
                    };
                }
            });

            // Found all cycles
            if cycles.values().all(|v| *v != 0) {
                break;
            }
        }
        let total = cycles.values().fold(0, |prev, current| {
            if prev == 0 {
                *current
            } else {
                get_lcm((prev, *current))
            }
        });
        Answer::from(total).into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(
        &mut self,
        _input: String,
        _request: ui_support::DisplayRequest,
    ) -> Option<ui_support::DisplayResult> {
        None
    }
}

fn run_pulse(map: &ModuleMap) -> (usize, usize) {
    let broadcaster = map.get("broadcaster").unwrap().clone();
    let mut low_count = 0;
    let mut high_count = 0;
    let mut queue = VecDeque::from([(broadcaster.clone(), broadcaster, false)]);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current.2 {
            high_count += 1;
        } else {
            low_count += 1;
        }
        let next = current.0.borrow_mut().apply_pulse(current.1, current.2);
        if let Some(next) = next {
            current.0.borrow().outputs.iter().for_each(|output| {
                queue.push_back((output.clone(), current.0.clone(), next));
            });
        }
    }
    (low_count, high_count)
}

fn parse_input(input: String) -> ModuleMap {
    let mut module_map = HashMap::new();
    input
        .lines()
        .map(|line| line.parse::<Module>().unwrap())
        .for_each(|module| {
            module_map.insert(module.name.clone(), Rc::new(RefCell::new(module)));
        });
    let ref_module_map = module_map.clone();
    let found_outputs: Vec<ModuleRef> = module_map
        .iter()
        .flat_map(|(_, module)| module.borrow_mut().update_outputs(&ref_module_map))
        .collect();
    module_map.extend(found_outputs.into_iter().map(|module| {
        let name = module.borrow().name.clone();
        (name, module)
    }));
    module_map.iter().for_each(|(_, module)| {
        Module::update_inputs(module, &ref_module_map);
    });
    module_map
}

#[derive(Debug, Clone, PartialEq)]
struct Module {
    name: String,
    module_type: ModuleType,
    outputs: Vec<ModuleRef>,
    str_outputs: Vec<String>,
}

impl Module {
    pub fn update_outputs(&mut self, map: &ModuleMap) -> Vec<ModuleRef> {
        let mut new_outputs = vec![];
        self.outputs = self
            .str_outputs
            .iter()
            .map(|output| {
                if let Some(module) = map.get(output) {
                    module.clone()
                } else {
                    let new = Rc::new(RefCell::new(Module {
                        name: output.clone(),
                        module_type: ModuleType::Output(None),
                        outputs: vec![],
                        str_outputs: vec![],
                    }));
                    new_outputs.push(new.clone());
                    new
                }
            })
            .collect::<Vec<_>>();
        new_outputs
    }
    pub fn update_inputs(self_ref: &ModuleRef, map: &ModuleMap) {
        let mod_name = self_ref.borrow().name.clone();
        let filtered_map = map
            .iter()
            .map(|(_, module)| module)
            .filter(|module| !Rc::ptr_eq(self_ref, *module))
            .filter(|module| module.borrow().str_outputs.contains(&mod_name))
            .collect::<Vec<_>>();
        assert_ne!(filtered_map.len(), map.len());
        if let ModuleType::Conjunction(ref mut conjunction_map) = self_ref.borrow_mut().module_type
        {
            filtered_map.into_iter().for_each(|module| {
                conjunction_map.insert(module.borrow().name.clone(), false);
            });
        }
    }

    pub fn apply_pulse(&mut self, src: ModuleRef, high: bool) -> Option<bool> {
        match self.module_type {
            ModuleType::Output(ref mut state) => {
                *state = Some(high);
                None
            }
            ModuleType::Broadcaster => Some(high),
            ModuleType::FlipFlop(ref mut state) => {
                if high {
                    return None;
                }
                *state = !*state;
                Some(*state)
            }
            ModuleType::Conjunction(ref mut states) => {
                *states
                    .get_mut(&src.borrow().name)
                    .unwrap_or_else(|| panic!("Mod not found: {}", src.borrow().name)) = high;
                Some(!states.values().all(|f| *f))
            }
            ModuleType::Recorder {
                ref mut seen_high,
                ref mut seen_low,
            } => {
                if high {
                    *seen_high = true;
                } else {
                    *seen_low = true;
                }
                None
            }
        }
    }
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (t, outputs) = s.split_once(" -> ").unwrap();
        let str_outputs: Vec<String> = if outputs.is_empty() {
            vec![]
        } else {
            outputs
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        };
        let (mod_type, mod_name) = t.split_at(1);
        match mod_type {
            "%" => Ok(Module {
                name: mod_name.to_string(),
                module_type: ModuleType::FlipFlop(false),
                outputs: vec![],
                str_outputs,
            }),
            "&" => Ok(Module {
                name: mod_name.to_string(),
                module_type: ModuleType::Conjunction(HashMap::default()),
                outputs: vec![],
                str_outputs,
            }),
            _ => Ok(Module {
                name: "broadcaster".to_string(),
                module_type: ModuleType::Broadcaster,
                outputs: vec![],
                str_outputs,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Recorder { seen_high: bool, seen_low: bool },
    Output(Option<bool>),
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const TEST_INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT1)),
            Ok(Answer::from(32000000))
        );
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT2)),
            Ok(Answer::from(11687500))
        );
    }

    const TEST_INPUT3: &str = "broadcaster -> no1l, no2l, no3l, no4l
%no1l -> no1src
%no2l -> no2src
%no3l -> no3src
%no4l -> no4src
&no1src -> no1
&no2src -> no2
&no3src -> no3
&no4src -> no4
&no1 -> rxcon
&no2 -> rxcon
&no3 -> rxcon
&no4 -> rxcon
&rxcon -> rx";

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        // this test only verifies that the input is parsable by our code
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT3)),
            Ok(Answer::from("1"))
        )
    }
}
