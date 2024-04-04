#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<DnaNucleotide>);

#[derive(Debug, PartialEq, Eq)]
enum DnaNucleotide {
    A,
    C,
    G,
    T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<RnaNucleotide>);

#[derive(Debug, PartialEq, Eq)]
enum RnaNucleotide {
    A,
    C,
    G,
    U,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .map(|(i, c)| match c {
                'A' => Ok(DnaNucleotide::A),
                'C' => Ok(DnaNucleotide::C),
                'G' => Ok(DnaNucleotide::G),
                'T' => Ok(DnaNucleotide::T),
                _ => Err(i),
            })
            .collect::<Result<Vec<_>, usize>>()
            .map(|nucleotides| Dna(nucleotides))
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .into_iter()
            .map(|nucleotide| match nucleotide {
                DnaNucleotide::A => RnaNucleotide::U,
                DnaNucleotide::C => RnaNucleotide::G,
                DnaNucleotide::G => RnaNucleotide::C,
                DnaNucleotide::T => RnaNucleotide::A,
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .map(|(i, c)| match c {
                'A' => Ok(RnaNucleotide::A),
                'C' => Ok(RnaNucleotide::C),
                'G' => Ok(RnaNucleotide::G),
                'U' => Ok(RnaNucleotide::U),
                _ => Err(i),
            })
            .collect::<Result<Vec<_>, usize>>()
            .map(|nucleotides| Rna(nucleotides))
    }
}
