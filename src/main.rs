use std::fs;
use byteorder::{LittleEndian, BigEndian, WriteBytesExt};
use std::path::Path;
use std::io::Write;

struct Opcode {
  code: String,
  byte: u8,
}

struct Label {
  name: String,
  position: usize,
}

struct Variable {
  name: String,
  index: usize,
}

fn is_an_opcode(opcode: &str, opcodes: &[Opcode]) -> bool {
  for op in opcodes {
    if op.code == opcode {
      return true
    }
  }
  return false
}

fn is_a_label(label: &str, labels: &Vec<Label>) -> bool {
  for l in labels {
    if l.name == label {
      return true;
    }
  }
  return false;
}

fn is_a_variable(var: &str, vars: &Vec<Variable>) -> bool {
  for v in vars {
    if v.name == var {
      return true;
    }
  }
  return false;
}

fn push_variable(name: &str, variables: &mut Vec<Variable>, index: usize) {
  variables.push(
    Variable {
      name: String::from(name),
      index,
  });
}

fn push_label(name: &str, position: usize, labels: &mut Vec<Label>) {
  labels.push(
    Label {
      name: String::from(name),
      position,
    }
  );
}


fn main() {
  let filename = "./hello.asm";
  let opcodes: [Opcode; 20] = [
    Opcode {
      code: "nop".to_string(),
      byte: 0x01,
    },
    Opcode {
      code: "iadd".to_string(),
      byte: 0x02,
    },
    Opcode {
      code: "isub".to_string(),
      byte: 0x05,
    },
    Opcode {
      code: "iand".to_string(),
      byte: 0x08,
    },
    Opcode {
      code: "ior".to_string(),
      byte: 0x0B,
    },
    Opcode {
      code: "dup".to_string(),
      byte: 0x0E,
    },
    Opcode {
      code: "pop".to_string(),
      byte: 0x10,
    },
    Opcode {
      code: "swap".to_string(),
      byte: 0x13,
    },
    Opcode {
      code: "bipush".to_string(),
      byte: 0x19,
    },
    Opcode {
      code: "iload".to_string(),
      byte: 0x1C,
    },
    Opcode {
        code: "istore".to_string(),
        byte: 0x22,
    },
    Opcode {
      code: "wide".to_string(),
      byte: 0x28,
    },
    Opcode {
      code: "ldc_w".to_string(),
      byte: 0x32,
    },
    Opcode {
      code: "iinc".to_string(),
      byte: 0x36,
    },
    Opcode {
      code: "goto".to_string(),
      byte: 0x3C,
    },
    Opcode {
      code: "iflt".to_string(),
      byte: 0x43,
    },
    Opcode {
      code: "ifeq".to_string(),
      byte: 0x47,
    },
    Opcode {
      code: "if_icmpeq".to_string(),
      byte: 0x4B,
    },
    Opcode {
      code: "invokevirtual".to_string(),
      byte: 0x55,
    },
    Opcode {
      code: "ireturn".to_string(),
      byte: 0x6B,
    },
  ];

  let mut final_program = vec![];
  let mut labels: Vec<Label> = Vec::new();
  let mut variables: Vec<Variable> = Vec::new();

  let contents = fs::read_to_string(filename)
      .expect("Something went wrong reading the file");
  let mut byte_counter: usize = 1;
  //  Getting labels and their byte positions
  for line in contents.lines() {
    let split = line.split_whitespace().collect::<Vec<&str>>();

    //  Stores label in its table
    if !is_an_opcode(split[0], &opcodes) {
      push_label(split[0], byte_counter, &mut labels);
    }
    byte_counter += split.len();
  }

  // Get all variables
  let mut vars_num = 0;
  for line in contents.lines() {
    let split = line.split_whitespace().collect::<Vec<&str>>();
    if split.len() > 1 && !split[1].parse::<f64>().is_ok() && !is_an_opcode(split[1], &opcodes) && !is_a_label(split[1], &labels) && !is_a_variable(split[1], &variables) {
      push_variable(split[1], &mut variables, vars_num);
      vars_num += 1;
    } else if  split.len() > 2 && !split[2].parse::<f64>().is_ok() && !is_an_opcode(split[2], &opcodes) && !is_a_label(split[2], &labels) && !is_a_variable(split[2], &variables){
      push_variable(split[2], &mut variables, vars_num);
      vars_num += 1;
    }
  }
  // Writing program size
  let program_size: u32 = 20 + byte_counter as u32;
  final_program.write_u32::<LittleEndian>(program_size).unwrap();
  
  // Initializing registers
  let registers: [u32; 5] = [
    0x7300, // INIT
    0x0006, // CPP
    0x1001, // LV
    0x0400, // PC
    0x1001 + vars_num as u32 // SP
  ];

  for reg in registers.iter() {
    final_program.write_u32::<LittleEndian>(*reg).unwrap();
  }

  while final_program.len() < 1025 {
    final_program.write_u32::<LittleEndian>(0).unwrap();
  }
  // Second step(Write jump labels as Big Indian to fix bug)
  byte_counter = 0;
  for line in contents.lines() {
    let split = line.split_whitespace().collect::<Vec<&str>>();
    if is_an_opcode(split[0], &opcodes) {
      for op in &opcodes {
        if op.code == split[0] {
          final_program.write_u32::<LittleEndian>(op.byte as u32).unwrap();
        }
      }
    }
    if split.len() > 1 && is_an_opcode(split[1], &opcodes) {
      for op in &opcodes {
        if op.code == split[1] {
          final_program.write_u32::<LittleEndian>(op.byte as u32).unwrap();
        }
      }
    }
    if split.len() > 1 && is_a_variable(split[1], &variables) {
      for var in &variables {
        if var.name == split[1] {
          final_program.write_u32::<LittleEndian>(var.index as u32).unwrap();
        }
      }
    }
    if split.len() > 1 && is_a_label(split[1], &labels) {
      for l in &labels {
        if l.name == split[1] {
          let jump_to: i32 = l.position as i32 + 1 - byte_counter as i32;
          println!("{:?}", jump_to);
          final_program.write_i32::<BigEndian>(jump_to).unwrap();
        }
      }
    }
    if split.len() > 1 && split[1].parse::<u8>().is_ok() {
      final_program.write_u32::<LittleEndian>(split[1].parse::<u32>().unwrap()).unwrap();
    }
    if split.len() > 2 && split[2].parse::<u32>().is_ok() {
      final_program.write_u32::<LittleEndian>(split[2].parse::<u32>().unwrap()).unwrap();
    }
    if split.len() > 2 && is_a_label(split[2], &labels) {
      for l in &labels {
        if l.name == split[1] {
          let jump_to: i32 = byte_counter as i32 - l.position as i32;
          final_program.write_i32::<BigEndian>(jump_to).unwrap();
        }
      }
    }
    byte_counter += split.len();
  }
  let path = Path::new("result");
  let display = path.display();

  let mut file = match fs::File::create(&path) {
      Err(why) => panic!("couldn't create {}: {}", display, why),
      Ok(file) => file,
  };
  //  Write result file
  match file.write_all(&final_program) {
    Err(why) => panic!("couldn't write to {}: {}", display, why),
    Ok(_) => println!("successfully wrote to {}", display),
  }

}
