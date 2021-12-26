use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct Solution {}

impl Solution {
  /*
    - refer to the comments in question 126 to learn how
      breadth first search works
    - comparing to question 127 this one is a bit complex as
      you need to construct all possible paths as well
    - the key to be able to re-construct all possible paths
      is to remember the parent of the vertex you are visiting
      - we use a HashMap for this purpose
    - if the target (end_word) is found while visiting a vertex
      at a given level, you will continue to visit other vertices
      at the same level but will not go any further
  */

  pub fn ladder_length(
    begin_word: String,
    end_word: String,
    word_list: Vec<String>,
  ) -> Vec<Vec<String>> {
    if begin_word == end_word {
      return vec![vec![begin_word]];
    };
    if !word_list.contains(&end_word) {
      return vec![];
    }
    /* end_word must exist in the word_list */
    let word_length = begin_word.len();
    /*
      - converting String to char vec so it is easier
        to compare the value to the combinations created
    */
    let start = Self::to_char_vec(&begin_word);
    let mut word_list = Self::to_char_hash_set(&word_list);
    let mut queue = VecDeque::from(vec![start]);
    let mut level = 0;
    let target = Self::to_char_vec(&end_word);

    /*
      - all valid paths that lead to the
        end_word
    */
    let mut paths: Vec<Vec<String>> = vec![];
    /*
      - used to remember the parent of a vertex
    */
    let mut parents: HashMap<String, String> = HashMap::new();

    /*
      - this will be set to true when the end_word
        is found
      - once the shortest paths are found at a level
        there is no reason to go to next level
      - however, you still need to finish visiting
        the vertices at the same level you found the
        shortest path to collect all possible paths
        - this is key difference between 126 and 127
    */
    let mut visit_next_level = true;

    while queue.len() > 0 && visit_next_level {
      level = level + 1;
      let count = queue.len();
      /* for each vertex at this level */
      for _ in 1..=count {
        /*
          - this is used to help break
            the outer loop (loop through
            different positions in a word)
            when a combination found matching
            the end_word
        */
        let mut path_found = false;
        if let Some(mut word) = queue.pop_front() {
          let parent = Self::to_string(&word);
          for pos in 0..word_length {
            let r = word[pos];
            for c in b'a'..=b'z' {
              let ch = c as char;
              if ch == r {
                continue;
              }
              word[pos] = ch;
              /*
                - don't need to go next level as the
                  shortest path has been found
              */
              if word == target {
                let mut new_path = Self::construct_path(&parent, &parents);
                new_path.push(parent.clone());
                new_path.push(end_word.clone());
                paths.push(new_path);
                path_found = true;
                visit_next_level = false;
                /* no point to try the next combination */
                break;
              }

              if word_list.contains(&word) {
                /* record who is your parent */
                parents.insert(Self::to_string(&word), parent.clone());
                word_list.remove(&word);
                queue.push_back(word.clone());
              }
            }
            word[pos] = r;

            /* don't need to look at the next pos  */
            if path_found {
              break;
            }
          }
        } else {
          continue;
        }
      }
    }

    paths
  }

  pub fn to_char_vec(input: &String) -> Vec<char> {
    input.to_ascii_lowercase().chars().collect()
  }

  pub fn to_string(input: &Vec<char>) -> String {
    String::from_iter(input.into_iter())
  }

  pub fn to_char_hash_set(list: &Vec<String>) -> HashSet<Vec<char>> {
    let mut hash_set = HashSet::new();
    for word in list.into_iter() {
      hash_set.insert(Self::to_char_vec(word));
    }
    hash_set
  }

  pub fn construct_path(node: &String, parents: &HashMap<String, String>) -> Vec<String> {
    if !parents.contains_key(node) {
      return vec![];
    }

    let mut path = vec![parents[node].clone()];

    let mut key = &parents[node];

    while parents.contains_key(key) {
      key = &parents[key];
      path.insert(0, key.clone());
    }
    path
  }

  pub fn text_fixture_1() -> Vec<String> {
    vec![
      "hot".to_string(),
      "dot".to_string(),
      "dog".to_string(),
      "lot".to_string(),
      "log".to_string(),
      "cog".to_string(),
    ]
  }

  pub fn text_fixture_2() -> Vec<String> {
    vec![
      "hot".to_string(),
      "dot".to_string(),
      "dog".to_string(),
      "lot".to_string(),
      "log".to_string(),
    ]
  }

  pub fn text_fixture_3() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(String::from("hot"), String::from("hit"));
    map.insert(String::from("dot"), String::from("hot"));
    map.insert(String::from("dog"), String::from("dot"));
    map
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let target = vec![
      vec!["hit", "hot", "dot", "dog", "cog"],
      vec!["hit", "hot", "lot", "log", "cog"],
    ];
    let word_list = Solution::text_fixture_1();
    let result = Solution::ladder_length(String::from("hit"), String::from("cog"), word_list);
    assert_eq!(result, target);
  }

  #[test]
  fn sample_2() {
    let word_list = Solution::text_fixture_2();
    let result = Solution::ladder_length(String::from("hit"), String::from("cog"), word_list);
    assert_eq!(result.len(), 0);
  }

  #[test]
  fn construct_path() {
    let target = ["hit", "hot", "dot"];
    let result = Solution::construct_path(&String::from("dog"), &Solution::text_fixture_3());
    assert_eq!(result, target);
  }
}
