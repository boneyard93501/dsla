use serde::{Deserialize, Serialize};
use bincode;

#[derive(Debug, Serialize, Deserialize)]
struct DefectReport {
    deal_id: String,
    start_timestamp: u64, 
    end_timestamp: u64,   
    report_id: u64, //nonce
    is_rejected: bool,                
}

impl DefectReport {
    fn new(deal_id: String, start_timestamp: u64, report_id: u64) -> Self {
        Self {
            deal_id,
            start_timestamp,
            end_timestamp:start_timestamp,
            report_id,
            is_rejected: false,              // Default to false
        }
    }

    fn update_timestamp(&mut self, end_timestamp: u64) {
        self.end_timestamp = end_timestamp;
    }

    fn reject(&mut self) {
        self.is_rejected = true;
    }

    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Failed to serialize")
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).expect("Failed to deserialize")
    }
}



mod tests {
    use super::*;

    #[test]
    fn test_report() {
        let deal_id = "deal-1".to_string();
        let nonce = 1;
        let ts_start = 1727025000;
        let ts_end = 1727025180;
        
        let mut report = DefectReport::new(deal_id,ts_start.clone(), nonce);
        assert_eq!(report.end_timestamp, ts_start);

        report.update_timestamp(ts_end);
        assert_eq!(report.end_timestamp, ts_end);

        assert_eq!(report.is_rejected, false);
        report.reject();
        assert_eq!(report.is_rejected, true);
    }

    #[test]
    fn test_serde() {
        let deal_id = "deal-1".to_string();
        let nonce = 1;
        let ts_start = 1727025000;
        let ts_end = 1727025180;
        
        let mut report = DefectReport::new(deal_id,ts_start.clone(), nonce);
        let bytes_report = report.to_bytes();

        let mut report = DefectReport::from_bytes(&bytes_report);

        report.update_timestamp(ts_end);
        assert_eq!(report.end_timestamp, ts_end);

        assert_eq!(report.is_rejected, false);
        report.reject();
        assert_eq!(report.is_rejected, true);
    }   
}