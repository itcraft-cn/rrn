mod param;

fn main() {
    let rs_param = param::args2param();
    if let Ok(param) = rs_param {
        println!("Param parsed successfully");
        println!("    from : {}", param.get_from_pattern());
        println!("      to : {}", param.get_to_pattern());
        println!("  target : {:?}", param.get_target_mode());
        println!("    exec : {:?}", param.get_exec_mode());
    } else {
        match rs_param.err().unwrap() {
            param::ParseArgFailure::Help => (),
            param::ParseArgFailure::Invalid => eprintln!("Params error"),
        }
    }
}
