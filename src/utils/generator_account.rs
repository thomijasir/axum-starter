use rand::Rng;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum AccountGeneratorError {
    GenerationTimeout,
    MaxRetryExceeded,
    ValidationFailed(String),
}

impl fmt::Display for AccountGeneratorError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::GenerationTimeout => write!(f, "Account number generation timed out"),
            Self::MaxRetryExceeded => write!(f, "Maximum retry attempts exceeded"),
            Self::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
        }
    }
}

impl Error for AccountGeneratorError {}

/// Configuration options for account number generation
#[derive(Clone, Debug)]
pub struct AccountGeneratorConfig {
    /// Length of the account number (default: 10)
    pub length: usize,
    /// Maximum attempts before giving up (default: 1000)
    pub max_attempts: usize,
    /// Timeout duration for generation (default: 5 seconds)
    pub timeout: Duration,
    /// Pattern weights as percentages (must sum to 100)
    pub pattern_weights: PatternWeights,
    /// Optional prefix for all account numbers
    pub prefix: Option<String>,
    /// Whether to validate with Luhn algorithm (for checksums)
    pub use_luhn_validation: bool,
}

impl Default for AccountGeneratorConfig {
    fn default() -> Self {
        Self {
            length: 10,
            max_attempts: 1000,
            timeout: Duration::from_secs(5),
            pattern_weights: PatternWeights::default(),
            prefix: None,
            use_luhn_validation: true,
        }
    }
}

/// Percentage weights for different account number patterns
#[derive(Clone, Debug)]
pub struct PatternWeights {
    pub repeating_pairs: u8,
    pub segment_with_repetition: u8,
    pub sequences: u8,
    pub palindrome: u8,
}

impl Default for PatternWeights {
    fn default() -> Self {
        Self {
            repeating_pairs: 30,
            segment_with_repetition: 30,
            sequences: 25,
            palindrome: 15,
        }
    }
}

impl PatternWeights {
    /// Validates that weights sum to 100
    pub fn validate(&self) -> Result<(), AccountGeneratorError> {
        let sum =
            self.repeating_pairs + self.segment_with_repetition + self.sequences + self.palindrome;

        if sum != 100 {
            return Err(AccountGeneratorError::ValidationFailed(format!(
                "Pattern weights must sum to 100, got {}",
                sum
            )));
        }
        Ok(())
    }
}

/// A thread-safe bank account number generator
#[derive(Clone)]
pub struct AccountNumberGenerator {
    config: AccountGeneratorConfig,
    existing_numbers: Arc<HashSet<String>>,
}

impl AccountNumberGenerator {
    /// Creates a new generator with the provided configuration and existing account numbers
    pub fn new(
        config: AccountGeneratorConfig,
        existing_numbers: HashSet<String>,
    ) -> Result<Self, AccountGeneratorError> {
        // Validate configuration
        config.pattern_weights.validate()?;

        // Validate existing account number formats if any exist
        for num in &existing_numbers {
            if let Some(prefix) = &config.prefix {
                if !num.starts_with(prefix) {
                    return Err(AccountGeneratorError::ValidationFailed(format!(
                        "Existing number {} doesn't match required prefix {}",
                        num, prefix
                    )));
                }
            }

            // Check lengths match the configuration
            let expected_len = if let Some(prefix) = &config.prefix {
                config.length + prefix.len()
            } else {
                config.length
            };

            if num.len() != expected_len {
                return Err(AccountGeneratorError::ValidationFailed(format!(
                    "Existing number {} has incorrect length",
                    num
                )));
            }
        }

        Ok(Self {
            config,
            existing_numbers: Arc::new(existing_numbers),
        })
    }

    /// Generates a unique, memorable account number
    pub fn generate(&self) -> Result<String, AccountGeneratorError> {
        let start_time = Instant::now();
        let mut rng = rand::thread_rng();

        for attempt in 0..self.config.max_attempts {
            // Check for timeout
            if start_time.elapsed() > self.config.timeout {
                return Err(AccountGeneratorError::GenerationTimeout);
            }

            // Determine which pattern to use based on weights
            let pattern_roll = rng.gen_range(0..100);
            let mut threshold = 0;

            let account_number = if pattern_roll
                < (threshold + self.config.pattern_weights.repeating_pairs)
            {
                // Pattern 1: Repeating digit pairs
                self.generate_repeating_pairs(&mut rng)
            } else {
                threshold += self.config.pattern_weights.repeating_pairs;
                if pattern_roll < (threshold + self.config.pattern_weights.segment_with_repetition)
                {
                    // Pattern 2: Segment with repetition
                    self.generate_segment_with_repetition(&mut rng)
                } else {
                    threshold += self.config.pattern_weights.segment_with_repetition;
                    if pattern_roll < (threshold + self.config.pattern_weights.sequences) {
                        // Pattern 3: Ascending or descending sequences
                        self.generate_sequence(&mut rng)
                    } else {
                        // Pattern 4: Palindrome-like pattern
                        self.generate_palindrome(&mut rng)
                    }
                }
            };

            // Apply validation if configured
            let final_number = if self.config.use_luhn_validation {
                self.apply_luhn_checksum(&account_number)
            } else {
                account_number
            };

            // Apply prefix if configured
            let final_number = if let Some(prefix) = &self.config.prefix {
                format!("{}{}", prefix, final_number)
            } else {
                final_number
            };

            // Check uniqueness
            if !self.existing_numbers.contains(&final_number) {
                return Ok(final_number);
            }

            // Log a warning if we're making many attempts (would use a proper logger in production)
            if attempt > self.config.max_attempts / 2 {
                eprintln!(
                    "Warning: Made {} attempts to generate unique account number",
                    attempt
                );
            }
        }

        Err(AccountGeneratorError::MaxRetryExceeded)
    }

