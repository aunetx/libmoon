use libmoon::*;

#[test]
fn import() {
    let mut prog = ProgramFile::new();
    match prog.open("tests/test_programs/import.moon") {
        Ok(_) => println!("File opened !"),
        Err(e) => panic!("Error : {:?}", e),
    };
    match prog.parse() {
        Ok(_) => println!("Program parsed !"),
        Err(e) => panic!("Error : {:?}", e),
    }
    assert_eq!(
        prog.lines[0],
        instructions::Instruction::Var {
            var: "a".to_owned(),
            var_type: instructions::Type::Int
        }
    );
    assert_eq!(
        prog.lines[1],
        instructions::Instruction::Set {
            var: "a".to_owned(),
            value: instructions::Val::Value("10".to_owned())
        }
    );
    assert_eq!(prog.lines[2], instructions::Instruction::Nll);
    assert_eq!(prog.lines[3], instructions::Instruction::Flg);
    assert_eq!(
        prog.lines[4],
        instructions::Instruction::Add {
            var: "a".to_owned(),
            value: instructions::Val::Value("5".to_owned())
        }
    );
    assert_eq!(
        prog.lines[5],
        instructions::Instruction::Gto {
            flag: "hello".to_owned()
        }
    );
}

#[test]
fn basic_run() {
    let mut prog_file = ProgramFile::new();
    match prog_file.open("tests/test_programs/basic.moon") {
        Ok(_) => println!("File opened !"),
        Err(e) => panic!("Error during opening : {:?}", e),
    };
    match prog_file.parse() {
        Ok(_) => println!("Program parsed !"),
        Err(e) => panic!("Error during parsing : {:?}", e),
    }
    let mut prog = Program::from(prog_file);
    match prog.run() {
        Ok(l) => println!("Program finished successfuly at l°{}", l),
        Err(e) => panic!("Error during runtime : {:?}", e),
    };
}

#[test]
fn tables() {
    let mut prog_file = ProgramFile::new();
    match prog_file.open("tests/test_programs/tables.moon") {
        Ok(_) => println!("File opened !"),
        Err(e) => panic!("Error during opening : {:?}", e),
    };
    match prog_file.parse() {
        Ok(_) => println!("Program parsed !"),
        Err(e) => panic!("Error during parsing : {:?}", e),
    }
    let mut prog = Program::from(prog_file);
    match prog.run() {
        Ok(l) => println!("Program finished successfuly at l°{}", l),
        Err(e) => panic!("Error during runtime : {:?}", e),
    };
}
