mod marker;
mod mutation;
use crate::utils;
use marker::Marker;
use mutation::MutationType;
use rand::prelude::*;

pub struct Gene {
    pub num_markers: u16,
    pub markers: Vec<Marker>,
}

///Creates a new gene with $markers
///markers.
#[macro_export]
macro_rules! gene {
    ($markers:expr) => {
        Gene{
            num_markers: $markers as u16,
            markers: (0..($markers as u16 + 1))
                .map(|_| marker::Marker::new())
                .collect(),

        }
    };
}

/// First marker is influence
impl Gene {
    /// Create a new gene
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let gene1 = Gene::new(2);
    /// ```
    pub fn new(num_markers: u16) -> Gene {
        if num_markers < 1 {
            panic!("Markers needs to be more than 0");
        }
        Gene {
            num_markers: num_markers,
            markers: (0..num_markers + 1)
                .map(|_| marker::Marker::new())
                .collect(),
        }
    }
    /// Get sum of genes
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let gene1 = Gene::new(2);
    ///
    /// let sum = gene1.get_sum();
    /// ```
    pub fn get_sum(&self) -> f32 {
        self.markers.iter().map(|m| m.value).sum()
    }
    /// Compare is two gene is equal
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let gene1 = Gene::new(2);
    /// let gene2 = Gene::new(2);
    ///
    /// let is_equal = Gene::is_equal(&gene1, &gene2);
    /// ```
    pub fn is_equal(left_gene: &Gene, right_gene: &Gene) -> bool {
        if left_gene.markers.len() != right_gene.markers.len() {
            return false;
        }
        let left_markers = left_gene.get_markers();
        let right_markers = right_gene.get_markers();
        let mut result = true;

        for (i, marker) in left_markers.iter().enumerate() {
            if *marker != right_markers[i] {
                result = false;
            }
        }

        return result;
    }
    /// Get influence for this gene
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let gene1 = Gene::new(2);
    ///
    /// let influence = gene1.get_influence();
    /// ```
    pub fn get_influence(&self) -> f32 {
        self.markers[0].value
    }
    /// Get markers at position for the gene, ignore influence
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let gene1 = Gene::new(2);
    ///
    /// let marker = gene1.get_marker(0);
    /// ```
    pub fn get_marker(&self, position: usize) -> Option<f32> {
        // position + 1 is used instead of position since first element is influence
        match self.markers.get(position + 1) {
            Some(m) => Some(m.value),
            None => None,
        }
    }
    /// Get all markers for the gene, ignore influence
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let gene1 = Gene::new(2);
    ///
    /// let markers = gene1.get_markers();
    /// ```
    pub fn get_markers(&self) -> Vec<f32> {
        self.markers
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != 0) // Filter out the first index
            .map(|(_, m)| m.value)
            .collect()
    }
    /// Convert gene to string
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let gene1 = Gene::new(2);
    ///
    /// let gene_str = gene1.to_string();
    /// ```
    pub fn to_string(&self) -> String {
        self.markers
            .iter()
            .map(|m| String::from(*m))
            .collect::<String>()
    }
    fn set_marker(&mut self, target: usize, value: f32) {
        self.markers.get_mut(target).unwrap().value = value;
    }
    /// Compare gene for equality
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let gene1 = Gene::new(2);
    /// let gene2 = Gene::new(2);
    ///
    /// let is_same = Gene::compare(gene1, gene2);
    /// ```
    pub fn compare(left_gene: Gene, right_gene: Gene) -> bool {
        left_gene.to_string() == right_gene.to_string()
    }
    /// Mutate the gene, with 5 different types of mutation
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let mut gene1 = Gene::new(2);
    ///
    /// gene1.mutate();
    /// ```
    pub fn mutate(&mut self) {
        let mutation_type = MutationType::get_mutation_type();
        let target = thread_rng().gen_range(0, self.markers.len());
        match mutation_type {
            MutationType::DELETE => self.set_marker(target, 0 as f32),
            MutationType::DUPLICATION => {
                // exclude the influence marker, allow noop
                let dup_target = thread_rng().gen_range(1, self.markers.len());
                self.set_marker(dup_target, self.markers[target].value);
            }
            MutationType::NEW => {
                let new_marker = Marker::new();
                self.set_marker(target, new_marker.value);
            }
            MutationType::REVERSAL => {
                // exclude the influence marker, allow noop
                let swap_target = thread_rng().gen_range(1, self.markers.len());
                let swap_value = self.markers[swap_target].value;
                self.set_marker(swap_target, self.markers[target].value);
                self.set_marker(target, swap_value);
            }
            MutationType::SHIFT => {
                self.markers.shuffle(&mut thread_rng());
            }
        }
    }
    /// Zero this gene
    ///
    /// # Examples
    ///
    /// ```
    /// use genome::Gene;
    ///
    /// let mut gene1 = Gene::new(2);
    ///
    /// gene1.zero();
    /// ```
    pub fn zero(&mut self) {
        for i in 0..self.markers.len() {
            self.set_marker(i, 0 as f32);
        }
    }
}

