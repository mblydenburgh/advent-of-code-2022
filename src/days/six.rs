use std::{fs, collections::HashSet, hash::Hash};

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn day_6() -> String {
  let signal: String = match fs::read_to_string("day-6-data.txt") {
    Ok(val) => val,
    Err(e) => panic!("Could not parse file: {}", e)
  };
  
  let max_index: usize = usize::try_from(signal.len()).unwrap();
  let mut index: usize = 3;
  while index < max_index {
    let marker: &str = &signal[index-3..=index];
    println!("marker: {}", marker);
    if has_unique_elements(marker.chars().into_iter()) {
      return format!("after {} characters", index + 1)
    }

    index += 1;
  }

  return String::from("No answer found");
}