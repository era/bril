use crate::program::Program;
use crate::program::Function;
use crate::program::Instruction;
use crate::program::EffectOps;
use crate::program::Code;

use std::collections::HashMap;


/**
created while watching https://vod.video.cornell.edu/media/1_jc91ke0h
*/

pub fn exec(program: &str) {
    println!("{}", program);
    let program: Program = serde_json::from_str(program).unwrap();
    for func in &program.functions {
        let cfg = form_blocks(func);
        println!("{:?}", cfg.elements);
    }
}

#[derive(Debug)]
struct CFGElement {
    pub label: String,
    pub(crate) successors: Vec<String>,
}

#[derive(Debug)]
struct CFG {
    // the actual cfg with a list of successors
    elements: Vec<CFGElement>,
    // all the code blocks
    blocks: Vec<Vec<Code>>,
    // a map of label -> block index
    map: HashMap<String, usize>,
}

fn is_terminator(instruction: &Instruction) -> bool {
    if let Instruction::Effect { op, .. } = instruction {
        return match op {
            EffectOps::Jump | EffectOps::Branch | EffectOps::Return => true,
            _ => false
        }
    }

    return false
}

fn form_blocks(func: &Function) -> CFG {
    let mut block_map = HashMap::new();
    let mut pos_to_label = Vec::new();
    let mut blocks: Vec<Vec<Code>> = Vec::new();
    let mut curr_block: Vec<Code> = Vec::new();
    for instruction in &func.instrs {
        match instruction {
            Code::Instruction(isnt) => {
                curr_block.push(instruction.clone());
                if is_terminator(isnt) {
                    blocks.push(curr_block);
                    curr_block = Vec::new();
                }
            },
            Code::Label{label} => {
                blocks.push(curr_block);

                curr_block = Vec::new();
                curr_block.push(instruction.clone());
            },
            _ => panic!("not yet handling {:?}", instruction),
        };
    }
    blocks.push(curr_block);

    //TODO yecks, but IDE decided to extract the method like this, will fix it later
    blocks_to_labels_map(&mut block_map, &mut pos_to_label, &blocks);

    let elements = create_cfg_as_list(&pos_to_label, &blocks);

    CFG {
        blocks,
        elements,
        map: block_map,
    }
}

fn blocks_to_labels_map(block_map: &mut HashMap<String, usize>, pos_to_label: &mut Vec<String>, blocks: &Vec<Vec<Code>>) {
    let mut i = 0;
    for block in blocks {
        if let Code::Label { label } = block.get(0).unwrap() {
            block_map.insert(label.to_owned(), i);
            pos_to_label.insert(i, label.to_owned())
        } else {
            let label = format!("b{}", block_map.len());
            block_map.insert(label.to_owned(), i);
            pos_to_label.insert(i, label.to_owned())
        }
        i = i + 1;
    }
}

fn create_cfg_as_list(pos_to_label: &Vec<String>, blocks: &Vec<Vec<Code>>) -> Vec<CFGElement> {
    let mut elements = vec![];

    for (idx, block) in blocks.iter().enumerate() {
        if block.len() == 0 {
            continue;
        }

        let last = match block.get(block.len() - 1) {
            Some(last) => last,
            _ => continue,
        };

        let succ = match last {
            Code::Instruction(inst) => successors(&inst, &idx, &pos_to_label),
            _ => next_block_as_successor(&idx, &pos_to_label)
        };

        let curr_label = &pos_to_label.get(idx);

        let element = CFGElement {
            label: curr_label.unwrap().to_owned(),
            successors: succ,
        };
        elements.push(element);
    }
    elements
}

fn successors(inst: &Instruction, idx: &usize, pos_to_label: &Vec<String> ) -> Vec<String> {
    if let Instruction::Effect{op, labels, .. } = inst {
        match op {
            EffectOps::Jump | EffectOps::Branch => labels.to_vec(),
            EffectOps::Return => vec![],
            _ => next_block_as_successor(idx, pos_to_label),
        }
    } else {
        next_block_as_successor(idx, pos_to_label)
    }
}

fn next_block_as_successor(idx: &usize, pos_to_label: &Vec<String>) -> Vec<String> {
    match pos_to_label.get(idx + 1) {
        Some(s) => vec![s.to_owned()],
        None => vec![],
    }
}