    // Helper methods for generating different patterns

    fn generate_repeating_pairs(
        &self,
        rng: &mut impl Rng,
    ) -> String {
        let mut number = String::with_capacity(self.config.length);
        let pairs_count = self.config.length / 2;

        for _ in 0..pairs_count {
            let digit = rng.gen_range(0..10).to_string();
            number.push_str(&digit);
            number.push_str(&digit);
        }

        // Handle odd length
        if self.config.length % 2 != 0 {
            number.push_str(&rng.gen_range(0..10).to_string());
        }

        number
    }

    fn generate_segment_with_repetition(
        &self,
        rng: &mut impl Rng,
    ) -> String {
        let mut number = String::with_capacity(self.config.length);

        // First segment (40% of digits as repeated)
        let repeat_count = self.config.length * 4 / 10;
        let repeat_digit = rng.gen_range(0..10).to_string();

        for _ in 0..repeat_count {
            number.push_str(&repeat_digit);
        }

        // Second segment (remaining random digits)
        for _ in repeat_count..self.config.length {
            number.push_str(&rng.gen_range(0..10).to_string());
        }

        number
    }

    fn generate_sequence(
        &self,
        rng: &mut impl Rng,
    ) -> String {
        let mut number = String::with_capacity(self.config.length);

        let ascending = rng.gen_bool(0.5);
        let start_digit = rng.gen_range(0..=9);

        for i in 0..self.config.length {
            let digit = if ascending {
                (start_digit + i) % 10
            } else {
                (start_digit + 10 - i) % 10
            };
            number.push_str(&digit.to_string());
        }

        number
    }

    fn generate_palindrome(
        &self,
        rng: &mut impl Rng,
    ) -> String {
        let mut number = String::with_capacity(self.config.length);

        // Generate first half
        let mid_point = self.config.length / 2;
        for _ in 0..mid_point {
            number.push_str(&rng.gen_range(0..10).to_string());
        }

        // For odd length, add middle digit
        if self.config.length % 2 != 0 {
            number.push_str(&rng.gen_range(0..10).to_string());
        }

        // Mirror for second half
        for i in (0..mid_point).rev() {
            number.push(number.chars().nth(i).unwrap());
        }

        number
    }

    /// Applies Luhn algorithm for checksum validation
    fn apply_luhn_checksum(
        &self,
        number: &str,
    ) -> String {
        if number.len() < 2 {
            return number.to_string();
        }

        // Keep all but the last digit, which we'll replace with a checksum
        let base = &number[0..number.len() - 1];
        let check_digit = self.calculate_luhn_check_digit(base);

        format!("{}{}", base, check_digit)
    }

    fn calculate_luhn_check_digit(
        &self,
        number: &str,
    ) -> char {
        let mut sum = 0;
        let mut alternate = true;

        // Process from right to left
        for c in number.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                if alternate {
                    let doubled = digit * 2;
                    sum += if doubled > 9 { doubled - 9 } else { doubled };
                } else {
                    sum += digit;
                }
                alternate = !alternate;
            }
        }

        // Calculate check digit (what value makes sum % 10 == 0)
        let check_digit = (10 - (sum % 10)) % 10;
        char::from_digit(check_digit, 10).unwrap()
    }
}

// Example of how to use in a production context
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creates_unique_numbers() {
        let config = AccountGeneratorConfig::default();
        let generator = AccountNumberGenerator::new(config, HashSet::new()).unwrap();

        let mut generated = HashSet::new();
        for _ in 0..100 {
            let number = generator.generate().unwrap();
            assert_eq!(number.len(), 10);
            assert!(
                !generated.contains(&number),
                "Generated duplicate: {}",
                number
            );
            generated.insert(number);
        }
    }
}
