use std::fs;

struct Opcode {
  code: String,
  byte: u8,
}

struct Label {
  name: String,
  position: u32,
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

fn push_label(label: &str, position: usize, labels: &mut Vec<Label>) {

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

  let mut labels: Vec<Label> = Vec::new();
  println!("In file {}", filename);

  let contents = fs::read_to_string(filename)
      .expect("Something went wrong reading the file");
  let mut byte_counter: usize = 1;
  for line in contents.lines() {
    let mut split = line.split_whitespace().collect::<Vec<&str>>();
    if !is_an_opcode(split[0], &opcodes) {
      println!("{:?}", split[0]);
      push_label(split[0], byte_counter, &mut labels);
    }
    byte_counter += split.len();
    println!("{:?}", byte_counter);
  }
}
