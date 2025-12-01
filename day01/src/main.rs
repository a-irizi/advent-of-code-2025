use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect(r#"failed to open "input.txt" at package root"#);
  let (result, _) =
    input.lines().fold((0u32, 50i32), |(mut zero_count, mut current_dile), instruction| {
      let distance: i32 =
        instruction[1..].parse().expect("distance after direction should be valid number");

      current_dile = if &instruction[..1] == "L" {
        (current_dile - distance) % 100
      } else {
        (current_dile + distance) % 100
      };

      if current_dile == 0 {
        zero_count += 1;
      }

      (zero_count, current_dile)
    });

  println!("result: {result}");
}
