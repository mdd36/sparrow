use gladius_core::uci::{parse_input, model::UciCommand};


const CLI_BANNER: &str = 
r#"
 _____ _           _ _           
|  __ \ |         | (_)          
| |  \/ | __ _  __| |_ _   _ ___ 
| | __| |/ _` |/ _` | | | | / __|
| |_\ \ | (_| | (_| | | |_| \__ \
 \____/_|\__,_|\__,_|_|\__,_|___/

      /| ________________
O|===|* >________________>
      \|

© 2023 Matthew Dickson
"#;

fn main() {
  println!("{CLI_BANNER}");
  uci_loop()
}

pub fn uci_loop() {
  loop {
    let mut cmd = String::new();
    std::io::stdin().read_line(&mut cmd).unwrap();
    let result = match parse_input(cmd) {
      Ok(cmd) => cmd,
      Err(str) =>  {
        println!("{str}");
        continue;
      }
    };

    match result {
      UciCommand::UCI => println!("uciok"),
      UciCommand::Position { start_position, moves } => {
        start_position.print_board();
        println!("{:?}", &start_position.metadata);
        println!("{moves:?}");
      }
      UciCommand::Quit => return,
      cmd => println!("{cmd:?}")
    }
  }
}