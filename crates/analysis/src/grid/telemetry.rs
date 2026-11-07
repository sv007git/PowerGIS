//! SCADA smart meter telemetry stream ingestion and anomaly detection (Roadmap Feature).
//!
//! Evaluates incoming voltage, current, and active power telemetry against utility
//! ANSI/IEC tolerances and flags real-time anomaly alerts.

use super::model::{MeterStatus, MeterTelemetry, TelemetryAnomaly};
use super::topology::ElectricalNetwork;

/// Evaluates a batch of telemetry readings, updates network telemetry cache, and returns
/// any detected grid anomalies.
pub fn ingest_telemetry_batch(
    network: &mut ElectricalNetwork,
    readings: Vec<MeterTelemetry>,
) -> Vec<TelemetryAnomaly> {
    let mut anomalies = Vec::new();

    for reading in readings {
        let meter_num = reading.meter_number.clone();
        let cust_num = network
            .meters
            .get(&meter_num)
            .map(|m| m.customer_number.clone())
            .unwrap_or_else(|| "UNKNOWN".to_string());

        // Standard 230V single-phase / 400V three-phase nominal
        let nominal_v = 230.0f64;

        // 1. Under-Voltage (< 207V, i.e. < 0.90 p.u.)
        if reading.voltage_v > 10.0 && reading.voltage_v < (nominal_v * 0.90) {
            anomalies.push(TelemetryAnomaly {
                meter_number: meter_num.clone(),
                customer_number: cust_num.clone(),
                anomaly_type: "UNDER_VOLTAGE".to_string(),
                observed_value: reading.voltage_v,
                threshold: nominal_v * 0.90,
                unit: "V".to_string(),
                message: format!("Severe under-voltage detected: {:.1}V (limit: 207V). Risk of brownout.", reading.voltage_v),
                timestamp_ms: reading.timestamp_ms,
            });
            if let Some(m) = network.meters.get_mut(&meter_num) {
                if m.status == MeterStatus::Active {
                    m.status = MeterStatus::VoltageAnomaly;
                }
            }
        }

        // 2. Over-Voltage (> 253V, i.e. > 1.10 p.u.)
        if reading.voltage_v > (nominal_v * 1.10) {
            anomalies.push(TelemetryAnomaly {
                meter_number: meter_num.clone(),
                customer_number: cust_num.clone(),
                anomaly_type: "OVER_VOLTAGE".to_string(),
                observed_value: reading.voltage_v,
                threshold: nominal_v * 1.10,
                unit: "V".to_string(),
                message: format!("Over-voltage surge detected: {:.1}V (limit: 253V). Risk of equipment damage.", reading.voltage_v),
                timestamp_ms: reading.timestamp_ms,
            });
            if let Some(m) = network.meters.get_mut(&meter_num) {
                if m.status == MeterStatus::Active {
                    m.status = MeterStatus::VoltageAnomaly;
                }
            }
        }

        // 3. Complete Outage / Zero Voltage (V < 10V)
        if reading.voltage_v < 10.0 {
            anomalies.push(TelemetryAnomaly {
                meter_number: meter_num.clone(),
                customer_number: cust_num.clone(),
                anomaly_type: "OUTAGE_DETECTED".to_string(),
                observed_value: reading.voltage_v,
                threshold: 10.0,
                unit: "V".to_string(),
                message: format!("Loss of voltage detected on meter {meter_num}. Feeder or service drop outage."),
                timestamp_ms: reading.timestamp_ms,
            });
            if let Some(m) = network.meters.get_mut(&meter_num) {
                m.status = MeterStatus::Outage;
            }
        }

        // 4. Overcurrent (> 65A on standard single-phase residential connection)
        if reading.current_a > 65.0 {
            anomalies.push(TelemetryAnomaly {
                meter_number: meter_num.clone(),
                customer_number: cust_num.clone(),
                anomaly_type: "OVER_CURRENT".to_string(),
                observed_value: reading.current_a,
                threshold: 65.0,
                unit: "A".to_string(),
                message: format!("Dangerous over-current detected: {:.1}A. Immediate fire / overload risk.", reading.current_a),
                timestamp_ms: reading.timestamp_ms,
            });
        }

        // 5. Poor Power Factor (< 0.80)
        if reading.power_factor > 0.0 && reading.power_factor < 0.80 {
            anomalies.push(TelemetryAnomaly {
                meter_number: meter_num.clone(),
                customer_number: cust_num.clone(),
                anomaly_type: "LOW_POWER_FACTOR".to_string(),
                observed_value: reading.power_factor,
                threshold: 0.80,
                unit: "PF".to_string(),
                message: format!("Low power factor ({:.2}) observed. Excessive reactive power demand.", reading.power_factor),
                timestamp_ms: reading.timestamp_ms,
            });
        }

        network.telemetry.insert(meter_num, reading);
    }

    anomalies
}
