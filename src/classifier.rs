extern crate stem;

use tokenize::tokenize;
use stem::get;
use std::collections::HashMap;

pub struct NaiveBayesClassifier {
  documents: HashMap<String, HashMap<String, uint>>,
  total_document_count: uint
}

impl NaiveBayesClassifier {
  pub fn new() -> NaiveBayesClassifier {
    NaiveBayesClassifier{ documents: HashMap::new(), total_document_count: 0 }
  }
  
  pub fn train(&mut self, text: String, classification: String) {
    let classification_map = self.documents.find_or_insert(classification, HashMap::new());
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    for stemmed_word in stemmed_and_tokenized.move_iter() {
      classification_map.insert_or_update_with(stemmed_word, 1, |_key, val| *val += 1);
    }
    self.total_document_count += 1;
  }
  
  pub fn train_ngram(&mut self, text: String, classification: String, n: uint) {
    
  }
  
  pub fn guess(&self, text: String) -> String {
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    
    let mut label_probabilities = HashMap::new();
    for (k,v) in self.documents.iter() {
      //Get the probability that the passed-in text is each class
      let mut probability: f32 = 0.0;
      for stemmed_word in stemmed_and_tokenized.iter() {
        if v.contains_key(stemmed_word) {
          probability += (1.0 / v.len() as f32).ln();
        }
      }
      if probability.abs() < 0.0001 {
        label_probabilities.insert(k, 0.0);  
      }
      else {
        label_probabilities.insert(k, (v.len() as f32 * probability.exp() / self.total_document_count as f32));
      }
      
    }
    
    let mut answer_label: String = String::from_str("");
    let mut answer_probability = 0.0;
    for (k,v) in label_probabilities.move_iter() {
      if answer_probability <= v {
        answer_label = k.clone();
        answer_probability = v;
      }
    }
    
    answer_label
  }
}

fn get_tokenized_and_stemmed(text: String) -> Vec<String> {
  let tokenized_text = tokenize(text.as_slice());
  Vec::from_fn(tokenized_text.len(), |idx| stem::get(tokenized_text[idx]).unwrap())
}