/// Convert gene to string
///
/// # Examples
///
/// ```
/// use genome::Gene;
///
/// let gene1 = Gene::new(2);
///
/// let gene_str = String::from(gene1);
/// ```
impl std::convert::From<Gene> for String {
    fn from(gene: Gene) -> String {
        gene.markers
            .iter()
            .map(|m| String::from(*m))
            .collect::<String>()
    }
}

/// Convert string to Gene
///
/// # Examples
///
/// ```
/// use genome::Gene;
///
/// let gene1 = Gene::new(2);
///
/// let gene_str = String::from(gene1);
///
/// let gene_copy = Gene::from(gene_str);
/// ```
impl std::convert::From<String> for Gene {
    fn from(gene: String) -> Gene {
        let markers = utils::partition_string(&gene, 8);

        Gene {
            num_markers: markers.len() as u16,
            markers: markers
                .iter()
                .map(|m| Marker::from(String::from(*m)))
                .collect(),
        }
    }
}

impl PartialEq for Gene {
    fn eq(&self, other: &Self) -> bool {
        Gene::is_equal(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_be_converted_and_back() {
        let gene = Gene::new(2);
        let original_influence = gene.get_influence();
        let string_value: String = gene.into();
        let restored_gene: Gene = string_value.clone().into();
        let restored_influence = restored_gene.get_influence();
        let restored_string_value: String = restored_gene.into();
        assert_eq!(restored_string_value, string_value);
        assert_eq!(original_influence, restored_influence);
    }
    #[test]
    fn marker_size_match() {
        let gene = Gene::new(2);
        let markers = gene.get_markers();
        assert_eq!(gene.num_markers, markers.len() as u16);
    }
    #[test]
    fn markers_should_include_influence() {
        let gene = Gene::new(2);
        assert_eq!(gene.markers.len(), 3);
    }
    #[test]
    fn get_marker_correctly() {
        let gene = Gene::new(2);
        assert_eq!(gene.get_marker(0).unwrap(), gene.markers[1].value);
    }
    #[test]
    fn get_marker_return_none() {
        let gene = Gene::new(2);
        match gene.get_marker(3) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
    #[test]
    fn should_return_true_on_equal() {
        let gene = Gene::new(2);
        let string_value: String = gene.into();
        let original_gene: Gene = string_value.clone().into();
        let restored_gene: Gene = string_value.clone().into();

        assert!(Gene::is_equal(&original_gene, &restored_gene));
    }
    #[test]
    fn should_be_partial_eq(){
        let gene = gene!(2);
        let str_val: String = gene.into();
        let original_gene: Gene = str_val.clone().into();
        let restored_gene: Gene = str_val.clone().into();

        assert!(original_gene == restored_gene)
    }
    #[test]
    fn should_be_partial_ne(){
        let gene1 = gene!(2);
        let gene2 = gene!(3);

        assert!(gene1 != gene2)
    }
    #[test]
    fn should_set_marker() {
        let mut gene = Gene::new(1);
        gene.set_marker(0, 0 as f32);
        gene.set_marker(1, 0 as f32);
        assert_eq!(gene.to_string(), "0000000000000000");
    }
    #[test]
    fn should_zero_gene() {
        let mut gene = Gene::new(1);
        gene.zero();
        assert_eq!(gene.to_string(), "0000000000000000");
    }
    #[test]
    fn macro_test(){
        let gene = gene!(10);
        assert_eq!(gene.get_markers().len(), 10);
    }
}
