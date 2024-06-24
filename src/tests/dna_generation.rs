#[cfg(test)]
#[test]
fn dna_genration_test() {
    use crate::generator::Generator;
    let mut generator = Generator::dummy();

    let maximum_generations = generator.config.get_maximum_generations_count();
    println!("{maximum_generations} artworks can be generated.",);

    let count = 100000;

    match generator.generate(count) {
        Err(e) => {
            println!("Error: {e}");
            assert!(false);
        }
        Ok(generated_count) => {
            println!("generated {generated_count} dnas",);
            assert_eq!(count, generated_count);
        }
    }
}
