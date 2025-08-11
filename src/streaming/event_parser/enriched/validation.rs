use serde::{Deserialize, Serialize};

/// Validation results for an enriched trade event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationData {
    /// Overall validation result
    pub is_valid: bool,
    
    /// Confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    
    /// Individual validation checks
    pub checks: ValidationChecks,
    
    /// Any validation errors found
    pub errors: Vec<ValidationError>,
    
    /// Warnings that don't invalidate the trade
    pub warnings: Vec<ValidationWarning>,
    
    /// Risk assessment
    pub risk_assessment: RiskAssessment,
}

/// Individual validation checks performed
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationChecks {
    /// Price reasonableness check
    pub price_check: ValidationResult,
    
    /// Amount reasonableness check
    pub amount_check: ValidationResult,
    
    /// Platform data integrity check
    pub platform_data_check: ValidationResult,
    
    /// Timing anomaly check
    pub timing_check: ValidationResult,
    
    /// Fee calculation check
    pub fee_check: ValidationResult,
    
    /// Duplicate detection check
    pub duplicate_check: ValidationResult,
}

/// Result of an individual validation check
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether this check passed
    pub passed: bool,
    
    /// Score contribution (0.0 to 1.0)
    pub score: f64,
    
    /// Details about the check
    pub details: Option<String>,
    
    /// Time taken for this check (microseconds)
    pub check_time_us: u64,
}

/// Validation error types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValidationError {
    InvalidPrice { 
        price: f64, 
        reason: String 
    },
    InvalidAmounts { 
        sol_amount: f64, 
        token_amount: f64, 
        reason: String 
    },
    MissingPlatformData { 
        platform: String, 
        missing_fields: Vec<String> 
    },
    TimingAnomaly { 
        delay_ms: u64, 
        threshold_ms: u64 
    },
    FeeCalculationError { 
        expected_fee: f64, 
        actual_fee: f64 
    },
    DuplicateTransaction { 
        signature: String, 
        previous_timestamp: u64 
    },
}

/// Validation warnings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValidationWarning {
    HighPriceImpact { 
        impact_percentage: f64 
    },
    LowLiquidity { 
        liquidity_value: Option<f64> 
    },
    HighProcessingDelay { 
        delay_ms: u64 
    },
    UnusualTradingPattern { 
        pattern_type: String, 
        confidence: f64 
    },
    IncompleteData { 
        missing_fields: Vec<String> 
    },
}

/// Risk assessment for the trade
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk level
    pub risk_level: RiskLevel,
    
    /// Risk score (0.0 = no risk, 1.0 = maximum risk)
    pub risk_score: f64,
    
    /// Individual risk factors
    pub risk_factors: Vec<RiskFactor>,
    
    /// Recommended actions
    pub recommendations: Vec<String>,
}

/// Risk level categories
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Individual risk factors
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Type of risk
    pub risk_type: RiskType,
    
    /// Risk score for this factor (0.0 to 1.0)
    pub score: f64,
    
    /// Description of the risk
    pub description: String,
    
    /// Impact if this risk materializes
    pub impact: RiskImpact,
}

/// Types of risks
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RiskType {
    PriceManipulation,
    FrontRunning,
    LowLiquidity,
    HighSlippage,
    UnknownToken,
    SuspiciousPattern,
    TechnicalIssue,
}

/// Risk impact levels
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskImpact {
    Low,
    Medium,
    High,
    Critical,
}

impl ValidationData {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            confidence_score: 1.0,
            checks: ValidationChecks::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            risk_assessment: RiskAssessment::new(),
        }
    }
    
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        self.is_valid = false;
        self.recalculate_confidence();
    }
    
    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
        self.recalculate_confidence();
    }
    
    pub fn set_check_result(&mut self, check_type: &str, result: ValidationResult) {
        match check_type {
            "price" => self.checks.price_check = result,
            "amount" => self.checks.amount_check = result,
            "platform_data" => self.checks.platform_data_check = result,
            "timing" => self.checks.timing_check = result,
            "fee" => self.checks.fee_check = result,
            "duplicate" => self.checks.duplicate_check = result,
            _ => {}
        }
        self.recalculate_confidence();
    }
    
    fn recalculate_confidence(&mut self) {
        let checks = &self.checks;
        let total_score = checks.price_check.score
            + checks.amount_check.score
            + checks.platform_data_check.score
            + checks.timing_check.score
            + checks.fee_check.score
            + checks.duplicate_check.score;
        
        self.confidence_score = (total_score / 6.0) * (1.0 - (self.errors.len() as f64 * 0.2).min(0.8));
        
        if !self.is_valid {
            self.confidence_score = self.confidence_score.min(0.5);
        }
    }
}

impl ValidationChecks {
    pub fn new() -> Self {
        Self {
            price_check: ValidationResult::pending(),
            amount_check: ValidationResult::pending(),
            platform_data_check: ValidationResult::pending(),
            timing_check: ValidationResult::pending(),
            fee_check: ValidationResult::pending(),
            duplicate_check: ValidationResult::pending(),
        }
    }
}

impl ValidationResult {
    pub fn pending() -> Self {
        Self {
            passed: false,
            score: 0.0,
            details: None,
            check_time_us: 0,
        }
    }
    
    pub fn passed(score: f64) -> Self {
        Self {
            passed: true,
            score: score.clamp(0.0, 1.0),
            details: None,
            check_time_us: 0,
        }
    }
    
    pub fn failed(reason: String) -> Self {
        Self {
            passed: false,
            score: 0.0,
            details: Some(reason),
            check_time_us: 0,
        }
    }
    
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
    
    pub fn with_timing(mut self, check_time_us: u64) -> Self {
        self.check_time_us = check_time_us;
        self
    }
}

impl RiskAssessment {
    pub fn new() -> Self {
        Self {
            risk_level: RiskLevel::Low,
            risk_score: 0.0,
            risk_factors: Vec::new(),
            recommendations: Vec::new(),
        }
    }
    
    pub fn add_risk_factor(&mut self, factor: RiskFactor) {
        self.risk_factors.push(factor);
        self.recalculate_risk();
    }
    
    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }
    
    fn recalculate_risk(&mut self) {
        if self.risk_factors.is_empty() {
            return;
        }
        
        // Calculate weighted average of risk scores
        let total_score: f64 = self.risk_factors.iter().map(|f| f.score).sum();
        self.risk_score = total_score / self.risk_factors.len() as f64;
        
        // Determine risk level based on score
        self.risk_level = match self.risk_score {
            0.0..0.25 => RiskLevel::Low,
            0.25..0.5 => RiskLevel::Medium,
            0.5..0.75 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };
    }
}

impl RiskFactor {
    pub fn new(risk_type: RiskType, score: f64, description: String, impact: RiskImpact) -> Self {
        Self {
            risk_type,
            score: score.clamp(0.0, 1.0),
            description,
            impact,
        }
    }
}
