use codebook::queries::LanguageType;
mod utils;

#[test]
fn test_haskell_simple() {
    utils::init_logging();
    let processor = utils::get_processor();
    let sample_text = r#"
        func myArrg = do
           calculatr <- makeCowculator
           sum calculatr [ numberr1, argument2, myArrg ]
    "#;
    let expected = vec!["Arrg", "Cowculator", "calculatr", "numberr"];
    let binding = processor
        .spell_check(sample_text, Some(LanguageType::Haskell), None)
        .to_vec();
    let mut misspelled = binding
        .iter()
        .map(|r| r.word.as_str())
        .collect::<Vec<&str>>();
    misspelled.sort();
    assert_eq!(misspelled, expected);
}

#[test]
fn test_haskell_string() {
    utils::init_logging();
    let processor = utils::get_processor();
    let sample_text = r#"
        let str =  "herlo, world"
        in str
    "#;
    let expected = vec!["herlo"];
    let binding = processor
        .spell_check(sample_text, Some(LanguageType::Haskell), None)
        .to_vec();
    let mut misspelled = binding
        .iter()
        .map(|r| r.word.as_str())
        .collect::<Vec<&str>>();
    misspelled.sort();
    assert_eq!(misspelled, expected);
}

#[test]
fn test_haskell_module() {
    utils::init_logging();
    let processor = utils::get_processor();
    let sample_text = r#"
        import Data.Functoin as Func
        import Data.Function qualified as D.Funcc
    "#;
    let expected = vec!["Funcc", "Functoin"];
    let binding = processor
        .spell_check(sample_text, Some(LanguageType::Haskell), None)
        .to_vec();
    let mut misspelled = binding
        .iter()
        .map(|r| r.word.as_str())
        .collect::<Vec<&str>>();
    misspelled.sort();
    assert_eq!(misspelled, expected);
}

#[test]
fn test_haskell_types() {
    utils::init_logging();
    let processor = utils::get_processor();
    let sample_text = r#"
        func :: forall badd . (MyTypeeClass var) => varr -> Intt
        func = varToInt
    "#;
    let expected = vec!["Intt", "Typee", "badd", "varr"];
    let binding = processor
        .spell_check(sample_text, Some(LanguageType::Haskell), None)
        .to_vec();
    let mut misspelled = binding
        .iter()
        .map(|r| r.word.as_str())
        .collect::<Vec<&str>>();
    misspelled.sort();
    assert_eq!(misspelled, expected);
}
