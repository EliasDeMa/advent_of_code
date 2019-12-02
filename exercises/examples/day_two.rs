use std::fs::{File, self};


fn main() {
    let data = fs::read_to_string("input_two.txt").unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for i in 0..=99 {
        for j in 0..=99 {
            let mut mut_data = data.clone();

            if let Some(elem) = mut_data.get_mut(1) {
                *elem = i;
            }
        
            if let Some(elem) = mut_data.get_mut(2) {
                *elem = j;
            }
            
            let mut program = Program::new(mut_data);

            program.run();

            if program.data.get(0) == Some(&19690720) {
                println!("{}", 100 * i + j);

                break;
            }
        }
    }
    
    // if let Some(elem) = data.get_mut(1) {
    //     *elem = 12;
    // }

    // if let Some(elem) = data.get_mut(2) {
    //     *elem = 2;
    // }
    
    println!("{:?}", data);

    let mut program = Program::new(data);
    program.run();

    println!("{:?}, {:?}", program.data.get(0), program.data.get(4));
    println!("{:?}", program.data);
}

struct Program {
    pub data: Vec<u64>,
    pc: usize,
}

impl Program {
    pub fn new(data: Vec<u64>) -> Self {
        Self {
            data,
            pc: 0
        }
    }

    pub fn run(&mut self) {
        fn get_positions(data: &Vec<u64>, pos: usize) -> (usize, usize, usize) {
            let one = *data.get(pos + 1).unwrap() as usize;
            let two = *data.get(pos + 2).unwrap() as usize;
            let three = *data.get(pos + 3).unwrap() as usize;

            (one, two , three)
        }

        loop {
            match self.data.get(self.pc) {
                Some(1) => {
                    let (one, two, three) = get_positions(&self.data, self.pc);
                    let total = self.data.get(one).unwrap() + self.data.get(two).unwrap();
                    let store = self.data.get_mut(three).unwrap();
                    *store = total;
                },
                Some(2) => {
                    let (one, two, three) = get_positions(&self.data, self.pc);
                    let total = self.data.get(one).unwrap() * self.data.get(two).unwrap();
                    let store = self.data.get_mut(three).unwrap();
                    *store = total;
                }
                Some(99) => break,
                _ => panic!("invalid opcode"),
            }

            self.pc += 4;
        }
    }
}