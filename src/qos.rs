struct QoSCommitment {
    uptime: String, // nines
    downtime_seconds: u64,
    penalty_coefficient: f64,
}

struct SLAHandler {
    penalty_ranges: Vec<QoSCommitment>,
}

impl SLAHandler {
    fn new(qos_ranges: Vec<QoSCommitment>) -> Self {
        Self {
            penalty_ranges: qos_ranges,
        }
    }

    fn calculate_penalty(&self, reported_downtime: u64, deal_value: f64) -> f64 {
        self.penalty_ranges
            .iter()
            .find(|range| reported_downtime > range.downtime_seconds) // Find the first applicable range
            .map(|range| range.penalty_coefficient.min(1.0) * deal_value) // Calculate penalty, capping coefficient at 1.0
            .unwrap_or(0.0) // No penalty if uptime is above all ranges
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_calculate_penalty() {
        let qos_ranges = vec![
            QoSCommitment {
                uptime: "95".to_string(),
                downtime_seconds: 80064, // 22.24 hours
                penalty_coefficient: 1.0,
            },
            QoSCommitment {
                uptime: "98.5".to_string(),
                downtime_seconds: 39132, // 10.87 hours
                penalty_coefficient: 0.3,
            },
            QoSCommitment {
                uptime: "99".to_string(),
                downtime_seconds: 26064, // 7.24 hours
                penalty_coefficient: 0.1,
            }
        ];

        let sla_handler = SLAHandler::new(qos_ranges);
        assert_eq!(sla_handler.penalty_ranges[1].uptime, "98.5".to_string());

        let deal_value = 10.0;

        let reported_downtime = 26064;
        let penalty = sla_handler.calculate_penalty(reported_downtime, deal_value.clone());
        assert_eq!(penalty, 0.0);

        let reported_downtime = 39132;
        let penalty = sla_handler.calculate_penalty(reported_downtime, deal_value.clone());
        assert_eq!(penalty, 1.0);

        let reported_downtime = 39133;
        let penalty = sla_handler.calculate_penalty(reported_downtime, deal_value.clone());
        assert_eq!(penalty, 3.0);

        let reported_downtime = 80064;
        let penalty: f64 = sla_handler.calculate_penalty(reported_downtime, deal_value.clone());
        assert_eq!(penalty, 3.0);

        let reported_downtime = 80065;
        let penalty: f64 = sla_handler.calculate_penalty(reported_downtime, deal_value.clone());
        assert_eq!(penalty, 10.0);
    }
}
