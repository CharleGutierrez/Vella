use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "LOW_RISK",
            Self::Medium => "MEDIUM_RISK",
            Self::High => "HIGH_RISK",
            Self::Critical => "CRITICAL_RISK",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Self::Low => "#22c55e",
            Self::Medium => "#f59e0b",
            Self::High => "#f97316",
            Self::Critical => "#ef4444",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_level: RiskLevel,
    pub confidence_score: f64, // 0.0 to 1.0
    pub recommendation: String,
    pub reasoning: Vec<String>,
}

/// AI Decision Engine: Evaluates risk for sensitive field updates,
/// detects anomalies, and scores pending approvals.
pub struct AiDecisionEngine;

impl AiDecisionEngine {
    /// Perform AI risk analysis on a pending field change
    pub fn assess_approval_risk(
        field_name: &str,
        old_value: Option<&str>,
        new_value: &str,
    ) -> RiskAssessment {
        let mut reasons = Vec::new();
        let mut risk = RiskLevel::Low;
        let mut confidence = 0.95;

        let field_lower = field_name.to_lowercase();

        // 1. Check for sensitive keywords
        let is_financial = field_lower.contains("salary")
            || field_lower.contains("price")
            || field_lower.contains("discount")
            || field_lower.contains("rate")
            || field_lower.contains("balance");

        let is_security = field_lower.contains("role")
            || field_lower.contains("permission")
            || field_lower.contains("admin")
            || field_lower.contains("secret")
            || field_lower.contains("password");

        // 2. Numerical deviation analysis
        if let (Some(old_str), Ok(new_num)) = (old_value, new_value.parse::<f64>()) {
            if let Ok(old_num) = old_str.parse::<f64>() {
                if old_num > 0.0 {
                    let ratio = new_num / old_num;
                    let pct_change = (ratio - 1.0) * 100.0;

                    if ratio >= 2.0 || ratio <= 0.3 {
                        risk = RiskLevel::High;
                        confidence = 0.98;
                        reasons.push(format!(
                            "Significant value deviation: {:.1}% change ({} -> {}).",
                            pct_change, old_num, new_num
                        ));
                    } else if ratio > 1.2 || ratio < 0.8 {
                        risk = RiskLevel::Medium;
                        reasons.push(format!(
                            "Moderate value shift: {:.1}% change.",
                            pct_change
                        ));
                    } else {
                        reasons.push("Value change is within normal operational tolerances (+/- 20%).".to_string());
                    }
                }
            }
        }

        // 3. Security role elevation check
        if is_security {
            let val_lower = new_value.to_lowercase();
            if val_lower == "admin" || val_lower == "superadmin" || val_lower == "superuser" {
                risk = RiskLevel::Critical;
                confidence = 0.99;
                reasons.push("Elevated privilege assignment: Change grants Superadmin access.".to_string());
            } else {
                if risk == RiskLevel::Low {
                    risk = RiskLevel::Medium;
                }
                reasons.push("Security-sensitive field modification.".to_string());
            }
        }

        // 4. Financial field adjustments
        if is_financial && risk == RiskLevel::Low {
            reasons.push("Standard financial adjustment within expected bounds.".to_string());
        }

        if reasons.is_empty() {
            reasons.push("Standard operational field change with no anomaly detected.".to_string());
        }

        let recommendation = match risk {
            RiskLevel::Low => "✅ Low Risk: Safe to approve. Parameters are within normal distribution.".to_string(),
            RiskLevel::Medium => "⚠️ Medium Risk: Review recommended. Minor anomaly or value shift detected.".to_string(),
            RiskLevel::High => "🚨 High Risk: Careful human review required. Substantial deviation detected.".to_string(),
            RiskLevel::Critical => "🛑 Critical Risk: Strict security verification required before sign-off.".to_string(),
        };

        RiskAssessment {
            risk_level: risk,
            confidence_score: confidence,
            recommendation,
            reasoning: reasons,
        }
    }
}
