pub fn reverse(input: &str) -> String {
 //   let reversed_string = input.to_string();
 //   reversed_string.to
 //   return reversed_string
    let characters = input.chars();
 //   input.push_str('a');
   // let reversed_string = characters.toString();
    let last_character: String = characters.next_back();
    let reversed_string = last_character.toString();
}
