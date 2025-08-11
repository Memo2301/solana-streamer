use serde::{Deserialize, Serialize};

/// Enhanced metadata with timing, network, and processing information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnrichedMetadata {
    /// Timing information
    pub timing: TimingData,
    
    /// Network and transaction data
    pub network: NetworkData,
    
    /// Processing metadata
    pub processing: ProcessingData,
}

/// Comprehensive timing information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimingData {
    /// Slot number
    pub slot: u64,
    
    /// Block time in milliseconds
    pub block_time_ms: u64,
    
    /// When the program received the event (ms)
    pub program_received_time_ms: u64,
    
    /// Processing delay (program_received - block_time)
    pub processing_delay_ms: u64,
    
    /// When we processed this event to enriched form
    pub enrichment_time_ms: u64,
    
    /// Total time from block to enrichment
    pub total_processing_time_ms: u64,
    
    /// Estimated confirmation time (if available)
    pub estimated_confirmation_time_ms: Option<u64>,
}

/// Network and transaction metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkData {
    /// Transaction signature
    pub signature: String,
    
    /// Program ID that generated this event
    pub program_id: String,
    
    /// Relevant account keys
    pub account_keys: Vec<String>,
    
    /// Instruction index within the transaction
    pub instruction_index: Option<u32>,
    
    /// Inner instruction index (if applicable)
    pub inner_instruction_index: Option<u32>,
}

/// Processing and enrichment metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessingData {
    /// SDK version that processed this event
    pub sdk_version: String,
    
    /// Parser version
    pub parser_version: String,
    
    /// Processing timestamp
    pub processed_at: u64,
    
    /// Any warnings during processing
    pub warnings: Vec<String>,
    
    /// Processing performance metrics
    pub performance: ProcessingPerformance,
}

/// Performance metrics for processing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessingPerformance {
    /// Time to parse raw event (microseconds)
    pub parse_time_us: u64,
    
    /// Time to enrich event (microseconds)
    pub enrichment_time_us: u64,
    
    /// Time to validate event (microseconds)
    pub validation_time_us: u64,
    
    /// Total processing time (microseconds)
    pub total_time_us: u64,
}

impl TimingData {
    pub fn new(slot: u64, block_time_ms: u64, program_received_time_ms: u64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
            
        let processing_delay_ms = program_received_time_ms.saturating_sub(block_time_ms);
        let total_processing_time_ms = now.saturating_sub(block_time_ms);
        
        Self {
            slot,
            block_time_ms,
            program_received_time_ms,
            processing_delay_ms,
            enrichment_time_ms: now,
            total_processing_time_ms,
            estimated_confirmation_time_ms: None,
        }
    }
}

impl NetworkData {
    pub fn new(signature: String, program_id: String) -> Self {
        Self {
            signature,
            program_id,
            account_keys: Vec::new(),
            instruction_index: None,
            inner_instruction_index: None,
        }
    }
    
    pub fn with_account_keys(mut self, account_keys: Vec<String>) -> Self {
        self.account_keys = account_keys;
        self
    }
    
    pub fn with_instruction_index(mut self, index: u32) -> Self {
        self.instruction_index = Some(index);
        self
    }
}

impl ProcessingData {
    pub fn new() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
            
        Self {
            sdk_version: env!("CARGO_PKG_VERSION").to_string(),
            parser_version: "1.0.0".to_string(),
            processed_at: now,
            warnings: Vec::new(),
            performance: ProcessingPerformance::default(),
        }
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    pub fn set_performance(&mut self, performance: ProcessingPerformance) {
        self.performance = performance;
    }
}

impl Default for ProcessingPerformance {
    fn default() -> Self {
        Self {
            parse_time_us: 0,
            enrichment_time_us: 0,
            validation_time_us: 0,
            total_time_us: 0,
        }
    }
}

impl ProcessingPerformance {
    pub fn start_timer() -> std::time::Instant {
        std::time::Instant::now()
    }
    
    pub fn record_parse_time(&mut self, start: std::time::Instant) {
        self.parse_time_us = start.elapsed().as_micros() as u64;
    }
    
    pub fn record_enrichment_time(&mut self, start: std::time::Instant) {
        self.enrichment_time_us = start.elapsed().as_micros() as u64;
    }
    
    pub fn record_validation_time(&mut self, start: std::time::Instant) {
        self.validation_time_us = start.elapsed().as_micros() as u64;
    }
    
    pub fn calculate_total(&mut self) {
        self.total_time_us = self.parse_time_us + self.enrichment_time_us + self.validation_time_us;
    }
}
