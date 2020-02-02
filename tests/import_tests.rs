use libmoon::*;

#[test]
fn import() {
    let mut prog = ProgramFile::new();
    match prog.open("tests/test_programs/moontest.moon") {
        Ok(_) => println!("File opened !"),
        Err(e) => panic!("Error : {:?}", e),
    };
    match prog.parse() {
        Ok(_) => println!("Program parsed !"),
        Err(e) => panic!("Error : {:?}", e),
    }
    dbg!(prog.lines);
}
