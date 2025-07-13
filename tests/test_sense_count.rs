#[test]
fn test_sense_count() {
    use jmdict::{self, Enum};
    
    // Find the entry for 換気 (ventilation) 
    let entry = jmdict::entries()
        .find(|e| e.number == 1212780)
        .expect("Entry 1212780 should exist");
    
    println!("Testing entry {} (換気 - ventilation)", entry.number);
    
    let mut sense_count = 0;
    let mut total_gloss_count = 0;
    
    for sense in entry.senses() {
        sense_count += 1;
        let mut gloss_count = 0;
        let mut languages = Vec::new();
        
        for gloss in sense.glosses() {
            gloss_count += 1;
            total_gloss_count += 1;
            languages.push(gloss.language.code());
            println!("  Sense {}: {} ({})", sense_count, gloss.text, gloss.language.code());
        }
        
        println!("  Sense {} has {} glosses in languages: {:?}", sense_count, gloss_count, languages);
    }
    
    println!("\nTotal senses: {}", sense_count);
    println!("Total glosses: {}", total_gloss_count);
    
    // The entry for 換気 should have only 1 sense (ventilation), not multiple senses
    // for different languages
    assert_eq!(sense_count, 1, "Expected 1 sense, found {}", sense_count);
}